# Phase 5.1: Command Editor MVP

Edit scenarios without YAML knowledge.

## Goal

Users can create and edit ivy scenarios using only GUI forms.

## Tasks

### 1. Tauri Project Scaffold

Create `editors/ivy-editor/` with Tauri 2.x.

```bash
cd editors
npm create tauri-app@latest ivy-editor -- --template react-ts
cd ivy-editor
```

#### Project Structure

```
editors/ivy-editor/
├── Cargo.toml
├── tauri.conf.json
├── src/
│   ├── main.rs
│   └── commands/
│       ├── mod.rs
│       └── scenario.rs
└── ui/
    ├── package.json
    ├── vite.config.ts
    └── src/
        ├── App.tsx
        ├── components/
        │   ├── CommandList/
        │   │   ├── index.tsx
        │   │   └── CommandRow.tsx
        │   └── CommandForm/
        │       ├── index.tsx
        │       ├── BasicSection.tsx
        │       ├── VisualSection.tsx
        │       ├── AudioSection.tsx
        │       ├── FlowSection.tsx
        │       └── AdvancedSection.tsx
        ├── hooks/
        │   └── useScenario.ts
        └── types/
            └── scenario.ts
```

### 2. TypeScript Type Generation

Use `ts-rs` crate to generate TypeScript types from Rust.

#### Add ts-rs to Cargo.toml

```toml
# src/scenario/Cargo.toml or root Cargo.toml
[dependencies]
ts-rs = { version = "10", features = ["serde-compat"] }
```

#### Derive TS trait

```rust
// src/scenario/types.rs
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../editors/ivy-editor/ui/src/types/")]
pub struct Command {
    // ... existing fields
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../editors/ivy-editor/ui/src/types/")]
pub struct Scenario {
    // ... existing fields
}

// Add to all types used in Command:
// Choice, Transition, Shake, CameraCommand, etc.
```

#### Generate Types

```bash
cargo test export_bindings
# Or add a test that calls ts-rs export
```

### 3. Tauri IPC Commands

#### Rust Backend (src/commands/scenario.rs)

```rust
use crate::scenario::{parser, Scenario};
use crate::scenario::validator::{validate_scenario, ValidationResult};
use std::fs;
use std::path::Path;

#[tauri::command]
pub fn load_scenario(path: &str) -> Result<Scenario, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    parser::parse_scenario(&content)
        .map_err(|e| format!("Failed to parse scenario: {}", e))
}

#[tauri::command]
pub fn save_scenario(path: &str, scenario: Scenario) -> Result<(), String> {
    let yaml = serde_yaml::to_string(&scenario)
        .map_err(|e| format!("Failed to serialize: {}", e))?;

    fs::write(path, yaml)
        .map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub fn validate(scenario: Scenario) -> ValidationResult {
    validate_scenario(&scenario)
}

#[tauri::command]
pub fn scenario_to_yaml(scenario: Scenario) -> Result<String, String> {
    serde_yaml::to_string(&scenario)
        .map_err(|e| format!("Failed to serialize: {}", e))
}

#[tauri::command]
pub fn yaml_to_scenario(yaml: &str) -> Result<Scenario, String> {
    parser::parse_scenario(yaml)
        .map_err(|e| format!("Failed to parse: {}", e))
}
```

#### Register Commands (src/main.rs)

```rust
mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::scenario::load_scenario,
            commands::scenario::save_scenario,
            commands::scenario::validate,
            commands::scenario::scenario_to_yaml,
            commands::scenario::yaml_to_scenario,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
```

### 4. React Components

#### useScenario Hook

