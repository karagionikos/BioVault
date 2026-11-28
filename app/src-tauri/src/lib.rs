mod lan;
mod specter;

use std::sync::Mutex;

use tauri::{Emitter, Manager};

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

/// Started on first use, so on Android it runs only after the local-network permission prompt.
static LAN: Mutex<Option<lan::Lan>> = Mutex::new(None);

fn lan(app: &tauri::AppHandle) -> Result<lan::Lan, String> {
    let mut slot = LAN.lock().map_err(|e| e.to_string())?;
    if let Some(lan) = &*slot {
        return Ok(lan.clone());
    }
    let app = app.clone();
    let lan = lan::Lan::start(move |line| {
        let _ = app.emit("lan", line); // nothing to do if the window is gone
    })
    .map_err(|e| e.to_string())?;
    Ok(slot.insert(lan).clone())
}

/// Spike 0 check: listen and advertise on the LAN. Incoming messages arrive as `lan` events.
#[tauri::command]
async fn lan_listen(app: tauri::AppHandle) -> Result<String, String> {
    let lan = lan(&app)?;
    Ok(format!("Listening as {} on port {}", lan.name, lan.port))
}

/// Spike 0 check: find another BioVault on the LAN and exchange one encrypted message with it.
#[tauri::command]
async fn lan_send(app: tauri::AppHandle) -> Result<String, String> {
    let lan = lan(&app)?;
    tauri::async_runtime::spawn_blocking(move || lan.find_and_send().map_err(|e| e.to_string()))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            platform,
            fingerprint_sample,
            lan_listen,
            lan_send
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
