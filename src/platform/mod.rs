//! Platform abstraction layer for cross-platform support (native and WASM).

use anyhow::Result;

/// Read a file as string (for scenarios, configs, saves).
#[cfg(not(target_arch = "wasm32"))]
pub fn read_file(path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
}

/// Write a string to a file.
#[cfg(not(target_arch = "wasm32"))]
pub fn write_file(path: &str, content: &str) -> Result<()> {
    if let Some(parent) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

/// Check if a file exists.
#[cfg(not(target_arch = "wasm32"))]
pub fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// WASM implementations using localStorage
#[cfg(target_arch = "wasm32")]
fn get_local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

/// Read from localStorage (WASM).
#[cfg(target_arch = "wasm32")]
pub fn read_file(path: &str) -> Result<String> {
    let storage = get_local_storage().ok_or_else(|| anyhow::anyhow!("localStorage not available"))?;
    storage
        .get_item(path)
        .map_err(|_| anyhow::anyhow!("Failed to read from localStorage"))?
        .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path))
}

/// Write to localStorage (WASM).
#[cfg(target_arch = "wasm32")]
pub fn write_file(path: &str, content: &str) -> Result<()> {
    let storage = get_local_storage().ok_or_else(|| anyhow::anyhow!("localStorage not available"))?;
    storage
        .set_item(path, content)
        .map_err(|_| anyhow::anyhow!("Failed to write to localStorage"))
}

/// Check if key exists in localStorage (WASM).
#[cfg(target_arch = "wasm32")]
pub fn file_exists(path: &str) -> bool {
    get_local_storage()
        .and_then(|s| s.get_item(path).ok())
        .flatten()
        .is_some()
}
