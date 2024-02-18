// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use app::io::read_file_bytes;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
async fn start_recording(recorder: tauri::State<'_, app::AudioRecorder>) -> Result<(), String> {
    let _ = recorder.start();
    Ok(())
}

#[tauri::command]
async fn stop_recording(recorder: tauri::State<'_, app::AudioRecorder>) -> Result<(), String> {
    recorder.stop();
    Ok(())
}

#[tauri::command]
async fn get_audio(recorder: tauri::State<'_, app::AudioRecorder>) -> Result<String, String> {
    while !recorder.is_stopped() {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let data = read_file_bytes(&recorder.path).unwrap_or_else(|e| {
        println!("Error reading file: {:?}", e);
        "Error".to_string()
    });
    Ok(data)
}

fn main() {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit", "Quit"))
        .add_item(CustomMenuItem::new("about", "About"));
    let tray = SystemTray::new().with_menu(menu);
    tauri::Builder::default()
        .manage(app::AudioRecorder::default())
        .system_tray(tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => app.exit(0),
                    "about" => {
                        println!("about");
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            stop_recording,
            get_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
