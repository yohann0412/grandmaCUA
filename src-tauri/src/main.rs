// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn capture_screen() -> Result<String, String> {
    let screens = screenshots::Screen::all().map_err(|e| e.to_string())?;
    let mut images = vec![];
    for screen in screens {
        let image = screen.capture().map_err(|e| e.to_string())?;
        let path = format!("screenshot-{}.png", screen.display_info.id);
        
        // Convert Image to PNG bytes and save
        let png_data = image.to_png().map_err(|e| e.to_string())?;
        std::fs::write(&path, png_data).map_err(|e| e.to_string())?;
        
        images.push(path);
    }
    Ok(images.join(","))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, capture_screen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
