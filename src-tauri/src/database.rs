use rusqlite::{Connection, Result as SqliteResult, params};
use chrono::{Utc, TimeZone};
use std::path::PathBuf;
use dirs;
use crate::models::{Task, TaskPriority, CreateTaskRequest, UpdateTaskRequest, Tag, CreateTagRequest, UpdateTagRequest, AppSettings, CloseBehavior, StartupBehavior, UpdateSettingsRequest};

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        // 确保目录存在
        if let Some(parent_dir) = db_path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        
        let connection = Connection::open(&db_path)?;
        let db = Database { connection };
        
        // 初始化数据库表
        db.init_tables()?;
        
        Ok(db)
    }
    
    /// 获取数据库文件路径：%UserProfile%\Documents\TodoAppData\tasks.db
    fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let documents_dir = dirs::document_dir()
            .ok_or("无法获取用户文档目录")?;
        
        let app_data_dir = documents_dir.join("TodoAppData");
        let db_path = app_data_dir.join("tasks.db");
        
        Ok(db_path)
    }
    
    /// 初始化数据库表结构
    fn init_tables(&self) -> SqliteResult<()> {
        // 创建任务表
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                description TEXT DEFAULT '',
                due_date DATETIME,
                priority INTEGER NOT NULL DEFAULT 1 CHECK(priority IN (0,1,2)),
                is_completed BOOLEAN NOT NULL DEFAULT 0,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // 创建标签表
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                color TEXT NOT NULL DEFAULT '#3b82f6',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        // 创建任务-标签关联表（多对多关系）
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS task_tags (
                task_id INTEGER NOT NULL,
                tag_name TEXT NOT NULL,
                PRIMARY KEY (task_id, tag_name),
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_name) REFERENCES tags(name) ON DELETE CASCADE
            )",
            [],
        )?;
        
        // 创建索引以提高查询性能
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_priority ON tasks(priority)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_due_date ON tasks(due_date)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_tasks_is_completed ON tasks(is_completed)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_task_tags_task_id ON task_tags(task_id)",
            [],
        )?;
        
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS idx_task_tags_tag_name ON task_tags(tag_name)",
            [],
        )?;
        
        // 创建设置表
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                close_behavior TEXT NOT NULL DEFAULT 'ask' CHECK(close_behavior IN ('exit','minimize','ask')),
                notifications_enabled BOOLEAN NOT NULL DEFAULT 1,
                notification_time_before INTEGER NOT NULL DEFAULT 15,
                startup_behavior TEXT NOT NULL DEFAULT 'normal' CHECK(startup_behavior IN ('normal','minimized')),
                theme TEXT NOT NULL DEFAULT 'auto'
            )",
            [],
        )?;
        
        // 插入默认设置（如果不存在）
        self.connection.execute(
            "INSERT OR IGNORE INTO settings (id, close_behavior, notifications_enabled, notification_time_before, startup_behavior, theme) 
             VALUES (1, 'ask', 1, 15, 'normal', 'auto')",
            [],
        )?;
        
        // 数据库迁移：添加sort_order字段
        self.migrate_add_sort_order()?;
        
        Ok(())
    }
    
    /// 迁移：添加sort_order字段到tasks表
    fn migrate_add_sort_order(&self) -> SqliteResult<()> {
        // 检查sort_order字段是否已存在
        let mut stmt = self.connection.prepare("PRAGMA table_info(tasks)").unwrap();
        let column_iter = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(1)?) // column name
        })?;
        
        let mut has_sort_order = false;
        for column_result in column_iter {
            if let Ok(column_name) = column_result {
                if column_name == "sort_order" {
                    has_sort_order = true;
                    break;
                }
            }
        }
        
        // 如果没有sort_order字段，则添加它
        if !has_sort_order {
            self.connection.execute(
                "ALTER TABLE tasks ADD COLUMN sort_order INTEGER DEFAULT 0",
                []
            )?;
            
            // 为现有任务设置sort_order值（基于created_at排序）
            self.connection.execute(
                "UPDATE tasks SET sort_order = (
                    SELECT COUNT(*) FROM tasks t2 WHERE t2.created_at <= tasks.created_at
                )",
                []
            )?;
        }
        
        Ok(())
    }
    
    /// 创建新任务
    pub fn create_task(&self, request: CreateTaskRequest) -> Result<Task, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let due_date_timestamp = request.due_date.map(|dt| dt.timestamp());
        let priority_value: i32 = request.priority.into();
        let description = request.description.clone().unwrap_or_default();
        
        // 获取下一个sort_order值
        let next_sort_order: i64 = self.connection
            .query_row("SELECT COALESCE(MAX(sort_order), 0) + 1 FROM tasks", [], |row| {
                Ok(row.get(0)?)
            })?;
        
        self.connection.execute(
            "INSERT INTO tasks (title, description, due_date, priority, created_at, sort_order) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                request.title,
                description,
                due_date_timestamp,
                priority_value,
                now.timestamp(),
                next_sort_order
            ],
        )?;
        
        let task_id = self.connection.last_insert_rowid();
        
        // 处理标签
        let tags = request.tags.unwrap_or_default();
        if !tags.is_empty() {
            self.set_task_tags(task_id, &tags)?;
        }
        
        Ok(Task {
            id: Some(task_id),
            title: request.title,
            description,
            due_date: request.due_date,
            priority: request.priority,
            is_completed: false,
            created_at: now,
            tags,
            sort_order: next_sort_order,
        })
    }
    
    /// 获取所有任务
    pub fn get_all_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks 
             ORDER BY sort_order ASC, created_at DESC"
        )?;
        
        let task_iter = stmt.query_map([], |row| {
            let due_date_timestamp: Option<i64> = row.get(3)?;
            let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
            
            let created_at_timestamp: i64 = row.get(6)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: TaskPriority::from(row.get::<_, i32>(4)?),
                is_completed: row.get(5)?,
                created_at,
                tags: Vec::new(), // 先创建空标签列表
                sort_order: row.get(7)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task_result in task_iter {
            let mut task = task_result?;
            // 获取任务的标签
            if let Some(task_id) = task.id {
                task.tags = self.get_task_tags(task_id).unwrap_or_default();
            }
            tasks.push(task);
        }
        
        Ok(tasks)
    }
    
    /// 更新任务排序
    pub fn update_tasks_sort_order(&self, task_orders: Vec<(i64, i64)>) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "UPDATE tasks SET sort_order = ?2 WHERE id = ?1"
        )?;
        
        for (task_id, sort_order) in task_orders {
            stmt.execute(params![task_id, sort_order])?;
        }
        
        Ok(())
    }
    
    /// 获取今日任务
    pub fn get_today_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let today_start = Utc::now().date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
        let today_end = Utc::now().date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();
        
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks 
             WHERE due_date BETWEEN ?1 AND ?2
             ORDER BY priority DESC, due_date ASC"
        )?;
        
        let task_iter = stmt.query_map(
            params![today_start.timestamp(), today_end.timestamp()],
            |row| {
                let due_date_timestamp: Option<i64> = row.get(3)?;
                let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
                
                let created_at_timestamp: i64 = row.get(6)?;
                let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
                
                Ok(Task {
                    id: Some(row.get(0)?),
                    title: row.get(1)?,
                    description: row.get(2)?,
                    due_date,
                    priority: TaskPriority::from(row.get::<_, i32>(4)?),
                    is_completed: row.get(5)?,
                    created_at,
                    tags: Vec::new(), // 稍后会填充标签
                    sort_order: row.get(7)?,
                })
            }
        )?;
        
        let mut tasks = Vec::new();
        for task_result in task_iter {
            let mut task = task_result?;
            // 获取任务的标签
            if let Some(task_id) = task.id {
                task.tags = self.get_task_tags(task_id).unwrap_or_default();
            }
            tasks.push(task);
        }
        
        Ok(tasks)
    }
    
    /// 获取已完成任务
    pub fn get_completed_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks 
             WHERE is_completed = 1
             ORDER BY created_at DESC"
        )?;
        
        let task_iter = stmt.query_map([], |row| {
            let due_date_timestamp: Option<i64> = row.get(3)?;
            let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
            
            let created_at_timestamp: i64 = row.get(6)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: TaskPriority::from(row.get::<_, i32>(4)?),
                is_completed: row.get(5)?,
                created_at,
                tags: Vec::new(), // 稍后会填充标签
                sort_order: row.get(7)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task_result in task_iter {
            let mut task = task_result?;
            // 获取任务的标签
            if let Some(task_id) = task.id {
                task.tags = self.get_task_tags(task_id).unwrap_or_default();
            }
            tasks.push(task);
        }
        
        Ok(tasks)
    }
    
    /// 更新任务
    pub fn update_task(&self, request: UpdateTaskRequest) -> Result<Task, Box<dyn std::error::Error>> {
        // 首先获取现有任务
        let existing_task = self.get_task_by_id(request.id)?;
        
        let title = request.title.unwrap_or(existing_task.title);
        let description = request.description.unwrap_or(existing_task.description);
        let due_date = request.due_date.or(existing_task.due_date);
        let priority = request.priority.unwrap_or(existing_task.priority);
        let is_completed = request.is_completed.unwrap_or(existing_task.is_completed);
        
        let due_date_timestamp = due_date.map(|dt| dt.timestamp());
        let priority_copy = priority; // 创建副本
        let priority_value: i32 = priority_copy.into();
        
        self.connection.execute(
            "UPDATE tasks SET title = ?1, description = ?2, due_date = ?3, priority = ?4, is_completed = ?5 
             WHERE id = ?6",
            params![
                title,
                description,
                due_date_timestamp,
                priority_value,
                is_completed,
                request.id
            ],
        )?;
        
        // 处理标签
        let tags = if let Some(ref tags) = request.tags {
            if !tags.is_empty() {
                self.set_task_tags(request.id, tags)?;
            }
            tags.clone()
        } else {
            // 如果没有提供标签信息，保持现有标签
            self.get_task_tags(request.id).unwrap_or_default()
        };
        
        Ok(Task {
            id: Some(request.id),
            title,
            description,
            due_date,
            priority,
            is_completed,
            created_at: existing_task.created_at,
            tags,
            sort_order: existing_task.sort_order,
        })
    }
    
    /// 根据ID获取任务
    pub fn get_task_by_id(&self, id: i64) -> Result<Task, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks WHERE id = ?1"
        )?;
        
        let task = stmt.query_row([id], |row| {
            let due_date_timestamp: Option<i64> = row.get(3)?;
            let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
            
            let created_at_timestamp: i64 = row.get(6)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: TaskPriority::from(row.get::<_, i32>(4)?),
                is_completed: row.get(5)?,
                created_at,
                tags: Vec::new(), // 稍后会填充标签
                sort_order: row.get(7)?,
            })
        })?;
        
        // 获取任务的标签
        let mut task_with_tags = task;
        task_with_tags.tags = self.get_task_tags(id).unwrap_or_default();
        
        Ok(task_with_tags)
    }
    
    /// 删除任务
    pub fn delete_task(&self, id: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute("DELETE FROM tasks WHERE id = ?1", [id])?;
        Ok(())
    }
    
    /// 切换任务完成状态
    pub fn toggle_task_completion(&self, id: i64) -> Result<Task, Box<dyn std::error::Error>> {
        let existing_task = self.get_task_by_id(id)?;
        let new_completion_status = !existing_task.is_completed;
        
        self.connection.execute(
            "UPDATE tasks SET is_completed = ?1 WHERE id = ?2",
            params![new_completion_status, id],
        )?;
        
        let mut updated_task = existing_task;
        updated_task.is_completed = new_completion_status;
        
        // 确保包含最新的标签信息
        if let Some(task_id) = updated_task.id {
            updated_task.tags = self.get_task_tags(task_id).unwrap_or_default();
        }
        
        Ok(updated_task)
    }
    
    /// 导出所有任务数据为 JSON 格式
    pub fn export_tasks_to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        let tasks = self.get_all_tasks()?;
        let json_data = serde_json::to_string_pretty(&tasks)?;
        Ok(json_data)
    }
    
    /// 从 JSON 数据导入任务
    pub fn import_tasks_from_json(&self, json_data: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let tasks: Vec<Task> = serde_json::from_str(json_data)?;
        let mut imported_count = 0;
        
        for task in tasks {
            // 创建导入任务请求
            let import_request = CreateTaskRequest {
                title: task.title,
                description: Some(task.description),
                due_date: task.due_date,
                priority: task.priority,
                tags: Some(task.tags),
            };
            
            // 导入任务（忽略错误，继续导入其他任务）
            if self.create_task(import_request).is_ok() {
                imported_count += 1;
            }
        }
        
        Ok(imported_count)
    }
    
    /// 清空所有任务数据
    pub fn clear_all_tasks(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute("DELETE FROM tasks", [])?;
        self.connection.execute("DELETE FROM task_tags", [])?;
        Ok(())
    }
    
    // === 标签相关方法 ===
    
    /// 获取任务的标签列表
    fn get_task_tags(&self, task_id: i64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT tag_name FROM task_tags WHERE task_id = ?1"
        )?;
        
        let tag_iter = stmt.query_map([task_id], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;
        
        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag?);
        }
        
        Ok(tags)
    }
    
    /// 设置任务的标签
    fn set_task_tags(&self, task_id: i64, tags: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        // 删除现有标签关联
        self.connection.execute(
            "DELETE FROM task_tags WHERE task_id = ?1",
            [task_id]
        )?;
        
        // 添加新的标签关联
        for tag_name in tags {
            // 确保标签存在（如果不存在则创建）
            self.connection.execute(
                "INSERT OR IGNORE INTO tags (name, color) VALUES (?1, ?2)",
                params![tag_name, "#3b82f6"] // 默认蓝色
            )?;
            
            // 创建任务-标签关联
            self.connection.execute(
                "INSERT INTO task_tags (task_id, tag_name) VALUES (?1, ?2)",
                params![task_id, tag_name]
            )?;
        }
        
        Ok(())
    }
    
    /// 创建标签
    pub fn create_tag(&self, request: CreateTagRequest) -> Result<Tag, Box<dyn std::error::Error>> {
        let now = Utc::now();
        
        self.connection.execute(
            "INSERT INTO tags (name, color, created_at) VALUES (?1, ?2, ?3)",
            params![request.name, request.color, now.timestamp()]
        )?;
        
        let tag_id = self.connection.last_insert_rowid();
        
        Ok(Tag {
            id: Some(tag_id),
            name: request.name,
            color: request.color,
            created_at: now,
        })
    }
    
    /// 获取所有标签
    pub fn get_all_tags(&self) -> Result<Vec<Tag>, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT id, name, color, created_at FROM tags ORDER BY name"
        )?;
        
        let tag_iter = stmt.query_map([], |row| {
            let created_at_timestamp: i64 = row.get(3)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                color: row.get(2)?,
                created_at,
            })
        })?;
        
        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag?);
        }
        
        Ok(tags)
    }
    
    /// 更新标签
    pub fn update_tag(&self, request: UpdateTagRequest) -> Result<Tag, Box<dyn std::error::Error>> {
        // 构建动态更新语句
        let mut updates = Vec::new();
        let mut params = Vec::new();
        
        if let Some(ref name) = request.name {
            updates.push("name = ?");
            params.push(name.as_str());
        }
        
        if let Some(ref color) = request.color {
            updates.push("color = ?");
            params.push(color.as_str());
        }
        
        if updates.is_empty() {
            return Err("没有提供要更新的字段".into());
        }
        
        let sql = format!("UPDATE tags SET {} WHERE id = ?", updates.join(", "));
        let id_str = request.id.to_string();
        params.push(&id_str);
        
        self.connection.execute(&sql, rusqlite::params_from_iter(params))?;
        
        // 获取更新后的标签
        let mut stmt = self.connection.prepare(
            "SELECT id, name, color, created_at FROM tags WHERE id = ?1"
        )?;
        
        let tag = stmt.query_row([request.id], |row| {
            let created_at_timestamp: i64 = row.get(3)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            Ok(Tag {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                color: row.get(2)?,
                created_at,
            })
        })?;
        
        Ok(tag)
    }
    
    /// 删除标签
    pub fn delete_tag(&self, tag_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        self.connection.execute("DELETE FROM tags WHERE id = ?1", [tag_id])?;
        Ok(())
    }
    
    // === 设置相关方法 ===
    
    /// 获取应用设置
    pub fn get_app_settings(&self) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let mut stmt = self.connection.prepare(
            "SELECT close_behavior, notifications_enabled, notification_time_before, startup_behavior, theme 
             FROM settings WHERE id = 1"
        )?;
        
        let settings = stmt.query_row([], |row| {
            let close_behavior_str: String = row.get(0)?;
            let close_behavior = match close_behavior_str.as_str() {
                "exit" => CloseBehavior::Exit,
                "minimize" => CloseBehavior::Minimize,
                _ => CloseBehavior::Ask,
            };
            
            let startup_behavior_str: String = row.get(3)?;
            let startup_behavior = match startup_behavior_str.as_str() {
                "minimized" => StartupBehavior::Minimized,
                _ => StartupBehavior::Normal,
            };
            
            Ok(AppSettings {
                close_behavior,
                notifications_enabled: row.get(1)?,
                notification_time_before: row.get(2)?,
                startup_behavior,
                theme: row.get(4)?,
            })
        });
        
        match settings {
            Ok(settings) => Ok(settings),
            Err(_) => {
                // 如果获取失败，返回默认设置
                Ok(AppSettings::default())
            }
        }
    }
    
    /// 更新应用设置
    pub fn update_app_settings(&self, request: UpdateSettingsRequest) -> Result<AppSettings, Box<dyn std::error::Error>> {
        // 先获取当前设置
        let current = self.get_app_settings()?;
        
        // 构建更新的设置
        let close_behavior = request.close_behavior.unwrap_or(current.close_behavior);
        let notifications_enabled = request.notifications_enabled.unwrap_or(current.notifications_enabled);
        let notification_time_before = request.notification_time_before.unwrap_or(current.notification_time_before);
        let startup_behavior = request.startup_behavior.unwrap_or(current.startup_behavior);
        let theme = request.theme.unwrap_or(current.theme);
        
        // 转换枚举为字符串
        let close_behavior_str = match close_behavior {
            CloseBehavior::Exit => "exit",
            CloseBehavior::Minimize => "minimize",
            CloseBehavior::Ask => "ask",
        };
        
        let startup_behavior_str = match startup_behavior {
            StartupBehavior::Normal => "normal",
            StartupBehavior::Minimized => "minimized",
        };
        
        // 更新数据库
        self.connection.execute(
            "UPDATE settings SET close_behavior = ?1, notifications_enabled = ?2, notification_time_before = ?3, startup_behavior = ?4, theme = ?5 WHERE id = 1",
            params![
                close_behavior_str,
                notifications_enabled,
                notification_time_before,
                startup_behavior_str,
                theme
            ],
        )?;
        
        Ok(AppSettings {
            close_behavior,
            notifications_enabled,
            notification_time_before,
            startup_behavior,
            theme,
        })
    }
    
    /// 获取过期的任务
    pub fn get_overdue_tasks(&self) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let now = Utc::now().to_rfc3339();
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks 
             WHERE is_completed = 0 AND due_date IS NOT NULL AND due_date < ?1
             ORDER BY due_date ASC"
        )?;
        
        let task_iter = stmt.query_map([now], |row| {
            let due_date_timestamp: Option<i64> = row.get(3)?;
            let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
            
            let created_at_timestamp: i64 = row.get(6)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            // 获取任务标签
            let task_id: i64 = row.get(0)?;
            let mut tag_stmt = self.connection.prepare(
                "SELECT tag_name FROM task_tags WHERE task_id = ?1"
            ).unwrap();
            
            let tag_iter = tag_stmt.query_map([task_id], |row| {
                Ok(row.get::<_, String>(0)?)
            }).unwrap();
            
            let mut tags = Vec::new();
            for tag in tag_iter {
                tags.push(tag.unwrap());
            }
            
            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: TaskPriority::from(row.get::<_, i32>(4)?),
                is_completed: row.get(5)?,
                created_at,
                tags,
                sort_order: row.get(7)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        
        Ok(tasks)
    }
    
    /// 获取即将到期的任务（在指定分钟数内）
    pub fn get_upcoming_tasks(&self, minutes_ahead: i32) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let future = now + chrono::Duration::minutes(minutes_ahead as i64);
        
        let now_str = now.to_rfc3339();
        let future_str = future.to_rfc3339();
        
        let mut stmt = self.connection.prepare(
            "SELECT id, title, description, due_date, priority, is_completed, created_at, COALESCE(sort_order, 0) as sort_order
             FROM tasks 
             WHERE is_completed = 0 AND due_date IS NOT NULL 
             AND due_date > ?1 AND due_date <= ?2
             ORDER BY due_date ASC"
        )?;
        
        let task_iter = stmt.query_map([now_str, future_str], |row| {
            let due_date_timestamp: Option<i64> = row.get(3)?;
            let due_date = due_date_timestamp.map(|ts| Utc.timestamp_opt(ts, 0).unwrap());
            
            let created_at_timestamp: i64 = row.get(6)?;
            let created_at = Utc.timestamp_opt(created_at_timestamp, 0).unwrap();
            
            // 获取任务标签
            let task_id: i64 = row.get(0)?;
            let mut tag_stmt = self.connection.prepare(
                "SELECT tag_name FROM task_tags WHERE task_id = ?1"
            ).unwrap();
            
            let tag_iter = tag_stmt.query_map([task_id], |row| {
                Ok(row.get::<_, String>(0)?)
            }).unwrap();
            
            let mut tags = Vec::new();
            for tag in tag_iter {
                tags.push(tag.unwrap());
            }
            
            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                due_date,
                priority: TaskPriority::from(row.get::<_, i32>(4)?),
                is_completed: row.get(5)?,
                created_at,
                tags,
                sort_order: row.get(7)?,
            })
        })?;
        
        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        
        Ok(tasks)
    }
}