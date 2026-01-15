# Phase 5.3: Preview Integration

Real-time preview within the editor (fully integrated).

## Goal

Users can see how the scenario looks while editing, with editor and preview synchronized.

## Prerequisites

- Phase 5.1 completed (Tauri project, IPC commands)

## Tasks

### 1. PreviewState Type

Reuse the existing `PreviewState` structure from `src/bin/preview.rs`.

```rust
// src/commands/preview.rs
use crate::scenario::{Command, Scenario};
use crate::i18n::LocalizedString;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Serialize)]
pub struct PreviewState {
    pub title: String,
    pub command_index: usize,
    pub total_commands: usize,
    pub text: Option<String>,
    pub speaker: Option<String>,
    pub background: Option<String>,
    pub character: Option<String>,
    pub char_pos: Option<String>,
    pub choices: Vec<ChoiceInfo>,
    pub variables: HashMap<String, String>,
    pub labels: Vec<String>,
    pub current_label: Option<String>,
    pub nvl_mode: bool,
}

#[derive(Clone, Serialize)]
pub struct ChoiceInfo {
    pub label: String,
    pub jump: String,
}

fn resolve_localized(s: &LocalizedString) -> String {
    match s {
        LocalizedString::Plain(text) => text.clone(),
        LocalizedString::Localized(map) => {
            map.get("en").or_else(|| map.values().next()).cloned().unwrap_or_default()
        }
        LocalizedString::Key(key) => format!("@{}", key),
    }
}

#[tauri::command]
pub fn get_preview_state(
    scenario: Scenario,
    index: usize,
    variables: HashMap<String, String>,
) -> PreviewState {
    let total = scenario.script.len();
    let idx = index.min(total.saturating_sub(1));

    // Collect all labels
    let labels: Vec<String> = scenario.script
        .iter()
        .filter_map(|cmd| cmd.label.clone())
        .collect();

    // Find current label (scan backwards from index)
    let current_label = scenario.script
        .iter()
        .take(idx + 1)
        .rev()
        .find_map(|cmd| cmd.label.clone());

    // Reconstruct visual state by scanning from start
    let mut background: Option<String> = None;
    let mut character: Option<String> = None;
    let mut char_pos: Option<String> = None;
    let mut nvl_mode = false;

    for cmd in scenario.script.iter().take(idx + 1) {
        if let Some(ref bg) = cmd.background {
            background = if bg.is_empty() { None } else { Some(bg.clone()) };
        }
        if let Some(ref ch) = cmd.character {
            character = if ch.is_empty() { None } else { Some(ch.clone()) };
        }
        if let Some(ref pos) = cmd.char_pos {
            char_pos = Some(format!("{:?}", pos).to_lowercase());
        }
        if let Some(nvl) = cmd.nvl {
            nvl_mode = nvl;
        }
    }

    // Current command info
    let current_cmd = scenario.script.get(idx);
    let text = current_cmd.and_then(|c| c.text.as_ref()).map(resolve_localized);
    let speaker = current_cmd.and_then(|c| c.speaker.as_ref()).map(resolve_localized);

    let choices = current_cmd
        .and_then(|c| c.choices.as_ref())
        .map(|choices| {
            choices.iter().map(|c| ChoiceInfo {
                label: resolve_localized(&c.label),
                jump: c.jump.clone(),
            }).collect()
        })
        .unwrap_or_default();

    PreviewState {
        title: scenario.title.clone(),
        command_index: idx,
        total_commands: total,
        text,
        speaker,
        background,
        character,
        char_pos,
        choices,
        variables,
        labels,
        current_label,
        nvl_mode,
    }
}
```

### 2. Asset Path Resolution

```rust
// src/commands/assets.rs
use std::path::{Path, PathBuf};
use tauri::AppHandle;

#[tauri::command]
pub fn resolve_asset_url(
    app: AppHandle,
    base_dir: &str,
    asset_path: &str,
) -> Result<String, String> {
    if asset_path.is_empty() {
        return Ok(String::new());
    }

    let base = Path::new(base_dir);
    let full_path = base.join(asset_path);

    if !full_path.exists() {
        return Err(format!("Asset not found: {}", full_path.display()));
    }

    // Convert to asset protocol URL for Tauri
    Ok(format!("asset://localhost/{}", full_path.display()))
}

#[tauri::command]
pub fn read_asset_base64(
    base_dir: &str,
    asset_path: &str,
) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    use std::fs;

    let base = Path::new(base_dir);
    let full_path = base.join(asset_path);

    let data = fs::read(&full_path)
        .map_err(|e| format!("Failed to read asset: {}", e))?;

    let mime = match full_path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    };

    Ok(format!("data:{};base64,{}", mime, STANDARD.encode(&data)))
}
```