```typescript
// ui/src/hooks/useScenario.ts
import { useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { Scenario, Command, ValidationResult } from '../types/scenario';

interface UseScenarioReturn {
  scenario: Scenario | null;
  filePath: string | null;
  isDirty: boolean;
  validationResult: ValidationResult | null;
  selectedIndex: number | null;
  load: (path: string) => Promise<void>;
  save: () => Promise<void>;
  saveAs: (path: string) => Promise<void>;
  updateCommand: (index: number, command: Command) => void;
  addCommand: (index?: number) => void;
  removeCommand: (index: number) => void;
  selectCommand: (index: number | null) => void;
  validate: () => Promise<void>;
  getYaml: () => Promise<string>;
}

export function useScenario(): UseScenarioReturn {
  const [scenario, setScenario] = useState<Scenario | null>(null);
  const [filePath, setFilePath] = useState<string | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [validationResult, setValidationResult] = useState<ValidationResult | null>(null);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  const load = useCallback(async (path: string) => {
    const loaded = await invoke<Scenario>('load_scenario', { path });
    setScenario(loaded);
    setFilePath(path);
    setIsDirty(false);
    setSelectedIndex(null);
  }, []);

  const save = useCallback(async () => {
    if (!scenario || !filePath) return;
    await invoke('save_scenario', { path: filePath, scenario });
    setIsDirty(false);
  }, [scenario, filePath]);

  const saveAs = useCallback(async (path: string) => {
    if (!scenario) return;
    await invoke('save_scenario', { path, scenario });
    setFilePath(path);
    setIsDirty(false);
  }, [scenario]);

  const updateCommand = useCallback((index: number, command: Command) => {
    setScenario(prev => {
      if (!prev) return prev;
      const newScript = [...prev.script];
      newScript[index] = command;
      return { ...prev, script: newScript };
    });
    setIsDirty(true);
  }, []);

  const addCommand = useCallback((index?: number) => {
    setScenario(prev => {
      if (!prev) return prev;
      const newCommand: Command = { text: '' };
      const newScript = [...prev.script];
      const insertIndex = index ?? newScript.length;
      newScript.splice(insertIndex, 0, newCommand);
      return { ...prev, script: newScript };
    });
    setIsDirty(true);
  }, []);

  const removeCommand = useCallback((index: number) => {
    setScenario(prev => {
      if (!prev) return prev;
      const newScript = prev.script.filter((_, i) => i !== index);
      return { ...prev, script: newScript };
    });
    setIsDirty(true);
    setSelectedIndex(null);
  }, []);

  const selectCommand = useCallback((index: number | null) => {
    setSelectedIndex(index);
  }, []);

  const validate = useCallback(async () => {
    if (!scenario) return;
    const result = await invoke<ValidationResult>('validate', { scenario });
    setValidationResult(result);
  }, [scenario]);

  const getYaml = useCallback(async () => {
    if (!scenario) return '';
    return invoke<string>('scenario_to_yaml', { scenario });
  }, [scenario]);

  return {
    scenario,
    filePath,
    isDirty,
    validationResult,
    selectedIndex,
    load,
    save,
    saveAs,
    updateCommand,
    addCommand,
    removeCommand,
    selectCommand,
    validate,
    getYaml,
  };
}
```

#### CommandList Component

```typescript
// ui/src/components/CommandList/index.tsx
import { FC } from 'react';
import type { Command } from '../../types/scenario';
import { CommandRow } from './CommandRow';

interface Props {
  commands: Command[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
  onAdd: (index: number) => void;
  onRemove: (index: number) => void;
}

export const CommandList: FC<Props> = ({
  commands,
  selectedIndex,
  onSelect,
  onAdd,
  onRemove,
}) => {
  return (
    <div className="command-list">
      <div className="command-list-header">
        <span>#</span>
        <span>Type</span>
        <span>Preview</span>
        <span>Actions</span>
      </div>
      {commands.map((cmd, index) => (
        <CommandRow
          key={index}
          index={index}
          command={cmd}
          isSelected={index === selectedIndex}
          onSelect={() => onSelect(index)}
          onAddAfter={() => onAdd(index + 1)}
          onRemove={() => onRemove(index)}
        />
      ))}
      <button
        className="add-command-button"
        onClick={() => onAdd(commands.length)}
      >
        + Add Command
      </button>
    </div>
  );
};
```

#### CommandRow Component

```typescript
// ui/src/components/CommandList/CommandRow.tsx
import { FC } from 'react';
import type { Command } from '../../types/scenario';

interface Props {
  index: number;
  command: Command;
  isSelected: boolean;
  onSelect: () => void;
  onAddAfter: () => void;
  onRemove: () => void;
}

export const CommandRow: FC<Props> = ({
  index,
  command,
  isSelected,
  onSelect,
  onAddAfter,
  onRemove,
}) => {
  const getCommandType = (cmd: Command): string => {
    if (cmd.label) return 'label';
    if (cmd.choices) return 'choice';
    if (cmd.jump) return 'jump';
    if (cmd.if_cond) return 'condition';
    if (cmd.background) return 'background';
    if (cmd.character) return 'character';
    return 'text';
  };

  const getPreview = (cmd: Command): string => {
    if (cmd.label) return `[${cmd.label}]`;
    if (cmd.text) {
      const text = typeof cmd.text === 'string' ? cmd.text : cmd.text.en || '';
      return text.length > 40 ? text.slice(0, 40) + '...' : text;
    }
    if (cmd.jump) return `→ ${cmd.jump}`;
    return '-';
  };

  return (
    <div
      className={`command-row ${isSelected ? 'selected' : ''}`}
      onClick={onSelect}
    >
      <span className="index">{index}</span>
      <span className="type">{getCommandType(command)}</span>
      <span className="preview">{getPreview(command)}</span>
      <span className="actions">
        <button onClick={(e) => { e.stopPropagation(); onAddAfter(); }}>+</button>
        <button onClick={(e) => { e.stopPropagation(); onRemove(); }}>×</button>
      </span>
    </div>
  );
};
```

