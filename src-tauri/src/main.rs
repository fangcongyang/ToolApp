// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod utils;
mod conf;

use app::{ cmd, menu, setup, window, webssh };
use conf::AppConf;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{
  fern::colors::{Color, ColoredLevelConfig},
  LogTarget,
};
use std::env;

struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Drop for Defer<F> {
  fn drop(&mut self) {
    if let Some(f) = self.0.take() {
      f()
    }
  }
}

fn main() {
    // 初始化写入配置文件
    let app_conf = AppConf::read().write();
    let context = tauri::generate_context!();

    let mut log = tauri_plugin_log::Builder::default()
    .targets([
      LogTarget::Folder(utils::app_root()),
      LogTarget::Stdout,
      LogTarget::Webview,
    ])
    .level(log::LevelFilter::Debug);

    if cfg!(debug_assertions) {
        log = log.with_colors(ColoredLevelConfig {
          error: Color::Red,
          warn: Color::Yellow,
          debug: Color::Blue,
          info: Color::BrightGreen,
          trace: Color::Cyan,
        });
    }
    let mut builder = tauri::Builder::default()
    .plugin(log.build())
    .plugin(tauri_plugin_positioner::init())
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![
      cmd::drag_window,
      cmd::fullscreen,
      cmd::download,
      cmd::save_file,
      cmd::open_link,
      // cmd::run_check_update,
      cmd::open_file,
      cmd::get_data,
      conf::cmd::get_app_conf,
      conf::cmd::reset_app_conf,
      conf::cmd::get_theme,
      conf::cmd::form_confirm,
      conf::cmd::form_cancel,
      conf::cmd::form_msg,
      window::cmd::wa_window,
      window::cmd::control_window,
      window::cmd::window_reload,
      window::cmd::dalle2_search_window,
      webssh::cmd::select_ssh_main,
      webssh::cmd::select_ssh_main_dto,
      webssh::cmd::save_ssh,
      webssh::cmd::del_ssh,
      webssh::cmd::close_webssh
    ])
    .setup(setup::init);

    if app_conf.tray {
        builder = builder.system_tray(menu::tray_menu());
    }

    if app_conf.save_window_state {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build());
    }

    builder
    .on_menu_event(menu::menu_handler)
    .on_system_tray_event(menu::tray_handler)
    .on_window_event(move |event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        let win = event.window().clone();
        let app_conf = AppConf::read();
        if win.label() == "main" {
          if app_conf.isinit {
            tauri::api::dialog::ask(
              Some(event.window()),
              "",
              "你确定退出程序吗？按[x]进行退出",
              move |is_ok| {
                app_conf
                  .amend(serde_json::json!({ "isinit" : false, "main_close": is_ok }))
                  .write();
                if is_ok {
                  std::process::exit(0);
                } else {
                  win.minimize().unwrap();
                }
              },
            );
          } else if app_conf.main_close {
            std::process::exit(0);
          } else {
            win.minimize().unwrap();
          }
        } else if win.label() == "webssh"{
          webssh::close_webssh();
          event.window().close().unwrap();
        } else {
          event.window().close().unwrap();
        }
        api.prevent_close();
      }
    })
    .run(context)
    .expect("运行complex应用失败");
}