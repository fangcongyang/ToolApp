use crate::{
  app::window,
  app::webssh,
  conf::{self, AppConf},
  utils,
};
use tauri::{
  AppHandle, CustomMenuItem, Manager, Menu, MenuItem, Submenu, SystemTray, SystemTrayEvent,
  SystemTrayMenu, SystemTrayMenuItem, WindowMenuEvent,
};
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

#[cfg(target_os = "macos")]
use tauri::AboutMetadata;

// --- Menu
pub fn init() -> Menu {
  let app_conf = AppConf::read();
  let name = "系统";
  let theme_light = CustomMenuItem::new("theme_light".to_string(), "明亮");
  let theme_dark = CustomMenuItem::new("theme_dark".to_string(), "暗黑");
  let theme_system = CustomMenuItem::new("theme_system".to_string(), "系统");
  let is_dark = app_conf.clone().theme_check("dark");
  let is_system = app_conf.clone().theme_check("system");
  
  let auto_update = app_conf.get_auto_update();
  let update_prompt = CustomMenuItem::new("update_prompt".to_string(), "Prompt");
  let update_silent = CustomMenuItem::new("update_silent".to_string(), "Silent");

  let app_menu = Submenu::new(
    name,
    Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuItem::About(name.into(), AboutMetadata::default()).into(),
      #[cfg(not(target_os = "macos"))]
      Submenu::new(
        "主题",
        Menu::new()
          .add_item(if is_dark || is_system {
            theme_light
          } else {
            theme_light.selected()
          })
          .add_item(if is_dark {
            theme_dark.selected()
          } else {
            theme_dark
          })
          .add_item(if is_system {
            theme_system.selected()
          } else {
            theme_system
          }),
      )
      .into(),
      Submenu::new(
        "自动更新",
        Menu::new()
          .add_item(if auto_update == "prompt" {
            update_prompt.selected()
          } else {
            update_prompt
          })
          .add_item(if auto_update == "silent" {
            update_silent.selected()
          } else {
            update_silent
          }), // .add_item(if auto_update == "disable" {
              //     update_disable.selected()
              // } else {
              //     update_disable
              // })
      )
      .into(),
      MenuItem::Separator.into(),
      CustomMenuItem::new("restart".to_string(), "重启")
        .accelerator("CmdOrCtrl+Shift+R")
        .into(),
    ]),
  );
  // let _update_disable = CustomMenuItem::new("update_disable".to_string(), "Disable");

  let view_menu = Submenu::new(
    "页面",
    Menu::new()
      .add_item(CustomMenuItem::new("go_back".to_string(), "返回").accelerator("CmdOrCtrl+["))
      .add_item(
        CustomMenuItem::new("go_forward".to_string(), "前进").accelerator("CmdOrCtrl+]"),
      )
      .add_item(
        CustomMenuItem::new("scroll_top".to_string(), "滚动到顶部")
          .accelerator("CmdOrCtrl+Up"),
      )
      .add_item(
        CustomMenuItem::new("scroll_bottom".to_string(), "滚动到底部")
          .accelerator("CmdOrCtrl+Down"),
      ),
  );

  let help_menu = Submenu::new(
    "帮助",
    Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuItem::About(name.into(), AboutMetadata::default()).into(),
      #[cfg(not(target_os = "macos"))]
      CustomMenuItem::new("about".to_string(), "关于Tauri").into(),
      CustomMenuItem::new("check_update".to_string(), "检查更新").into(),
      CustomMenuItem::new("dev_tools".to_string(), "开发者工具")
      .accelerator("f12").into()
    ]),
  );

  Menu::new()
    .add_submenu(app_menu)
    .add_submenu(view_menu)
    .add_submenu(help_menu)
}