#### CommandForm Component

```typescript
// ui/src/components/CommandForm/index.tsx
import { FC, useState } from 'react';
import type { Command } from '../../types/scenario';
import { BasicSection } from './BasicSection';
import { VisualSection } from './VisualSection';
import { AudioSection } from './AudioSection';
import { FlowSection } from './FlowSection';
import { AdvancedSection } from './AdvancedSection';

interface Props {
  command: Command;
  labels: string[];
  onChange: (command: Command) => void;
}

export const CommandForm: FC<Props> = ({ command, labels, onChange }) => {
  const [showAdvanced, setShowAdvanced] = useState(false);

  const updateField = <K extends keyof Command>(
    field: K,
    value: Command[K]
  ) => {
    onChange({ ...command, [field]: value });
  };

  return (
    <div className="command-form">
      <BasicSection
        speaker={command.speaker}
        text={command.text}
        label={command.label}
        onSpeakerChange={(v) => updateField('speaker', v)}
        onTextChange={(v) => updateField('text', v)}
        onLabelChange={(v) => updateField('label', v)}
      />

      <VisualSection
        background={command.background}
        character={command.character}
        charPos={command.char_pos}
        onBackgroundChange={(v) => updateField('background', v)}
        onCharacterChange={(v) => updateField('character', v)}
        onCharPosChange={(v) => updateField('char_pos', v)}
      />

      <AudioSection
        bgm={command.bgm}
        se={command.se}
        voice={command.voice}
        onBgmChange={(v) => updateField('bgm', v)}
        onSeChange={(v) => updateField('se', v)}
        onVoiceChange={(v) => updateField('voice', v)}
      />

      <FlowSection
        jump={command.jump}
        choices={command.choices}
        labels={labels}
        onJumpChange={(v) => updateField('jump', v)}
        onChoicesChange={(v) => updateField('choices', v)}
      />

      <button
        className="advanced-toggle"
        onClick={() => setShowAdvanced(!showAdvanced)}
      >
        {showAdvanced ? '▼ Advanced' : '▶ Advanced'}
      </button>

      {showAdvanced && (
        <AdvancedSection
          command={command}
          onChange={onChange}
        />
      )}
    </div>
  );
};
```

#### BasicSection Component

```typescript
// ui/src/components/CommandForm/BasicSection.tsx
import { FC } from 'react';

interface Props {
  speaker?: string;
  text?: string;
  label?: string;
  onSpeakerChange: (value: string | undefined) => void;
  onTextChange: (value: string | undefined) => void;
  onLabelChange: (value: string | undefined) => void;
}

export const BasicSection: FC<Props> = ({
  speaker,
  text,
  label,
  onSpeakerChange,
  onTextChange,
  onLabelChange,
}) => {
  return (
    <section className="form-section">
      <h3>Basic</h3>

      <div className="form-field">
        <label htmlFor="label">Label</label>
        <input
          id="label"
          type="text"
          value={label ?? ''}
          onChange={(e) => onLabelChange(e.target.value || undefined)}
          placeholder="e.g., scene_start"
        />
      </div>

      <div className="form-field">
        <label htmlFor="speaker">Speaker</label>
        <input
          id="speaker"
          type="text"
          value={speaker ?? ''}
          onChange={(e) => onSpeakerChange(e.target.value || undefined)}
          placeholder="e.g., Alice"
        />
      </div>

      <div className="form-field">
        <label htmlFor="text">Text</label>
        <textarea
          id="text"
          rows={4}
          value={text ?? ''}
          onChange={(e) => onTextChange(e.target.value || undefined)}
          placeholder="Enter dialogue text..."
        />
      </div>
    </section>
  );
};
```

### 5. File Dialog Integration

