# Phase 5.5: Asset Management

Browse, preview, and manage game assets.

## Goal

Users can easily manage images and audio files with visual preview and usage tracking.

## Prerequisites

- Phase 5.1 completed (Tauri project, file I/O)
- Phase 5.3 completed (Asset loading)

## Tasks

### 1. Tauri Commands for Asset Operations

```rust
// src/commands/assets.rs (expand)
use std::path::{Path, PathBuf};
use std::fs;
use serde::Serialize;
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

#[tauri::command]
pub fn list_assets(base_dir: &str) -> Result<AssetTree, String> {
    let base = Path::new(base_dir);
    if !base.exists() {
        return Err("Directory not found".to_string());
    }

    fn build_tree(path: &Path, base: &Path) -> AssetTree {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let relative = path.strip_prefix(base)
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

            children.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            });

            AssetTree {
                name,
                path: relative,
                is_dir: true,
                children,
                asset_type: None,
            }
        } else {
            let asset_type = get_asset_type(&path);
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
pub fn get_asset_info(base_dir: &str, relative_path: &str) -> Result<AssetInfo, String> {
    let base = Path::new(base_dir);
    let full_path = base.join(relative_path);

    if !full_path.exists() {
        return Err("Asset not found".to_string());
    }

    let metadata = fs::metadata(&full_path)
        .map_err(|e| format!("Failed to read metadata: {}", e))?;

    let name = full_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let asset_type = get_asset_type(&full_path)
        .unwrap_or_else(|| "unknown".to_string());

    let modified = metadata.modified()
        .map(|t| t.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())
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
pub fn find_asset_usages(
    scenario: crate::scenario::Scenario,
    asset_path: &str,
) -> Vec<usize> {
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
    scenario: crate::scenario::Scenario,
) -> Result<Vec<String>, String> {
    let base = Path::new(base_dir);

    // Collect all asset paths from scenario
    let mut used_assets: std::collections::HashSet<String> = std::collections::HashSet::new();

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

    // Find all assets in directory
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
```

### 2. Add walkdir Dependency

```toml
# Cargo.toml
[dependencies]
walkdir = "2"
```

### 3. AssetBrowser Component

