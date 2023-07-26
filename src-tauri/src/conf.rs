use log::{error, info};
use serde_json::Value;
use std::{collections::BTreeMap, path::PathBuf};
use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;

use crate::utils::{app_root, create_file, exists};

// pub const BUY_COFFEE: &str = "https://www.buymeacoffee.com/lencx";

pub const APP_CONF_PATH: &str = "complex.conf.json";
pub const CHATGPT_URL: &str = "https://chat.openai.com";
pub const UA_MOBILE: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/604.1";

macro_rules! pub_struct {
  ($name:ident {$($field:ident: $t:ty,)*}) => {
    #[allow(non_snake_case)]
    #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
    pub struct $name {
      $(pub $field: $t),*
    }
  }
}

pub_struct!(SystemConf {
  saveWindowState: bool,
  mainWidth: f64,
  mainHeight: f64,
  theme: String,
});

impl SystemConf {
  pub fn new() -> Self {
    Self { 
      saveWindowState: false,
      mainWidth: 1080.0,
      mainHeight: 720.0,
      theme: "theme-light".into(),
    }
  }
}

pub_struct!(AppConf {
  systemConf: SystemConf,
  titlebar: bool,
  hide_dock_icon: bool,
  // auto update policy: prompt / silent / disable
  auto_update: String,
  stay_on_top: bool,
  global_shortcut: Option<String>,
  default_origin: String,
  speech_lang: String,

  // Main Window
  isinit: bool,
  popup_search: bool,
  main_close: bool,
  main_dashboard: bool,
  main_origin: String,
  ua_window: String,

  // Tray Window
  tray_width: f64,
  tray_height: f64,
  tray: bool,
  tray_dashboard: bool,
  tray_origin: String,
  ua_tray: String,
});

impl AppConf {
  pub fn new() -> Self {
    info!("conf_init");
    Self {
      systemConf: SystemConf::new(),
      titlebar: !cfg!(target_os = "macos"),
      hide_dock_icon: false,
      auto_update: "prompt".into(),
      #[cfg(target_os = "macos")]
      speech_lang: "com.apple.eloquence.en-US.Rocko".into(),
      #[cfg(not(target_os = "macos"))]
      speech_lang: "".into(),
      tray: true,
      popup_search: false,
      isinit: true,
      main_close: false,
      stay_on_top: false,
      main_dashboard: true,
      tray_dashboard: false,
      tray_width: 360.0,
      tray_height: 540.0,
      main_origin: CHATGPT_URL.into(),
      tray_origin: CHATGPT_URL.into(),
      default_origin: CHATGPT_URL.into(),
      ua_tray: UA_MOBILE.into(),
      ua_window: "".into(),
      global_shortcut: None,
    }
  }

  pub fn file_path() -> PathBuf {
    app_root().join(APP_CONF_PATH)
  }

  pub fn read() -> Self {
    match std::fs::read_to_string(Self::file_path()) {
      Ok(v) => {
        if let Ok(v2) = serde_json::from_str::<AppConf>(&v) {
          v2
        } else {
          error!("conf_read_parse_error");
          Self::default()
        }
      }
      Err(err) => {
        error!("conf_read_error: {}", err);
        Self::default()
      }
    }
  }

  pub fn write(self) -> Self {
    let path = &Self::file_path();
    if !exists(path) {
      create_file(path).unwrap();
      info!("conf_create");
    }
    if let Ok(v) = serde_json::to_string_pretty(&self) {
      std::fs::write(path, v).unwrap_or_else(|err| {
        error!("conf_write: {}", err);
        Self::default().write();
      });
    } else {
      error!("conf_ser");
    }
    self
  }

  pub fn amend(self, json: Value) -> Self {
    let val = serde_json::to_value(&self).unwrap();
    let mut config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    let new_json: BTreeMap<String, Value> = serde_json::from_value(json).unwrap();

    for (k, v) in new_json {
      config.insert(k, v);
    }

    match serde_json::to_string_pretty(&config) {
      Ok(v) => match serde_json::from_str::<AppConf>(&v) {
        Ok(v) => v,
        Err(err) => {
          error!("conf_amend_parse: {}", err);
          self
        }
      },
      Err(err) => {
        error!("conf_amend_str: {}", err);
        self
      }
    }
  }

  #[cfg(target_os = "macos")]
  pub fn titlebar(self) -> TitleBarStyle {
    if self.titlebar {
      TitleBarStyle::Transparent
    } else {
      TitleBarStyle::Overlay
    }
  }

  pub fn get_auto_update(self) -> String {
    self.auto_update.to_lowercase()
  }
  
  pub fn get_conf_by_name(self, conf_name: &str) -> String {
    let val = serde_json::to_value(&self).unwrap();
    let config: BTreeMap<String, Value> = serde_json::from_value(val).unwrap();
    if config.contains_key(conf_name) {
      info!("{}", serde_json::to_string(&config.get(conf_name)).unwrap());
      serde_json::to_string(&config.get(conf_name)).unwrap()
    } else {
      error!("配置不存在: {}", conf_name);
      "".into()
    }
  }

  pub fn restart(self, app: tauri::AppHandle) {
    tauri::api::process::restart(&app.env());
  }
}

impl Default for AppConf {
  fn default() -> Self {
    Self::new()
  }
}

pub mod cmd {
  use super::AppConf;
  use tauri::{command, AppHandle, Manager};

  #[command]
  pub fn get_app_conf() -> AppConf {
    AppConf::read()
  }

  #[command]
  pub fn reset_app_conf() -> AppConf {
    AppConf::default().write()
  }

  #[command]
  pub fn get_conf_by_name(conf_name: &str) -> String {
    AppConf::read().get_conf_by_name(conf_name)
  }

  #[command]
  pub fn form_confirm(_app: AppHandle, data: serde_json::Value) {
    AppConf::read().amend(serde_json::json!(data)).write();
  }

  #[command]
  pub fn form_cancel(app: AppHandle, label: &str, title: &str, msg: &str) {
    let win = app.app_handle().get_window(label).unwrap();
    tauri::api::dialog::ask(
      app.app_handle().get_window(label).as_ref(),
      title,
      msg,
      move |is_cancel| {
        if is_cancel {
          win.close().unwrap();
        }
      },
    );
  }

  #[command]
  pub fn form_msg(app: AppHandle, label: &str, title: &str, msg: &str) {
    let win = app.app_handle().get_window(label);
    tauri::api::dialog::message(win.as_ref(), title, msg);
  }
}
