// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::Path;
use std::process::Command;

#[tauri::command]
fn open_executable(path: &str) -> bool {
    let executable_path = Path::new(path);

    if !executable_path.exists() {
        eprintln!("Executable path does not exist: {}", path);
        return false;
    }

    match Command::new(executable_path).spawn() {
        Ok(_) => {
            println!("Successfully opened executable: {}", path);
            true
        }
        Err(e) => {
            eprintln!("Failed to open executable: {}. Error: {}", path, e);
            false
        }
    }
}

#[tauri::command]
fn open_with_firefox(path: &str) -> bool {
    if !Path::new(path).exists() {
        eprintln!("Path does not exist: {}", path);
        return false;
    }

    match open::with(path, "firefox") {
        Ok(_) => {
            println!("Successfully opened with Firefox: {}", path);
            true
        }
        Err(e) => {
            eprintln!("Failed to open with Firefox: {}. Error: {}", path, e);
            false
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_executable, open_with_firefox])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
