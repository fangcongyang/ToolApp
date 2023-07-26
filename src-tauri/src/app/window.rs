
pub mod cmd {
  use log::info;
  use tauri::{command, Manager};
use window_shadows::set_shadow;

  #[command]
  pub fn wa_window(
    app: tauri::AppHandle,
    label: String,
    title: String,
    url: String
  ) {
    info!("wa_window: {} :=> {}", title, url);
    let win = app.get_window(&label);
    if win.is_none() {
      tauri::async_runtime::spawn(async move {
        let new_window = tauri::WindowBuilder::new(&app, label, tauri::WindowUrl::App(url.parse().unwrap()))
          .title(title)
          .inner_size(960.0, 700.0)
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
      v.eval("window.location.reload()").unwrap();
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
