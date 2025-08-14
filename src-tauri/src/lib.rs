pub mod models;
pub mod database;
pub mod commands;

use database::Database;
use std::sync::Mutex;
use tauri::{tray::TrayIconBuilder, Manager, Emitter, menu::{MenuBuilder, MenuItem, PredefinedMenuItem}};

// 全局数据库状态
pub struct AppState {
    pub db: Mutex<Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // 初始化数据库
  let database = Database::new().expect("Failed to initialize database");
  let app_state = AppState {
    db: Mutex::new(database),
  };

  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .manage(app_state)
    .invoke_handler(tauri::generate_handler![
      commands::create_task,
      commands::get_all_tasks,
      commands::get_today_tasks,
      commands::get_completed_tasks,
      commands::update_task,
      commands::delete_task,
      commands::toggle_task_completion,
      commands::export_tasks_to_json,
      commands::import_tasks_from_json,
      commands::clear_all_tasks,
      commands::create_tag,
      commands::get_all_tags,
      commands::update_tag,
      commands::delete_tag,
      commands::get_app_settings,
      commands::update_app_settings,
      commands::handle_close_request,
      commands::force_exit_app,
      commands::minimize_to_tray,
      commands::send_notification,
      commands::check_overdue_tasks,
      commands::get_upcoming_tasks,
      commands::update_tasks_sort_order,
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // 创建托盘菜单
      let show_item = MenuItem::with_id(app, "show", "显示主界面", true, None::<&str>)?;
      let add_task_item = MenuItem::with_id(app, "add_task", "快速添加任务", true, None::<&str>)?;
      let separator1 = PredefinedMenuItem::separator(app)?;
      let settings_item = MenuItem::with_id(app, "settings", "设置", true, None::<&str>)?;
      let separator2 = PredefinedMenuItem::separator(app)?;
      let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
      
      let menu = MenuBuilder::new(app)
        .items(&[&show_item, &add_task_item, &separator1, &settings_item, &separator2, &quit_item])
        .build()?;
      
      // 创建系统托盘
      let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("MyTodo - 个人待办事项")
        .on_tray_icon_event(|tray, event| {
          match event {
            tauri::tray::TrayIconEvent::Click { button, rect: _, .. } => {
              match button {
                tauri::tray::MouseButton::Left => {
                  // 左键单击：显示窗口
                  if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                  }
                }
                _ => {}
              }
            }
            tauri::tray::TrayIconEvent::DoubleClick { button, rect: _, .. } => {
              match button {
                tauri::tray::MouseButton::Left => {
                  // 左键双击：如果窗口可见则隐藏
                  if let Some(window) = tray.app_handle().get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                      let _ = window.hide();
                    }
                  }
                }
                _ => {}
              }
            }
            _ => {}
          }
        })
        .on_menu_event(|app, event| {
          match event.id().as_ref() {
            "show" => {
              // 显示主界面
              if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
                // 确保窗口在前台
                let _ = window.set_always_on_top(true);
                let _ = window.set_always_on_top(false);
              }
            }
            "add_task" => {
              // 显示主界面并触发添加任务
              if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
                // 确保窗口在前台
                let _ = window.set_always_on_top(true);
                let _ = window.set_always_on_top(false);
                // 发送事件到前端来显示添加任务表单
                let _ = window.emit("show-add-task", ());
              }
            }
            "settings" => {
              // 显示设置页面
              if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
                // 确保窗口在前台
                let _ = window.set_always_on_top(true);
                let _ = window.set_always_on_top(false);
                // 发送事件到前端来显示设置页面
                let _ = window.emit("show-settings", ());
              }
            }
            "quit" => {
              // 退出应用
              app.exit(0);
            }
            _ => {}
          }
        })
        .build(app)?;
      
      // 处理窗口关闭事件
      if let Some(window) = app.get_webview_window("main") {
        let window_clone = window.clone();
        window.on_window_event(move |event| {
          match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              // 阻止默认关闭行为
              api.prevent_close();
              
              // 通过JS事件通知前端处理关闭请求
              let _ = window_clone.emit("close-requested", ());
            }
            _ => {}
          }
        });
      }
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
