// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{
    io::read_file_bytes,
    resample::{resample_audio, write_resampled},
    AudioRecorder,
};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
async fn start_recording(recorder: tauri::State<'_, AudioRecorder>) -> Result<(), String> {
    let _ = recorder.start();
    Ok(())
}

#[tauri::command]
async fn stop_recording(recorder: tauri::State<'_, AudioRecorder>) -> Result<(), String> {
    println!("Is stopped? {}...", recorder.is_stopping());
    recorder.order_stop();
    while recorder.is_stopping() {
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("Waiting for recorder to stop...");
    }
    // check if the file exists
    let tmp_path = recorder.get_tmp_path();
    if !tmp_path.exists() {
        println!("File not found! {:?}", tmp_path);
        return Err("File not found".to_string());
    }
    let (left, right) = resample_audio(tmp_path.to_str().unwrap()).unwrap();
    recorder.set_path();
    let out = recorder.get_path();
    match out {
        Some(path) => {
            write_resampled(left, right, path.to_str().unwrap());
        }
        None => {
            println!("Path not set!");
            return Err("Path not set!".to_string());
        }
    }
    Ok(())
}

#[tauri::command]
async fn get_audio(recorder: tauri::State<'_, AudioRecorder>) -> Result<String, String> {
    let path = recorder.get_path();
    println!("Fetching audio... {:?}", path);
    if let Some(path) = path {
        let bytes = read_file_bytes(path.to_str().unwrap()).unwrap();
        println!("Fetched!");
        return Ok(bytes);
    }
    println!("File not found {:?}", path);
    Err("File not found!".to_string())
}

fn main() {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit", "Quit"))
        .add_item(CustomMenuItem::new("about", "About"));
    let tray = SystemTray::new().with_menu(menu);
    let recorder_state = app::AudioRecorder::default();
    tauri::Builder::default()
        .manage(recorder_state)
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
