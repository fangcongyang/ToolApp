use crate::conf::AppConf;
use tauri::{
  AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
  SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::on_tray_event;

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

// --- SystemTray Menu
pub fn tray_menu() -> SystemTray {
  if cfg!(target_os = "macos") {
    let mut tray_menu = SystemTrayMenu::new();

    if AppConf::read().hide_dock_icon {
      tray_menu = tray_menu.add_item(CustomMenuItem::new(
        "show_dock_icon".to_string(),
        "Show Dock Icon",
      ));
    } else {
      tray_menu = tray_menu
        .add_item(CustomMenuItem::new(
          "hide_dock_icon".to_string(),
          "Hide Dock Icon",
        ))
        .add_item(CustomMenuItem::new("show_main".to_string(), "显示"));
    }

    SystemTray::new().with_menu(
      tray_menu
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出")),
    )
  } else {
    SystemTray::new().with_menu(
      SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show_main".to_string(), "显示"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出")),
    )
  }
}

// --- SystemTray Event
pub fn tray_handler(handle: &AppHandle, event: SystemTrayEvent) {
  on_tray_event(handle, &event);

  let app = handle.clone();

  match event {
    SystemTrayEvent::LeftClick { 
      position: _,
      size: _,
      ..
    } => {
      let app_conf = AppConf::read();

      if !app_conf.hide_dock_icon {
        if let Some(core_win) = handle.get_window("main") {
          if core_win.is_visible().unwrap() {
            core_win.hide().unwrap();
          } else {
            core_win.show().unwrap();
            core_win.set_focus().unwrap();
          }
        }
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "restart" => tauri::api::process::restart(&handle.env()),
      "show_dock_icon" => {
        AppConf::read()
          .amend(serde_json::json!({ "hide_dock_icon": false }))
          .write()
          .restart(app);
      }
      "hide_dock_icon" => {
        let app_conf = AppConf::read();
        if !app_conf.hide_dock_icon {
          app_conf
            .amend(serde_json::json!({ "hide_dock_icon": true }))
            .write()
            .restart(app);
        }
      }
      "show_main" => {
        if let Some(core_win) = app.get_window("main") {
          let tray_win = app.get_window("tray").unwrap();
          if !core_win.is_visible().unwrap() {
            core_win.show().unwrap();
            tray_win.hide().unwrap();
          }
          if core_win.is_minimized().unwrap() {
            core_win.unminimize().unwrap()
          }
          core_win.set_focus().unwrap();
        };
      }
      "quit" => {
        std::process::exit(0);
      }
      _ => (),
    },
    _ => (),
  }
}

pub mod cmd {
  use tauri::{command, AppHandle, Manager};
  use tauri_plugin_window_state::{AppHandleExt, StateFlags};
  use crate::conf::AppConf;

  #[command]
  pub fn exist_app(app: AppHandle) {
    let main = app.get_focused_window().unwrap();
    let app_conf = AppConf::read();
    let save_window_state = app_conf.systemConf.saveWindowState;
    if app_conf.isinit {
      tauri::api::dialog::ask(
        Some(&app.get_focused_window().unwrap()),
        "退出",
        "你确定退出程序吗？按[x]进行退出",
        move |is_ok| {
            app_conf
            .amend(serde_json::json!({ "isinit" : false, "main_close": is_ok }))
            .write();
            if is_ok {
              if save_window_state {
                app.save_window_state(StateFlags::all()).expect("保存窗口状态失败"); 
              }
              std::process::exit(0);
            } else {
              main.minimize().unwrap();
            }
        },
      );
    } else if app_conf.main_close {
      if save_window_state {
        app.save_window_state(StateFlags::all()).expect("保存窗口状态失败"); 
      }
      std::process::exit(0);
    } else {
      main.minimize().unwrap();
    }
  }
}