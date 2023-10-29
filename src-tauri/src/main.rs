#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod messaging;
mod state_machine;

use state_machine::StateStorage;

fn main() {
    let state_manager =
        StateStorage::new("/dev/ttyACM0", 9600).expect("Failed to connect to serial console");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| state_manager.tauri_app_setup(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
