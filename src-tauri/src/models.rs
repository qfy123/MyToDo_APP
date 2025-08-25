use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub is_completed: bool,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>, // 任务标签列表
    pub sort_order: i64,   // 新增：排序顺序
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub color: String, // 标签颜色（十六进制）
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum TaskPriority {
    Low = 0,
    Medium = 1,
    High = 2,
}

// 自定义序列化为数字
impl Serialize for TaskPriority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

// 自定义反序列化以支持从整数值反序列化
impl<'de> serde::Deserialize<'de> for TaskPriority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(TaskPriority::from(value))
    }
}

impl From<i32> for TaskPriority {
    fn from(value: i32) -> Self {
        match value {
            0 => TaskPriority::Low,
            1 => TaskPriority::Medium,
            2 => TaskPriority::High,
            _ => TaskPriority::Medium, // 默认为中等优先级
        }
    }
}

impl Into<i32> for TaskPriority {
    fn into(self) -> i32 {
        self as i32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: TaskPriority,
    pub tags: Option<Vec<String>>, // 新增：创建任务时的标签
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<TaskPriority>,
    pub is_completed: Option<bool>,
    pub tags: Option<Vec<String>>, // 更新任务时的标签
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskSortOrderRequest {
    pub task_orders: Vec<TaskOrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskOrderItem {
    pub id: i64,
    pub sort_order: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub id: i64,
    pub name: Option<String>,
    pub color: Option<String>,
}

// 用户设置相关模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub close_behavior: CloseBehavior,
    pub notifications_enabled: bool,
    pub notification_time_before: i32, // 提前多少分钟通知（截止时间提醒）
    pub startup_behavior: StartupBehavior,
    pub theme: String, // 主题设置 ("light", "dark", "auto")
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CloseBehavior {
    #[serde(rename = "exit")]
    Exit,      // 直接退出
    #[serde(rename = "minimize")]
    Minimize,  // 最小化到托盘
    #[serde(rename = "ask")]
    Ask,       // 每次询问
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StartupBehavior {
    #[serde(rename = "normal")]
    Normal,    // 正常启动
    #[serde(rename = "minimized")]
    Minimized, // 启动时最小化到托盘
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            close_behavior: CloseBehavior::Ask,
            notifications_enabled: true,
            notification_time_before: 15, // 默认提前15分钟通知
            startup_behavior: StartupBehavior::Normal,
            theme: "auto".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSettingsRequest {
    pub close_behavior: Option<CloseBehavior>,
    pub notifications_enabled: Option<bool>,
    pub notification_time_before: Option<i32>,
    pub startup_behavior: Option<StartupBehavior>,
    pub theme: Option<String>,
}