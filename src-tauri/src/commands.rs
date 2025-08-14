use tauri::State;
use tauri::Manager;
use crate::{AppState, models::{Task, CreateTaskRequest, UpdateTaskRequest, Tag, CreateTagRequest, UpdateTagRequest, AppSettings, UpdateSettingsRequest, UpdateTaskSortOrderRequest}};

#[tauri::command]
pub fn create_task(
    state: State<AppState>,
    request: CreateTaskRequest,
) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.create_task(request).map_err(|e| format!("创建任务失败: {}", e))
}

#[tauri::command]
pub fn get_all_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_all_tasks().map_err(|e| format!("获取任务列表失败: {}", e))
}

#[tauri::command]
pub fn get_today_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_today_tasks().map_err(|e| format!("获取今日任务失败: {}", e))
}

#[tauri::command]
pub fn get_completed_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_completed_tasks().map_err(|e| format!("获取已完成任务失败: {}", e))
}

#[tauri::command]
pub fn update_task(
    state: State<AppState>,
    request: UpdateTaskRequest,
) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.update_task(request).map_err(|e| format!("更新任务失败: {}", e))
}

#[tauri::command]
pub fn delete_task(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.delete_task(id).map_err(|e| format!("删除任务失败: {}", e))
}

#[tauri::command]
pub fn toggle_task_completion(state: State<AppState>, id: i64) -> Result<Task, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.toggle_task_completion(id).map_err(|e| format!("切换任务状态失败: {}", e))
}

#[tauri::command]
pub fn export_tasks_to_json(state: State<AppState>) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.export_tasks_to_json().map_err(|e| format!("导出任务失败: {}", e))
}

#[tauri::command]
pub fn import_tasks_from_json(state: State<AppState>, json_data: String) -> Result<usize, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.import_tasks_from_json(&json_data).map_err(|e| format!("导入任务失败: {}", e))
}

#[tauri::command]
pub fn clear_all_tasks(state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.clear_all_tasks().map_err(|e| format!("清空任务失败: {}", e))
}

// === 标签相关命令 ===

#[tauri::command]
pub fn create_tag(
    state: State<AppState>,
    request: CreateTagRequest,
) -> Result<Tag, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.create_tag(request).map_err(|e| format!("创建标签失败: {}", e))
}

#[tauri::command]
pub fn get_all_tags(state: State<AppState>) -> Result<Vec<Tag>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_all_tags().map_err(|e| format!("获取标签列表失败: {}", e))
}

#[tauri::command]
pub fn update_tag(
    state: State<AppState>,
    request: UpdateTagRequest,
) -> Result<Tag, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.update_tag(request).map_err(|e| format!("更新标签失败: {}", e))
}

#[tauri::command]
pub fn delete_tag(state: State<AppState>, id: i64) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.delete_tag(id).map_err(|e| format!("删除标签失败: {}", e))
}

// === 设置相关命令 ===

#[tauri::command]
pub fn get_app_settings(state: State<AppState>) -> Result<AppSettings, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_app_settings().map_err(|e| format!("获取应用设置失败: {}", e))
}

#[tauri::command]
pub fn update_app_settings(
    state: State<AppState>,
    request: UpdateSettingsRequest,
) -> Result<AppSettings, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.update_app_settings(request).map_err(|e| format!("更新应用设置失败: {}", e))
}

// === 窗口管理命令 ===

#[tauri::command]
pub fn handle_close_request(
    state: State<AppState>,
    window: tauri::Window
) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    let settings = db.get_app_settings().map_err(|e| format!("获取设置失败: {}", e))?;
    
    match settings.close_behavior {
        crate::models::CloseBehavior::Exit => {
            // 直接退出应用
            window.app_handle().exit(0);
            Ok("exit".to_string())
        },
        crate::models::CloseBehavior::Minimize => {
            // 最小化到托盘
            let _ = window.hide();
            Ok("minimize".to_string())
        },
        crate::models::CloseBehavior::Ask => {
            // 返回"ask"，前端会显示询问对话框
            Ok("ask".to_string())
        }
    }
}

#[tauri::command]
pub fn force_exit_app(window: tauri::Window) -> Result<(), String> {
    window.app_handle().exit(0);
    Ok(())
}

#[tauri::command]
pub fn minimize_to_tray(window: tauri::Window) -> Result<(), String> {
    window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
    Ok(())
}

// === 通知相关命令 ===

#[tauri::command]
pub fn send_notification(
    app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;
    
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .icon("icons/icon.ico") // 尝试设置图标
        .show()
        .map_err(|e| format!("发送通知失败: {}", e))?;
    
    Ok(())
}

#[tauri::command]
pub fn check_overdue_tasks(state: State<AppState>) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_overdue_tasks().map_err(|e| format!("获取过期任务失败: {}", e))
}

#[tauri::command]
pub fn get_upcoming_tasks(
    state: State<AppState>,
    minutes_ahead: i32,
) -> Result<Vec<Task>, String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    db.get_upcoming_tasks(minutes_ahead).map_err(|e| format!("获取即将到期任务失败: {}", e))
}

#[tauri::command]
pub fn update_tasks_sort_order(
    state: State<AppState>,
    request: UpdateTaskSortOrderRequest,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| format!("数据库锁定失败: {}", e))?;
    let task_orders: Vec<(i64, i64)> = request.task_orders
        .into_iter()
        .map(|item| (item.id, item.sort_order))
        .collect();
    db.update_tasks_sort_order(task_orders).map_err(|e| format!("更新任务排序失败: {}", e))
}