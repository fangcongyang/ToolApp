use crate::{conf::AppConf, utils};
use log::{error, info};
use tauri::{utils::config::WindowUrl, window::WindowBuilder, App, GlobalShortcutManager, Manager};
use window_shadows::set_shadow;
use tauri_plugin_window_state::{WindowExt, StateFlags};
use wry::application::accelerator::Accelerator;

pub fn init(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
  info!("stepup");
  let app_conf = AppConf::read();

  if let Some(v) = app_conf.clone().global_shortcut {
    info!("global_shortcut: `{}`", v);
    match v.parse::<Accelerator>() {
      Ok(_) => {
        info!("global_shortcut_register");
        let handle = app.app_handle();
        let mut shortcut = app.global_shortcut_manager();
        shortcut
          .register(&v, move || {
            if let Some(w) = handle.get_window("core") {
              if w.is_visible().unwrap() {
                w.hide().unwrap();
              } else {
                w.show().unwrap();
                w.set_focus().unwrap();
              }
            }
          })
          .unwrap_or_else(|err| {
            error!("global_shortcut_register_error: {}", err);
          });
      }
      Err(err) => {
        error!("global_shortcut_parse_error: {}", err);
      }
    }
  } else {
    info!("global_shortcut_unregister");
  };

  let app_conf2 = app_conf.clone();
  if app_conf.hide_dock_icon {
    #[cfg(target_os = "macos")]
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
  } else {
    let app = app.handle();
    tauri::async_runtime::spawn(async move {
      let main_win = WindowBuilder::new(&app, "main", WindowUrl::App("index.html".into()))
        .title("complex")
        .resizable(true)
        .fullscreen(false)
        .inner_size(app_conf2.systemConf.mainWidth, app_conf2.systemConf.mainHeight)
        .min_inner_size(app_conf2.systemConf.mainWidth, app_conf2.systemConf.mainHeight)
        .center()
        .decorations(false)
        .always_on_top(app_conf2.stay_on_top)
        .initialization_script(&utils::user_script())
        .initialization_script(include_str!("../scripts/core.js"))
        .user_agent(&app_conf2.ua_window);

      #[cfg(target_os = "macos")]
      {
        main_win = main_win
          .title_bar_style(app_conf2.clone().titlebar())
          .hidden_title(true);
      }

      let main = main_win.build().unwrap();
  
      if app_conf.systemConf.saveWindowState {
        main.restore_state(StateFlags::all()).expect("还原窗口失败");
      } else {
        #[cfg(any(windows, target_os = "macos"))]
        set_shadow(&main, true).expect("不支持的平台!");
      }
    });
  }

  // auto_update
  let auto_update = app_conf.get_auto_update();
  if auto_update != "disable" {
    info!("run_check_update");
    let _app = app.handle();
    // utils::run_check_update(app, auto_update == "silent", None);
  }

  Ok(())
}