### 3. Register Commands

```rust
// src/main.rs
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // ... existing commands
            commands::preview::get_preview_state,
            commands::assets::resolve_asset_url,
            commands::assets::read_asset_base64,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
```

### 4. usePreview Hook

```typescript
// ui/src/hooks/usePreview.ts
import { useState, useCallback, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Scenario } from '../types/scenario';

interface PreviewState {
  title: string;
  command_index: number;
  total_commands: number;
  text: string | null;
  speaker: string | null;
  background: string | null;
  character: string | null;
  char_pos: string | null;
  choices: { label: string; jump: string }[];
  variables: Record<string, string>;
  labels: string[];
  current_label: string | null;
  nvl_mode: boolean;
}

interface UsePreviewReturn {
  state: PreviewState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  goto: (index: number) => void;
  next: () => void;
  prev: () => void;
}

export function usePreview(
  scenario: Scenario | null,
  baseDir: string | null,
  selectedIndex: number | null
): UsePreviewReturn {
  const [state, setState] = useState<PreviewState | null>(null);
  const [backgroundUrl, setBackgroundUrl] = useState<string | null>(null);
  const [characterUrl, setCharacterUrl] = useState<string | null>(null);
  const [index, setIndex] = useState(0);

  // Sync with selected index from editor
  useEffect(() => {
    if (selectedIndex !== null) {
      setIndex(selectedIndex);
    }
  }, [selectedIndex]);

  // Fetch preview state when scenario or index changes
  useEffect(() => {
    if (!scenario) {
      setState(null);
      return;
    }

    const fetchState = async () => {
      const previewState = await invoke<PreviewState>('get_preview_state', {
        scenario,
        index,
        variables: {},
      });
      setState(previewState);
    };

    fetchState();
  }, [scenario, index]);

  // Load assets when state changes
  useEffect(() => {
    if (!state || !baseDir) {
      setBackgroundUrl(null);
      setCharacterUrl(null);
      return;
    }

    const loadAssets = async () => {
      if (state.background) {
        try {
          const url = await invoke<string>('read_asset_base64', {
            baseDir,
            assetPath: state.background,
          });
          setBackgroundUrl(url);
        } catch {
          setBackgroundUrl(null);
        }
      } else {
        setBackgroundUrl(null);
      }

      if (state.character) {
        try {
          const url = await invoke<string>('read_asset_base64', {
            baseDir,
            assetPath: state.character,
          });
          setCharacterUrl(url);
        } catch {
          setCharacterUrl(null);
        }
      } else {
        setCharacterUrl(null);
      }
    };

    loadAssets();
  }, [state?.background, state?.character, baseDir]);

  const goto = useCallback((newIndex: number) => {
    if (!state) return;
    const clamped = Math.max(0, Math.min(newIndex, state.total_commands - 1));
    setIndex(clamped);
  }, [state]);

  const next = useCallback(() => {
    goto(index + 1);
  }, [index, goto]);

  const prev = useCallback(() => {
    goto(index - 1);
  }, [index, goto]);

  return {
    state,
    backgroundUrl,
    characterUrl,
    goto,
    next,
    prev,
  };
}
```

### 5. PreviewPanel Component