// --- Menu Event
pub fn menu_handler(event: WindowMenuEvent<tauri::Wry>) {
  let win = Some(event.window()).unwrap();
  let app = win.app_handle();
  let script_path = utils::script_path().to_string_lossy().to_string();
  let menu_id = event.menu_item_id();
  let menu_handle = win.menu_handle();

  match menu_id {
    // App
    // Preferences
    "control_center" => window::cmd::control_window(app),
    "restart" => tauri::api::process::restart(&app.env()),
    "inject_script" => open(&app, script_path),
    "buy_coffee" => open(&app, conf::BUY_COFFEE.to_string()),
    "popup_search" => {
      let app_conf = AppConf::read();
      let popup_search = !app_conf.popup_search;
      menu_handle
        .get_item(menu_id)
        .set_selected(popup_search)
        .unwrap();
      app_conf
        .amend(serde_json::json!({ "popup_search": popup_search }))
        .write();
      window::cmd::window_reload(app.clone(), "core");
      window::cmd::window_reload(app, "tray");
    }
    "sync_prompts" => {
      tauri::api::dialog::ask(
        app.get_window("core").as_ref(),
        "Sync Prompts",
        "Data sync will enable all prompts, are you sure you want to sync?",
        move |is_restart| {
          if is_restart {
            app
              .get_window("core")
              .unwrap()
              .eval("window.__sync_prompts && window.__sync_prompts()")
              .unwrap()
          }
        },
      );
    }
    "hide_dock_icon" => {
      AppConf::read()
        .amend(serde_json::json!({ "hide_dock_icon": true }))
        .write()
        .restart(app);
    }
    "titlebar" => {
      let app_conf = AppConf::read();
      app_conf
        .clone()
        .amend(serde_json::json!({ "titlebar": !app_conf.titlebar }))
        .write()
        .restart(app);
    }
    "system_tray" => {
      let app_conf = AppConf::read();
      app_conf
        .clone()
        .amend(serde_json::json!({ "tray": !app_conf.tray }))
        .write()
        .restart(app);
    }
    "theme_light" | "theme_dark" | "theme_system" => {
      let theme = match menu_id {
        "theme_dark" => "dark",
        "theme_system" => "system",
        _ => "light",
      };
      AppConf::read()
        .amend(serde_json::json!({ "theme": theme }))
        .write()
        .restart(app);
    }
    "update_prompt" | "update_silent" | "update_disable" => {
      // for id in ["update_prompt", "update_silent", "update_disable"] {
      for id in ["update_prompt", "update_silent"] {
        menu_handle.get_item(id).set_selected(false).unwrap();
      }
      let auto_update = match menu_id {
        "update_silent" => {
          menu_handle
            .get_item("update_silent")
            .set_selected(true)
            .unwrap();
          "silent"
        }
        "update_disable" => {
          menu_handle
            .get_item("update_disable")
            .set_selected(true)
            .unwrap();
          "disable"
        }
        _ => {
          menu_handle
            .get_item("update_prompt")
            .set_selected(true)
            .unwrap();
          "prompt"
        }
      };
      AppConf::read()
        .amend(serde_json::json!({ "auto_update": auto_update }))
        .write();
    }

    // 页面
    "go_back" => win.eval("window.history.go(-1)").unwrap(),
    "go_forward" => win.eval("window.history.go(1)").unwrap(),
    "scroll_top" => win
      .eval(
        r#"window.scroll({
          top: 0,
          left: 0,
          behavior: "smooth"
          })"#,
      )
      .unwrap(),
    "scroll_bottom" => win
      .eval(
        r#"window.scroll({
          top: document.body.scrollHeight,
          left: 0,
          behavior: "smooth"})"#,
      )
      .unwrap(),
    // 帮助
    "about" => {
      let tauri_conf = utils::get_tauri_conf().unwrap();
      tauri::api::dialog::message(
        app.get_window("main").as_ref(),
        "tauri app",
        format!("Version {}", tauri_conf.package.version.unwrap()),
      );
    }
    "check_update" => {
      // utils::run_check_update(app, false, None);
    }
    "dev_tools" => {
      #[cfg(debug_assertions)]
      win.open_devtools();
      #[cfg(debug_assertions)]
      win.close_devtools();
    }
    _ => (),
  }
}

// --- SystemTray Menu
pub fn tray_menu() -> SystemTray {
  if cfg!(target_os = "macos") {
    let mut tray_menu = SystemTrayMenu::new()
      .add_item(CustomMenuItem::new(
        "control_center".to_string(),
        "Control Center",
      ))
      .add_native_item(SystemTrayMenuItem::Separator);

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
        .add_item(CustomMenuItem::new(
          "control_center".to_string(),
          "控制中心",
        ))
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

      if let Some(tray_win) = handle.get_window("tray") {
        tray_win.move_window(Position::TrayFixedCenter).unwrap();

        if tray_win.is_visible().unwrap() {
          tray_win.hide().unwrap();
        } else {
          tray_win.show().unwrap();
        }
      }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "control_center" => window::cmd::control_window(app),
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
        webssh::close_webssh();
        std::process::exit(0);
      }
      _ => (),
    },
    _ => (),
  }
}

pub fn open(app: &AppHandle, path: String) {
  tauri::api::shell::open(&app.shell_scope(), path, None).unwrap();
}
