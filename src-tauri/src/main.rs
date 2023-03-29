#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod app;
mod utils;

use app::{builder, cmd};
use tauri::Manager;
use tauri_plugin_log::{
  fern::colors::{Color, ColoredLevelConfig},
  LogTarget,
};

fn main() {


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
  .invoke_handler(tauri::generate_handler![
    cmd::gpt_access_token::fetch_chat_api_by_access_token,
    cmd::gpt_api_key::fetch_chat_api_by_api_key,
    cmd::download::download_img,
    cmd::window::new_window
  ])
  .setup(builder::setup);

  #[cfg(target_os = "macos")]
  {
    builder = builder.on_window_event(move |event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        let win = event.window().clone();
        if win.label() == "core" {
          win.minimize().unwrap();
        }else {
          cmd::window::window_reload(event.window().app_handle(), "core");
          event.window().close().unwrap();
        }
        api.prevent_close();
      }
    })
  }

  builder.run(tauri::generate_context!())
  .expect("error while running tauri application");
}