```typescript
// ui/src/components/AssetBrowser/index.tsx
import { FC, useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { AssetTree } from './AssetTree';
import { AssetPreview } from './AssetPreview';
import { AssetInfo } from './AssetInfo';
import './styles.css';

interface AssetTreeData {
  name: string;
  path: string;
  is_dir: boolean;
  children: AssetTreeData[];
  asset_type: string | null;
}

interface AssetInfoData {
  path: string;
  relative_path: string;
  name: string;
  asset_type: string;
  size: number;
  modified: number;
}

interface Props {
  baseDir: string | null;
  onSelectAsset: (path: string) => void;
  onShowUsages: (indices: number[]) => void;
}

export const AssetBrowser: FC<Props> = ({
  baseDir,
  onSelectAsset,
  onShowUsages,
}) => {
  const [tree, setTree] = useState<AssetTreeData | null>(null);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [assetInfo, setAssetInfo] = useState<AssetInfoData | null>(null);
  const [unusedAssets, setUnusedAssets] = useState<string[]>([]);
  const [showUnusedOnly, setShowUnusedOnly] = useState(false);

  useEffect(() => {
    if (!baseDir) {
      setTree(null);
      return;
    }

    const loadTree = async () => {
      try {
        const data = await invoke<AssetTreeData>('list_assets', { baseDir });
        setTree(data);
      } catch (err) {
        console.error('Failed to load assets:', err);
      }
    };

    loadTree();
  }, [baseDir]);

  const handleSelect = useCallback(async (path: string) => {
    setSelectedPath(path);

    if (baseDir) {
      try {
        const info = await invoke<AssetInfoData>('get_asset_info', {
          baseDir,
          relativePath: path,
        });
        setAssetInfo(info);
      } catch (err) {
        setAssetInfo(null);
      }
    }
  }, [baseDir]);

  const handleUse = useCallback(() => {
    if (selectedPath) {
      onSelectAsset(selectedPath);
    }
  }, [selectedPath, onSelectAsset]);

  const handleFindUsages = useCallback(async () => {
    if (!selectedPath || !baseDir) return;

    try {
      // Need to get current scenario from parent
      // This would be passed as prop or via context
      const usages = await invoke<number[]>('find_asset_usages', {
        scenario: {}, // Would be actual scenario
        assetPath: selectedPath,
      });
      onShowUsages(usages);
    } catch (err) {
      console.error('Failed to find usages:', err);
    }
  }, [selectedPath, baseDir, onShowUsages]);

  const handleFindUnused = useCallback(async () => {
    if (!baseDir) return;

    try {
      const unused = await invoke<string[]>('find_unused_assets', {
        baseDir,
        scenario: {}, // Would be actual scenario
      });
      setUnusedAssets(unused);
      setShowUnusedOnly(true);
    } catch (err) {
      console.error('Failed to find unused assets:', err);
    }
  }, [baseDir]);

  const handleRefresh = useCallback(async () => {
    if (!baseDir) return;

    try {
      const data = await invoke<AssetTreeData>('list_assets', { baseDir });
      setTree(data);
    } catch (err) {
      console.error('Failed to refresh:', err);
    }
  }, [baseDir]);

  if (!baseDir) {
    return (
      <div className="asset-browser empty">
        <p>Open a scenario to browse assets</p>
      </div>
    );
  }

  return (
    <div className="asset-browser">
      <div className="asset-browser-toolbar">
        <button onClick={handleRefresh}>Refresh</button>
        <button onClick={handleFindUnused}>Find Unused</button>
        {showUnusedOnly && (
          <button onClick={() => setShowUnusedOnly(false)}>Show All</button>
        )}
      </div>

      <div className="asset-browser-content">
        <div className="asset-tree-panel">
          {tree && (
            <AssetTree
              node={tree}
              selectedPath={selectedPath}
              unusedAssets={showUnusedOnly ? unusedAssets : []}
              onSelect={handleSelect}
            />
          )}
        </div>

        <div className="asset-detail-panel">
          {selectedPath && assetInfo && (
            <>
              <AssetPreview
                baseDir={baseDir}
                path={selectedPath}
                type={assetInfo.asset_type}
              />
              <AssetInfo
                info={assetInfo}
                onUse={handleUse}
                onFindUsages={handleFindUsages}
              />
            </>
          )}
        </div>
      </div>
    </div>
  );
};
```

### 4. AssetTree Component

```typescript
// ui/src/components/AssetBrowser/AssetTree.tsx
import { FC, useState } from 'react';

interface AssetTreeNode {
  name: string;
  path: string;
  is_dir: boolean;
  children: AssetTreeNode[];
  asset_type: string | null;
}

interface Props {
  node: AssetTreeNode;
  selectedPath: string | null;
  unusedAssets: string[];
  onSelect: (path: string) => void;
  depth?: number;
}

export const AssetTree: FC<Props> = ({
  node,
  selectedPath,
  unusedAssets,
  onSelect,
  depth = 0,
}) => {
  const [expanded, setExpanded] = useState(depth < 2);

  const isUnused = unusedAssets.includes(node.path);
  const showNode = unusedAssets.length === 0 || isUnused || node.is_dir;

  if (!showNode) return null;

  const getIcon = () => {
    if (node.is_dir) return expanded ? '📂' : '📁';
    switch (node.asset_type) {
      case 'image': return '🖼️';
      case 'audio': return '🔊';
      case 'video': return '🎬';
      case 'scenario': return '📄';
      default: return '📎';
    }
  };

  if (node.is_dir) {
    const visibleChildren = node.children.filter(child => {
      if (unusedAssets.length === 0) return true;
      if (child.is_dir) return true;
      return unusedAssets.includes(child.path);
    });

    return (
      <div className="tree-node">
        <div
          className="tree-item folder"
          style={{ paddingLeft: depth * 16 }}
          onClick={() => setExpanded(!expanded)}
        >
          <span className="icon">{getIcon()}</span>
          <span className="name">{node.name}</span>
        </div>
        {expanded && (
          <div className="tree-children">
            {visibleChildren.map((child) => (
              <AssetTree
                key={child.path}
                node={child}
                selectedPath={selectedPath}
                unusedAssets={unusedAssets}
                onSelect={onSelect}
                depth={depth + 1}
              />
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <div
      className={`tree-item file ${selectedPath === node.path ? 'selected' : ''} ${isUnused ? 'unused' : ''}`}
      style={{ paddingLeft: depth * 16 }}
      onClick={() => onSelect(node.path)}
    >
      <span className="icon">{getIcon()}</span>
      <span className="name">{node.name}</span>
      {isUnused && <span className="unused-badge">unused</span>}
    </div>
  );
};
```

