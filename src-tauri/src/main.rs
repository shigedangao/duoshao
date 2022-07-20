#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_log::{
  fern::colors::ColoredLevelConfig,
  LoggerBuilder,
  LogTarget
};

mod state;
mod command;
mod error;

fn main() {
  let context = tauri::generate_context!();
  let colors = ColoredLevelConfig::default();
  let targets = [
    LogTarget::LogDir,
    LogTarget::Stderr,
  ];

  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![
      command::set_language,
      command::generate_definitions,
      command::export_definition_to_csv
    ])
    .manage(state::Data::new())
    .plugin(
      LoggerBuilder::new()
        .with_colors(colors)
        .targets(targets)
        .build()
    )
    .run(context)
    .expect("error while running duoshao application");
}
