
pub mod cmd {
  use log::info;
  use tauri::{command, Manager};
use window_shadows::set_shadow;

use crate::conf::AppConf;

  #[command]
  pub fn wa_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    url: String,
    path: String,
  ) {
    info!("wa_window: {} :=> {}", title, url);
    let win = app.get_window(&label);
    let app_conf = AppConf::read();
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        let new_window = tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App((url + &path).parse().unwrap()))
          .title(title)
          .inner_size(app_conf.systemConf.mainWidth, app_conf.systemConf.mainHeight)
          .min_inner_size(app_conf.systemConf.mainWidth, app_conf.systemConf.mainHeight)
          .resizable(true)
          .decorations(false)
          .center()
          .build()
          .unwrap();
  
        #[cfg(any(windows, target_os = "macos"))]
        set_shadow(&new_window, true).expect("不支持的平台!");
      });
    } else if let Some(v) = win {
      if !v.is_visible().unwrap() {
        v.show().unwrap();
      }
      v.eval(&("window.location.hash = '".to_owned() + &path + "'")).unwrap();
      v.set_focus().unwrap();
    }
  }

  #[command]
  pub fn window_reload(app: tauri::AppHandle, label: &str) {
    app
      .app_handle()
      .get_window(label)
      .unwrap()
      .eval("window.location.reload()")
      .unwrap();
  }
}
