use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn read_asset_base64(base_dir: &str, asset_path: &str) -> Result<String, String> {
    if asset_path.is_empty() {
        return Ok(String::new());
    }

    let base = Path::new(base_dir);
    let full_path = base.join(asset_path);

    let data =
        fs::read(&full_path).map_err(|e| format!("Failed to read asset {}: {}", asset_path, e))?;

    let mime = match full_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    };

    Ok(format!("data:{};base64,{}", mime, STANDARD.encode(&data)))
}
