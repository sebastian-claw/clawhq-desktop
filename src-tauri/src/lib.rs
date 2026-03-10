use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // Enable Cmd+R / Ctrl+R / F5 to reload
            window.on_window_event(|_| {});
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::KeyboardInput { event: key_event, .. } = event {
                let _ = (window, key_event);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
