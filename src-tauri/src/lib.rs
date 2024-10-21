mod commandes;
mod state;

use commandes::{get_state, set_debug, set_state, start_counting};
use state::MachineState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MachineState::default())

        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            start_counting,
            set_state,
            get_state,
            set_debug
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}