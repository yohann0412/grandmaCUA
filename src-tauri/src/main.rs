// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn capture_screen() -> Result<String, String> {
    // Get the user's home directory for saving screenshots
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let screenshots_dir = format!("{}/Desktop", home_dir);

    // Create screenshots directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&screenshots_dir) {
        eprintln!("Warning: Could not create screenshots directory: {}", e);
    }

    let screens = screenshots::Screen::all().map_err(|e| {
        eprintln!("Error getting screens: {}", e);
        e.to_string()
    })?;

    let mut images: Vec<String> = vec![];
    for screen in screens {
        let image = screen.capture().map_err(|e| {
            eprintln!("Error capturing screen {}: {}", screen.display_info.id, e);
            e.to_string()
        })?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let path = format!(
            "{}/screenshot-{}-{}.png",
            screenshots_dir, screen.display_info.id, timestamp
        );

        // Convert Image to PNG bytes and save
        let png_data = image.to_png(None).map_err(|e| {
            eprintln!("Error converting to PNG: {}", e);
            e.to_string()
        })?;

        std::fs::write(&path, png_data).map_err(|e| {
            eprintln!("Error saving file {}: {}", path, e);
            e.to_string()
        })?;

        images.push(path);
    }
    Ok(images.join(","))
}

fn main() {
    match tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, capture_screen])
        .run(tauri::generate_context!())
    {
        Ok(_) => println!("Tauri application exited successfully"),
        Err(e) => {
            eprintln!("Error running Tauri application: {}", e);
            std::process::exit(1);
        }
    }
}
