use serde::{Serialize, Deserialize};
use uuid::Uuid;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize, Deserialize)]
pub struct JwtParts {
    pub header: String,
    pub payload: String,
}

#[tauri::command]
pub fn decode_jwt(token: String) -> Result<JwtParts, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 2 { return Err("Invalid JWT".to_string()); }
    let decode_part = |part: &str| -> Result<String, String> {
        let bytes = general_purpose::URL_SAFE_NO_PAD.decode(part).map_err(|e| e.to_string())?;
        String::from_utf8(bytes).map_err(|e| e.to_string())
    };
    Ok(JwtParts { header: decode_part(parts[0])?, payload: decode_part(parts[1])? })
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
pub fn generate_bulk_uuid(version: String, count: usize) -> Vec<String> { (0..count).map(|_| generate_uuid(version.clone())).collect() }

#[tauri::command]
pub fn base64_encode(input: String) -> String { general_purpose::STANDARD.encode(input) }

#[tauri::command]
pub fn base64_decode(input: String) -> Result<String, String> {
    let bytes = general_purpose::STANDARD.decode(input).map_err(|e| e.to_string())?;
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
