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
    let storage =
        get_local_storage().ok_or_else(|| anyhow::anyhow!("localStorage not available"))?;
    storage
        .get_item(path)
        .map_err(|_| anyhow::anyhow!("Failed to read from localStorage"))?
        .ok_or_else(|| anyhow::anyhow!("Key not found: {}", path))
}

/// Write to localStorage (WASM).
#[cfg(target_arch = "wasm32")]
pub fn write_file(path: &str, content: &str) -> Result<()> {
    let storage =
        get_local_storage().ok_or_else(|| anyhow::anyhow!("localStorage not available"))?;
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

/// Speak text using text-to-speech (native: no-op, WASM: Web Speech API).
#[cfg(not(target_arch = "wasm32"))]
pub fn speak_text(_text: &str) {
    // Native TTS would require platform-specific libraries
    // For now, this is a no-op on native platforms
}

/// Speak text using Web Speech API (WASM).
#[cfg(target_arch = "wasm32")]
pub fn speak_text(text: &str) {
    use wasm_bindgen::JsCast;

    if let Some(window) = web_sys::window() {
        if let Ok(speech_synthesis) = js_sys::Reflect::get(&window, &"speechSynthesis".into()) {
            if let Some(synthesis) = speech_synthesis.dyn_ref::<web_sys::SpeechSynthesis>() {
                // Cancel any ongoing speech
                synthesis.cancel();

                // Create new utterance
                if let Ok(utterance) = web_sys::SpeechSynthesisUtterance::new_with_text(text) {
                    // Set language to match content (default to Japanese for VN)
                    utterance.set_lang("ja-JP");
                    synthesis.speak(&utterance);
                }
            }
        }
    }
}

/// Copy text to clipboard (native).
#[cfg(not(target_arch = "wasm32"))]
pub fn copy_to_clipboard(text: &str) {
    // Use arboard or similar on native, for now just print
    // This allows external screen readers to access the text
    eprintln!("[Screen Reader] {}", text);
}

/// Copy text to clipboard (WASM).
#[cfg(target_arch = "wasm32")]
pub fn copy_to_clipboard(text: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(navigator) = window.navigator().clipboard() {
            let _ = navigator.write_text(text);
        }
    }
}

/// Check if TTS is available on this platform.
#[cfg(not(target_arch = "wasm32"))]
pub fn tts_available() -> bool {
    false // Native TTS not implemented yet
}

/// Check if TTS is available (WASM: Web Speech API).
#[cfg(target_arch = "wasm32")]
pub fn tts_available() -> bool {
    web_sys::window()
        .map(|w| js_sys::Reflect::has(&w, &"speechSynthesis".into()).unwrap_or(false))
        .unwrap_or(false)
}