```typescript
// ui/src/App.tsx
import { useState, useEffect } from 'react';
import { open, save } from '@tauri-apps/plugin-dialog';
import { useScenario } from './hooks/useScenario';
import { CommandList } from './components/CommandList';
import { CommandForm } from './components/CommandForm';

function App() {
  const {
    scenario,
    filePath,
    isDirty,
    validationResult,
    selectedIndex,
    load,
    save: saveScenario,
    saveAs,
    updateCommand,
    addCommand,
    removeCommand,
    selectCommand,
    validate,
    getYaml,
  } = useScenario();

  const [yamlPreview, setYamlPreview] = useState('');

  useEffect(() => {
    if (scenario) {
      getYaml().then(setYamlPreview);
    }
  }, [scenario, getYaml]);

  const handleOpen = async () => {
    const path = await open({
      filters: [{ name: 'YAML', extensions: ['yaml', 'yml'] }],
    });
    if (path) {
      await load(path as string);
    }
  };

  const handleSave = async () => {
    if (filePath) {
      await saveScenario();
    } else {
      await handleSaveAs();
    }
  };

  const handleSaveAs = async () => {
    const path = await save({
      filters: [{ name: 'YAML', extensions: ['yaml', 'yml'] }],
    });
    if (path) {
      await saveAs(path);
    }
  };

  const labels = scenario?.script
    .map((cmd) => cmd.label)
    .filter((label): label is string => !!label) ?? [];

  const selectedCommand = selectedIndex !== null && scenario
    ? scenario.script[selectedIndex]
    : null;

  return (
    <div className="app">
      <header className="toolbar">
        <button onClick={handleOpen}>Open</button>
        <button onClick={handleSave} disabled={!isDirty}>
          Save{isDirty ? '*' : ''}
        </button>
        <button onClick={handleSaveAs}>Save As</button>
        <button onClick={validate}>Validate</button>
        <span className="file-path">{filePath ?? 'Untitled'}</span>
      </header>

      <main className="main-content">
        <aside className="sidebar">
          {scenario && (
            <CommandList
              commands={scenario.script}
              selectedIndex={selectedIndex}
              onSelect={selectCommand}
              onAdd={addCommand}
              onRemove={removeCommand}
            />
          )}
        </aside>

        <section className="editor">
          {selectedCommand && (
            <CommandForm
              command={selectedCommand}
              labels={labels}
              onChange={(cmd) => updateCommand(selectedIndex!, cmd)}
            />
          )}
        </section>

        <aside className="yaml-preview">
          <h3>YAML Preview</h3>
          <pre>{yamlPreview}</pre>
        </aside>
      </main>

      {validationResult && (
        <footer className="validation-panel">
          {validationResult.errors.map((err, i) => (
            <div key={i} className="error">{err.message}</div>
          ))}
          {validationResult.warnings.map((warn, i) => (
            <div key={i} className="warning">{warn.message}</div>
          ))}
        </footer>
      )}
    </div>
  );
}

export default App;
```

### 6. Tauri Configuration

```json
// tauri.conf.json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "ivy-editor",
  "version": "0.1.0",
  "identifier": "com.ivy.editor",
  "build": {
    "frontendDist": "../ui/dist"
  },
  "app": {
    "windows": [
      {
        "title": "ivy Editor",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "plugins": {
    "dialog": {}
  }
}
```

## Testing

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_scenario() {
        let yaml = r#"
title: Test
script:
  - text: "Hello"
"#;
        let result = yaml_to_scenario(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_scenario_roundtrip() {
        let scenario = Scenario {
            title: "Test".to_string(),
            script: vec![Command { text: Some("Hello".to_string()), ..Default::default() }],
            ..Default::default()
        };
        let yaml = scenario_to_yaml(scenario.clone()).unwrap();
        let loaded = yaml_to_scenario(&yaml).unwrap();
        assert_eq!(loaded.title, scenario.title);
    }
}
```

### E2E Tests (Playwright)

```typescript
// tests/e2e/editor.spec.ts
import { test, expect } from '@playwright/test';

test('can create new scenario', async ({ page }) => {
  await page.goto('/');
  await page.click('text=Add Command');
  await page.fill('#text', 'Hello, World!');
  await expect(page.locator('.yaml-preview')).toContainText('Hello, World!');
});

test('can open and edit existing scenario', async ({ page }) => {
  // Test with mock file dialog
});
```

## Verification

1. **Build**: `cd editors/ivy-editor && cargo tauri build`
2. **Run**: `cd editors/ivy-editor && cargo tauri dev`
3. **Test**: Create new scenario, add commands, save as YAML, verify with `ivy-validate`