### 5. AssetPreview Component

```typescript
// ui/src/components/AssetBrowser/AssetPreview.tsx
import { FC, useState, useRef, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  baseDir: string;
  path: string;
  type: string;
}

export const AssetPreview: FC<Props> = ({ baseDir, path, type }) => {
  const [dataUrl, setDataUrl] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    const loadAsset = async () => {
      if (type === 'image' || type === 'audio') {
        try {
          const url = await invoke<string>('read_asset_base64', {
            baseDir,
            assetPath: path,
          });
          setDataUrl(url);
        } catch (err) {
          setDataUrl(null);
        }
      }
    };

    loadAsset();
    setIsPlaying(false);
  }, [baseDir, path, type]);

  const toggleAudio = () => {
    if (!audioRef.current) return;

    if (isPlaying) {
      audioRef.current.pause();
      audioRef.current.currentTime = 0;
    } else {
      audioRef.current.play();
    }
    setIsPlaying(!isPlaying);
  };

  if (type === 'image' && dataUrl) {
    return (
      <div className="asset-preview image-preview">
        <img src={dataUrl} alt={path} />
      </div>
    );
  }

  if (type === 'audio' && dataUrl) {
    return (
      <div className="asset-preview audio-preview">
        <audio
          ref={audioRef}
          src={dataUrl}
          onEnded={() => setIsPlaying(false)}
        />
        <button className="play-button" onClick={toggleAudio}>
          {isPlaying ? '⏹️ Stop' : '▶️ Play'}
        </button>
        <div className="waveform-placeholder">
          🎵 {path}
        </div>
      </div>
    );
  }

  return (
    <div className="asset-preview no-preview">
      <p>Preview not available</p>
    </div>
  );
};
```

### 6. AssetInfo Component

```typescript
// ui/src/components/AssetBrowser/AssetInfo.tsx
import { FC } from 'react';

interface AssetInfoData {
  path: string;
  relative_path: string;
  name: string;
  asset_type: string;
  size: number;
  modified: number;
}

interface Props {
  info: AssetInfoData;
  onUse: () => void;
  onFindUsages: () => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleDateString();
}

export const AssetInfo: FC<Props> = ({ info, onUse, onFindUsages }) => {
  return (
    <div className="asset-info">
      <h4>{info.name}</h4>
      <dl>
        <dt>Type</dt>
        <dd>{info.asset_type}</dd>
        <dt>Size</dt>
        <dd>{formatSize(info.size)}</dd>
        <dt>Modified</dt>
        <dd>{formatDate(info.modified)}</dd>
        <dt>Path</dt>
        <dd className="path">{info.relative_path}</dd>
      </dl>
      <div className="asset-actions">
        <button onClick={onUse}>Use in Command</button>
        <button onClick={onFindUsages}>Find Usages</button>
      </div>
    </div>
  );
};
```

### 7. Styles

