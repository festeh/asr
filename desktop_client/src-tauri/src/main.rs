// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn start_recording() -> String {
    // sleep for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));
    "Recording started".to_string()
}

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
