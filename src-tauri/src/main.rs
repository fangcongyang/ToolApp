// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod utils;
mod conf;
mod business;

use app::{ cmd, menu, setup, window };
use business::{ cron, webssh, route };
use conf::AppConf;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::{
  fern::colors::{Color, ColoredLevelConfig},
  LogTarget,
};
use std::env;

fn main() {
    // 初始化写入配置文件
    AppConf::read().write();
    let context = tauri::generate_context!();

    webssh::init();

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
    let builder = tauri::Builder::default()
    .plugin(log.build())
    .plugin(tauri_plugin_positioner::init())
    .plugin(tauri_plugin_autostart::init(
      MacosLauncher::LaunchAgent,
      None,
    ))
    .invoke_handler(tauri::generate_handler![
      cmd::download,
      cmd::save_file,
      cmd::open_link,
      // cmd::run_check_update,
      cmd::open_file,
      cmd::get_data,
      menu::cmd::exist_app,
      conf::cmd::get_app_conf,
      conf::cmd::reset_app_conf,
      conf::cmd::get_conf_by_name,
      conf::cmd::form_confirm,
      conf::cmd::form_cancel,
      conf::cmd::form_msg,
      window::cmd::wa_window,
      window::cmd::window_reload,
      webssh::cmd::select_ssh_main,
      webssh::cmd::select_ssh_main_dto,
      webssh::cmd::save_ssh,
      webssh::cmd::del_ssh,
      webssh::cmd::close_webssh,
      webssh::cmd::init_webssh,
      cron::cmd::next_trigger_time,
      route::cmd::save_route_active_keys,
      route::cmd::save_route_active_key,
      route::cmd::get_route_active_key,
    ])
    .setup(setup::init);

    builder.system_tray(menu::tray_menu())
    .on_system_tray_event(menu::tray_handler)
    .run(context)
    .expect("运行complex应用失败");
}