```css
/* ui/src/components/AssetBrowser/styles.css */
.asset-browser {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.asset-browser.empty {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
}

.asset-browser-toolbar {
  display: flex;
  gap: 8px;
  padding: 8px;
  border-bottom: 1px solid #333;
}

.asset-browser-toolbar button {
  padding: 4px 12px;
  background: #333;
  border: 1px solid #555;
  color: #fff;
  border-radius: 4px;
  cursor: pointer;
}

.asset-browser-toolbar button:hover {
  background: #444;
}

.asset-browser-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.asset-tree-panel {
  width: 250px;
  overflow-y: auto;
  border-right: 1px solid #333;
  padding: 8px;
}

.asset-detail-panel {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
}

/* Tree */
.tree-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  cursor: pointer;
  border-radius: 4px;
  font-size: 13px;
}

.tree-item:hover {
  background: #2a2a3e;
}

.tree-item.selected {
  background: #3b82f6;
}

.tree-item.unused {
  opacity: 0.6;
}

.tree-item .icon {
  font-size: 14px;
}

.tree-item .name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.unused-badge {
  font-size: 10px;
  background: #f59e0b;
  color: #000;
  padding: 1px 4px;
  border-radius: 3px;
}

/* Preview */
.asset-preview {
  margin-bottom: 16px;
  background: #1a1a2e;
  border-radius: 8px;
  overflow: hidden;
}

.image-preview img {
  max-width: 100%;
  max-height: 300px;
  display: block;
  margin: 0 auto;
}

.audio-preview {
  padding: 20px;
  text-align: center;
}

.play-button {
  padding: 10px 20px;
  font-size: 16px;
  background: #3b82f6;
  border: none;
  color: white;
  border-radius: 8px;
  cursor: pointer;
  margin-bottom: 10px;
}

.play-button:hover {
  background: #2563eb;
}

.waveform-placeholder {
  color: #666;
  font-size: 12px;
}

.no-preview {
  padding: 40px;
  text-align: center;
  color: #666;
}

/* Info */
.asset-info h4 {
  margin: 0 0 12px;
  font-size: 16px;
}

.asset-info dl {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 4px 12px;
  font-size: 13px;
}

.asset-info dt {
  color: #888;
}

.asset-info dd {
  margin: 0;
}

.asset-info dd.path {
  font-family: monospace;
  font-size: 11px;
  word-break: break-all;
}

.asset-actions {
  margin-top: 16px;
  display: flex;
  gap: 8px;
}

.asset-actions button {
  flex: 1;
  padding: 8px;
  background: #333;
  border: 1px solid #555;
  color: white;
  border-radius: 4px;
  cursor: pointer;
}

.asset-actions button:hover {
  background: #444;
}
```

### 8. Integration with App

```typescript
// ui/src/App.tsx (add AssetBrowser panel)
import { AssetBrowser } from './components/AssetBrowser';

function App() {
  // ...existing state

  const [activeTab, setActiveTab] = useState<'commands' | 'assets'>('commands');

  const handleSelectAsset = (path: string) => {
    // Insert asset path into current command's appropriate field
    // Based on asset type
  };

  const handleShowUsages = (indices: number[]) => {
    // Highlight commands that use the selected asset
    // Could show in sidebar or filter command list
  };

  return (
    <div className="app">
      <header className="toolbar">
        {/* ... */}
      </header>

      <main className="main-content">
        <aside className="sidebar">
          <div className="sidebar-tabs">
            <button
              className={activeTab === 'commands' ? 'active' : ''}
              onClick={() => setActiveTab('commands')}
            >
              Commands
            </button>
            <button
              className={activeTab === 'assets' ? 'active' : ''}
              onClick={() => setActiveTab('assets')}
            >
              Assets
            </button>
          </div>

          {activeTab === 'commands' && scenario && (
            <CommandList /* ... */ />
          )}

          {activeTab === 'assets' && (
            <AssetBrowser
              baseDir={baseDir}
              onSelectAsset={handleSelectAsset}
              onShowUsages={handleShowUsages}
            />
          )}
        </aside>

        {/* ... rest of layout */}
      </main>
    </div>
  );
}
```

## Features Summary

| Feature | Description |
|---------|-------------|
| File tree | Browse project assets hierarchically |
| Image preview | View images before using |
| Audio preview | Play/stop audio files |
| Asset info | Size, type, modification date |
| Find usages | Which commands use this asset |
| Find unused | List assets not referenced in scenario |
| Use in command | Insert asset path into form |

## Verification

1. Open scenario → asset tree shows project files
2. Click image → preview displays
3. Click audio → play button works
4. Click "Find Usages" → shows command indices
5. Click "Find Unused" → filters to unused assets
6. Click "Use in Command" → path inserted into form
