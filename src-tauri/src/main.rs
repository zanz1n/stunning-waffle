#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod messaging;
mod state_machine;

use state_machine::StateMachine;

fn main() {
    let state_manager =
        StateMachine::new("/dev/ttyACM0", 115_200).expect("Failed to connect to serial console");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| state_manager.tauri_app_setup(app))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
