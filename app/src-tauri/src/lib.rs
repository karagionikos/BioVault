mod specter;

use tauri::Manager;

/// Spike 0 check: the interface reaches Rust on this device and gets its OS back.
#[tauri::command]
fn platform() -> &'static str {
    std::env::consts::OS
}

/// Spike 0 check: SPECTER2 fingerprints the sample abstract on this device, timed.
/// Model files go in `<app data dir>/models` (on the phone: pushed with adb).
#[tauri::command]
async fn fingerprint_sample(app: tauri::AppHandle) -> Result<String, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("models");
    tauri::async_runtime::spawn_blocking(move || {
        specter::spike_report(&dir).map_err(|e| format!("{e:#}"))
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![platform, fingerprint_sample])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
