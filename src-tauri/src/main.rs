#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod state;
mod command;
mod error;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![
      hello_tauri,
      command::set_language,
      command::generate_definitions,
      command::get_definition_vec
    ])
    .manage(state::Data::new())
    .run(context)
    .expect("error while running tauri application");
}

#[tauri::command]
fn hello_tauri() -> String {
  "hello".to_string()
}
