use base64::{engine::general_purpose, Engine as _};
use celerix_store::{sdk, CelerixStore};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::sync::Arc;
use tauri::Manager;
use tauri::WebviewWindow;
use tokio::sync::OnceCell;
use uuid::Uuid;
use xcap::Window;

static STORE: OnceCell<Arc<dyn CelerixStore>> = OnceCell::const_new();

async fn get_store(app_handle: tauri::AppHandle) -> Result<Arc<dyn CelerixStore>, String> {
    if let Some(store) = STORE.get() {
        return Ok(store.clone());
    }

    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("data");

    let store = sdk::new(data_dir.to_str().ok_or("Invalid path")?)
        .await
        .map_err(|e| e.to_string())?;

    // Ensure "default" persona exists for our single-user toolbox context
    let _ = store
        .set("default", "toolbox", "_init", serde_json::json!(true))
        .await;

    let _ = STORE.set(store.clone());
    Ok(store)
}

#[tauri::command]
pub async fn store_get(
    app_handle: tauri::AppHandle,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let store = get_store(app_handle).await?;
    match store.get("default", "toolbox", &key).await {
        Ok(val) => Ok(Some(val)),
        Err(celerix_store::Error::KeyNotFound)
        | Err(celerix_store::Error::AppNotFound)
        | Err(celerix_store::Error::PersonaNotFound) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn store_set(
    app_handle: tauri::AppHandle,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = get_store(app_handle).await?;
    store
        .set("default", "toolbox", &key, value)
        .await
        .map_err(|e| e.to_string())
}

#[derive(Serialize, Deserialize)]
pub struct JwtParts {
    pub header: String,
    pub payload: String,
}

#[tauri::command]
pub async fn capture_window(window: WebviewWindow) -> Result<String, String> {
    // 1. Get the current window title to help identify it
    let title = window.title().map_err(|e| e.to_string())?;

    // 2. Get all windows and find our match
    let windows = Window::all().map_err(|e| e.to_string())?;

    // Attempt to find the window by title.
    // In Tauri, the window title is usually unique enough for this app.
    let x_window = windows
        .into_iter()
        .find(|w| w.title() == title)
        .ok_or_else(|| "Could not find application window for capture".to_string())?;

    // 3. Capture the window
    let image_buffer = x_window.capture_image().map_err(|e| e.to_string())?;

    // 4. Encode to PNG
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    image_buffer
        .write_to(&mut cursor, xcap::image::ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(general_purpose::STANDARD.encode(buffer))
}

#[tauri::command]
pub fn decode_jwt(token: String) -> Result<JwtParts, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 {
        return Err("Invalid JWT".to_string());
    }
    let decode_part = |part: &str| -> Result<String, String> {
        let bytes = general_purpose::URL_SAFE_NO_PAD
            .decode(part)
            .map_err(|e| e.to_string())?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    };
    Ok(JwtParts {
        header: decode_part(parts[0])?,
        payload: decode_part(parts[1])?,
    })
}

#[tauri::command]
pub fn generate_uuid(version: String) -> String {
    match version.as_str() {
        "v1" => Uuid::now_v1(&[0, 0, 0, 0, 0, 0]).to_string(),
        "v4" => Uuid::new_v4().to_string(),
        "v7" => Uuid::now_v7().to_string(),
        "nil" => Uuid::nil().to_string(),
        _ => Uuid::new_v4().to_string(),
    }
}

#[tauri::command]
pub fn generate_bulk_uuid(version: String, count: usize) -> Vec<String> {
    (0..count).map(|_| generate_uuid(version.clone())).collect()
}

#[tauri::command]
pub fn base64_encode(input: String) -> String {
    general_purpose::STANDARD.encode(input)
}

#[tauri::command]
pub fn base64_decode(input: String) -> Result<String, String> {
    let bytes = general_purpose::STANDARD
        .decode(input)
        .map_err(|e| e.to_string())?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