```typescript
// ui/src/components/PreviewPanel/index.tsx
import { FC } from 'react';
import { SceneView } from './SceneView';
import { TextBox } from './TextBox';
import { ChoiceButtons } from './ChoiceButtons';
import { Controls } from './Controls';
import './styles.css';

interface PreviewState {
  title: string;
  command_index: number;
  total_commands: number;
  text: string | null;
  speaker: string | null;
  background: string | null;
  character: string | null;
  char_pos: string | null;
  choices: { label: string; jump: string }[];
  nvl_mode: boolean;
}

interface Props {
  state: PreviewState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  onPrev: () => void;
  onNext: () => void;
  onGoto: (index: number) => void;
}

export const PreviewPanel: FC<Props> = ({
  state,
  backgroundUrl,
  characterUrl,
  onPrev,
  onNext,
  onGoto,
}) => {
  if (!state) {
    return (
      <div className="preview-panel empty">
        <p>Load a scenario to see preview</p>
      </div>
    );
  }

  return (
    <div className={`preview-panel ${state.nvl_mode ? 'nvl-mode' : 'adv-mode'}`}>
      <SceneView
        backgroundUrl={backgroundUrl}
        characterUrl={characterUrl}
        charPos={state.char_pos}
      />

      {state.nvl_mode ? (
        <div className="nvl-text-area">
          {state.speaker && <span className="nvl-speaker">{state.speaker}: </span>}
          {state.text}
        </div>
      ) : (
        <TextBox
          speaker={state.speaker}
          text={state.text}
        />
      )}

      {state.choices.length > 0 && (
        <ChoiceButtons choices={state.choices} />
      )}

      <Controls
        currentIndex={state.command_index}
        totalCommands={state.total_commands}
        onPrev={onPrev}
        onNext={onNext}
        onGoto={onGoto}
      />
    </div>
  );
};
```

### 6. SceneView Component

```typescript
// ui/src/components/PreviewPanel/SceneView.tsx
import { FC } from 'react';

interface Props {
  backgroundUrl: string | null;
  characterUrl: string | null;
  charPos: string | null;
}

export const SceneView: FC<Props> = ({ backgroundUrl, characterUrl, charPos }) => {
  const getCharacterStyle = (): React.CSSProperties => {
    const baseStyle: React.CSSProperties = {
      position: 'absolute',
      bottom: 0,
      maxHeight: '80%',
      objectFit: 'contain',
    };

    switch (charPos) {
      case 'left':
        return { ...baseStyle, left: '10%' };
      case 'right':
        return { ...baseStyle, right: '10%' };
      case 'center':
      default:
        return { ...baseStyle, left: '50%', transform: 'translateX(-50%)' };
    }
  };

  return (
    <div className="scene-view">
      {backgroundUrl && (
        <div
          className="background-layer"
          style={{ backgroundImage: `url(${backgroundUrl})` }}
        />
      )}
      {characterUrl && (
        <img
          className="character-layer"
          src={characterUrl}
          alt="Character"
          style={getCharacterStyle()}
        />
      )}
    </div>
  );
};
```

### 7. TextBox Component

```typescript
// ui/src/components/PreviewPanel/TextBox.tsx
import { FC } from 'react';

interface Props {
  speaker: string | null;
  text: string | null;
}

export const TextBox: FC<Props> = ({ speaker, text }) => {
  return (
    <div className="text-box">
      {speaker && <div className="speaker-name">{speaker}</div>}
      <div className="dialogue-text">{text ?? ''}</div>
    </div>
  );
};
```

### 8. ChoiceButtons Component

```typescript
// ui/src/components/PreviewPanel/ChoiceButtons.tsx
import { FC } from 'react';

interface Choice {
  label: string;
  jump: string;
}

interface Props {
  choices: Choice[];
}

export const ChoiceButtons: FC<Props> = ({ choices }) => {
  return (
    <div className="choice-buttons">
      {choices.map((choice, index) => (
        <button key={index} className="choice-button" disabled>
          {choice.label}
        </button>
      ))}
    </div>
  );
};
```

### 9. Controls Component

```typescript
// ui/src/components/PreviewPanel/Controls.tsx
import { FC } from 'react';

interface Props {
  currentIndex: number;
  totalCommands: number;
  onPrev: () => void;
  onNext: () => void;
  onGoto: (index: number) => void;
}

export const Controls: FC<Props> = ({
  currentIndex,
  totalCommands,
  onPrev,
  onNext,
  onGoto,
}) => {
  return (
    <div className="preview-controls">
      <button onClick={onPrev} disabled={currentIndex <= 0}>
        ◀ Prev
      </button>
      <span className="position-indicator">
        {currentIndex + 1} / {totalCommands}
      </span>
      <button onClick={onNext} disabled={currentIndex >= totalCommands - 1}>
        Next ▶
      </button>
    </div>
  );
};
```

### 10. Styles

