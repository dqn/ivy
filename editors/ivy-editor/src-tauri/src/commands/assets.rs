use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize)]
pub struct AssetInfo {
    pub path: String,
    pub relative_path: String,
    pub name: String,
    pub asset_type: String,
    pub size: u64,
    pub modified: u64,
}

#[derive(Serialize)]
pub struct AssetTree {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<AssetTree>,
    pub asset_type: Option<String>,
}

fn get_asset_type(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => Some("image".to_string()),
        "mp3" | "ogg" | "wav" | "flac" => Some("audio".to_string()),
        "mp4" | "webm" | "avi" => Some("video".to_string()),
        "yaml" | "yml" => Some("scenario".to_string()),
        _ => None,
    }
}

#[tauri::command]
pub fn get_relative_path(base_dir: &str, file_path: &str) -> Result<String, String> {
    let base = Path::new(base_dir);
    let file = Path::new(file_path);

    if let Ok(relative) = file.strip_prefix(base) {
        return Ok(relative.to_string_lossy().to_string());
    }

    Ok(file_path.to_string())
}

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
        Some("mp3") => "audio/mpeg",
        Some("ogg") => "audio/ogg",
        Some("wav") => "audio/wav",
        Some("flac") => "audio/flac",
        _ => "application/octet-stream",
    };

    Ok(format!("data:{};base64,{}", mime, STANDARD.encode(&data)))
}

#[tauri::command]
pub fn list_assets(base_dir: &str) -> Result<AssetTree, String> {
    let base = Path::new(base_dir);
    if !base.exists() {
        return Err("Directory not found".to_string());
    }

    fn build_tree(path: &Path, base: &Path) -> AssetTree {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let relative = path
            .strip_prefix(base)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        if path.is_dir() {
            let mut children: Vec<AssetTree> = fs::read_dir(path)
                .into_iter()
                .flatten()
                .filter_map(|entry| entry.ok())
                .map(|entry| build_tree(&entry.path(), base))
                .filter(|child| child.is_dir || child.asset_type.is_some())
                .collect();

            children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });

            AssetTree {
                name,
                path: relative,
                is_dir: true,
                children,
                asset_type: None,
            }
        } else {
            let asset_type = get_asset_type(path);
            AssetTree {
                name,
                path: relative,
                is_dir: false,
                children: vec![],
                asset_type,
            }
        }
    }

    Ok(build_tree(base, base))
}

#[tauri::command]
pub fn get_asset_info(base_dir: &str, relative_path: &str) -> Result<AssetInfo, String> {
    let base = Path::new(base_dir);
    let full_path = base.join(relative_path);

    if !full_path.exists() {
        return Err("Asset not found".to_string());
    }

    let metadata =
        fs::metadata(&full_path).map_err(|e| format!("Failed to read metadata: {}", e))?;

    let name = full_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let asset_type = get_asset_type(&full_path).unwrap_or_else(|| "unknown".to_string());

    let modified = metadata
        .modified()
        .map(|t| {
            t.duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })
        .unwrap_or(0);

    Ok(AssetInfo {
        path: full_path.to_string_lossy().to_string(),
        relative_path: relative_path.to_string(),
        name,
        asset_type,
        size: metadata.len(),
        modified,
    })
}

#[tauri::command]
pub fn find_asset_usages(scenario: ivy::scenario::Scenario, asset_path: &str) -> Vec<usize> {
    let mut usages = Vec::new();

    for (index, cmd) in scenario.script.iter().enumerate() {
        let matches = [
            cmd.background.as_deref(),
            cmd.character.as_deref(),
            cmd.bgm.as_deref(),
            cmd.se.as_deref(),
            cmd.voice.as_deref(),
        ]
        .iter()
        .any(|field| field == &Some(asset_path));

        if matches {
            usages.push(index);
        }
    }

    usages
}

#[tauri::command]
pub fn find_unused_assets(
    base_dir: &str,
    scenario: ivy::scenario::Scenario,
) -> Result<Vec<String>, String> {
    let base = Path::new(base_dir);

    let mut used_assets: HashSet<String> = HashSet::new();

    for cmd in &scenario.script {
        if let Some(ref bg) = cmd.background {
            if !bg.is_empty() {
                used_assets.insert(bg.clone());
            }
        }
        if let Some(ref ch) = cmd.character {
            if !ch.is_empty() {
                used_assets.insert(ch.clone());
            }
        }
        if let Some(ref bgm) = cmd.bgm {
            if !bgm.is_empty() {
                used_assets.insert(bgm.clone());
            }
        }
        if let Some(ref se) = cmd.se {
            if !se.is_empty() {
                used_assets.insert(se.clone());
            }
        }
        if let Some(ref voice) = cmd.voice {
            if !voice.is_empty() {
                used_assets.insert(voice.clone());
            }
        }
    }

    let mut unused = Vec::new();

    for entry in WalkDir::new(base).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && get_asset_type(path).is_some() {
            if let Ok(relative) = path.strip_prefix(base) {
                let relative_str = relative.to_string_lossy().to_string();
                if !used_assets.contains(&relative_str) {
                    unused.push(relative_str);
                }
            }
        }
    }

    Ok(unused)
}
