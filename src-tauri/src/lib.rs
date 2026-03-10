use tauri::Manager;

const URL_FILENAME: &str = "server_url.txt";
const DEFAULT_URL: &str = "http://100.123.154.91:3001";

fn saved_url(app: &tauri::AppHandle) -> Option<String> {
    let path = app.path().app_config_dir().ok()?.join(URL_FILENAME);
    let s = std::fs::read_to_string(path).ok()?;
    let s = s.trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

#[tauri::command]
fn get_server_url(app: tauri::AppHandle) -> String {
    saved_url(&app).unwrap_or_else(|| DEFAULT_URL.to_string())
}

#[tauri::command]
fn save_server_url(app: tauri::AppHandle, url: String) -> Result<(), String> {
    let dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    std::fs::write(dir.join(URL_FILENAME), url).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn navigate_to(window: tauri::WebviewWindow, url: String) -> Result<(), String> {
    let parsed = tauri::Url::parse(&url).map_err(|e| e.to_string())?;
    window.navigate(parsed).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_server_url, save_server_url, navigate_to])
        .setup(|app| {
            if let Some(url_str) = saved_url(&app.handle()) {
                if let Ok(parsed) = tauri::Url::parse(&url_str) {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.navigate(parsed);
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