```css
/* ui/src/components/PreviewPanel/styles.css */
.preview-panel {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  background: #1a1a2e;
  border-radius: 8px;
  overflow: hidden;
}

.preview-panel.empty {
  display: flex;
  align-items: center;
  justify-content: center;
  color: #666;
}

/* Scene */
.scene-view {
  position: absolute;
  inset: 0;
}

.background-layer {
  position: absolute;
  inset: 0;
  background-size: cover;
  background-position: center;
}

.character-layer {
  position: absolute;
  bottom: 0;
  max-height: 80%;
  object-fit: contain;
}

/* ADV Mode Text Box */
.text-box {
  position: absolute;
  bottom: 10%;
  left: 5%;
  right: 5%;
  background: rgba(0, 0, 0, 0.8);
  border: 2px solid #4a4a6a;
  border-radius: 8px;
  padding: 15px 20px;
  color: white;
}

.speaker-name {
  font-weight: bold;
  color: #ffd700;
  margin-bottom: 8px;
}

.dialogue-text {
  line-height: 1.6;
  font-size: 14px;
}

/* NVL Mode */
.nvl-mode .scene-view {
  opacity: 0.3;
}

.nvl-text-area {
  position: absolute;
  inset: 10%;
  background: rgba(0, 0, 0, 0.9);
  color: white;
  padding: 20px;
  overflow-y: auto;
  line-height: 2;
}

.nvl-speaker {
  font-weight: bold;
  color: #ffd700;
}

/* Choices */
.choice-buttons {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.choice-button {
  background: rgba(0, 0, 0, 0.8);
  border: 2px solid #4a4a6a;
  color: white;
  padding: 12px 30px;
  border-radius: 4px;
  cursor: not-allowed;
  opacity: 0.8;
}

/* Controls */
.preview-controls {
  position: absolute;
  bottom: 5px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 10px;
  background: rgba(0, 0, 0, 0.7);
  padding: 5px 15px;
  border-radius: 20px;
}

.preview-controls button {
  background: transparent;
  border: none;
  color: white;
  cursor: pointer;
  padding: 5px 10px;
}

.preview-controls button:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.position-indicator {
  color: white;
  font-size: 12px;
}
```

### 11. Integration with App

```typescript
// ui/src/App.tsx (updated)
import { usePreview } from './hooks/usePreview';
import { PreviewPanel } from './components/PreviewPanel';

function App() {
  const {
    scenario,
    filePath,
    selectedIndex,
    selectCommand,
    /* ... */
  } = useScenario();

  const baseDir = filePath ? filePath.replace(/[^/\\]+$/, '') : null;

  const {
    state: previewState,
    backgroundUrl,
    characterUrl,
    goto: previewGoto,
    next: previewNext,
    prev: previewPrev,
  } = usePreview(scenario, baseDir, selectedIndex);

  // Sync: when preview navigates, update editor selection
  const handlePreviewGoto = (index: number) => {
    previewGoto(index);
    selectCommand(index);
  };

  return (
    <div className="app">
      {/* ... toolbar and sidebar */}

      <main className="main-content">
        {/* ... command list / flowchart */}

        <section className="preview-section">
          <PreviewPanel
            state={previewState}
            backgroundUrl={backgroundUrl}
            characterUrl={characterUrl}
            onPrev={previewPrev}
            onNext={previewNext}
            onGoto={handlePreviewGoto}
          />
        </section>

        {/* ... command form */}
      </main>
    </div>
  );
}
```

## Editor ↔ Preview Synchronization

| Action | Result |
|--------|--------|
| Click command in list | Preview jumps to that command |
| Click Prev/Next in preview | Editor selection updates |
| Edit command | Preview updates immediately |
| Add/Remove command | Preview re-renders |

## Keyboard Shortcuts

```typescript
// Add to App.tsx
useEffect(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'ArrowRight' && e.ctrlKey) {
      previewNext();
    } else if (e.key === 'ArrowLeft' && e.ctrlKey) {
      previewPrev();
    }
  };

  window.addEventListener('keydown', handleKeyDown);
  return () => window.removeEventListener('keydown', handleKeyDown);
}, [previewNext, previewPrev]);
```

## Verification

1. Open scenario with backgrounds and characters
2. Verify images display correctly
3. Navigate through commands → preview updates
4. Edit text → preview updates immediately
5. NVL mode displays differently from ADV mode
6. Choices display as buttons (disabled in editor)
