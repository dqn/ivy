# Phase 5.4: Drag & Drop

Intuitive command reordering and asset path input.

## Goal

Users can reorder commands by dragging and drop asset files to auto-fill paths.

## Prerequisites

- Phase 5.1 completed (Command list, form)
- Phase 5.3 completed (Asset path resolution)

## Tasks

### 1. Dependencies

```bash
cd editors/ivy-editor/ui
npm install @dnd-kit/core @dnd-kit/sortable @dnd-kit/utilities
```

### 2. Sortable Command List

```typescript
// ui/src/components/CommandList/index.tsx
import { FC, useCallback } from 'react';
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  DragEndEvent,
} from '@dnd-kit/core';
import {
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from '@dnd-kit/sortable';
import type { Command } from '../../types/scenario';
import { SortableCommandRow } from './SortableCommandRow';

interface Props {
  commands: Command[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
  onAdd: (index: number) => void;
  onRemove: (index: number) => void;
  onReorder: (fromIndex: number, toIndex: number) => void;
}

export const CommandList: FC<Props> = ({
  commands,
  selectedIndex,
  onSelect,
  onAdd,
  onRemove,
  onReorder,
}) => {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  const handleDragEnd = useCallback(
    (event: DragEndEvent) => {
      const { active, over } = event;

      if (over && active.id !== over.id) {
        const oldIndex = commands.findIndex((_, i) => `cmd-${i}` === active.id);
        const newIndex = commands.findIndex((_, i) => `cmd-${i}` === over.id);

        if (oldIndex !== -1 && newIndex !== -1) {
          onReorder(oldIndex, newIndex);
        }
      }
    },
    [commands, onReorder]
  );

  const items = commands.map((_, i) => `cmd-${i}`);

  return (
    <div className="command-list">
      <div className="command-list-header">
        <span></span>
        <span>#</span>
        <span>Type</span>
        <span>Preview</span>
        <span>Actions</span>
      </div>

      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragEnd={handleDragEnd}
      >
        <SortableContext items={items} strategy={verticalListSortingStrategy}>
          {commands.map((cmd, index) => (
            <SortableCommandRow
              key={`cmd-${index}`}
              id={`cmd-${index}`}
              index={index}
              command={cmd}
              isSelected={index === selectedIndex}
              onSelect={() => onSelect(index)}
              onAddAfter={() => onAdd(index + 1)}
              onRemove={() => onRemove(index)}
            />
          ))}
        </SortableContext>
      </DndContext>

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

### 3. SortableCommandRow Component

```typescript
// ui/src/components/CommandList/SortableCommandRow.tsx
import { FC } from 'react';
import { useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import type { Command } from '../../types/scenario';

interface Props {
  id: string;
  index: number;
  command: Command;
  isSelected: boolean;
  onSelect: () => void;
  onAddAfter: () => void;
  onRemove: () => void;
}

export const SortableCommandRow: FC<Props> = ({
  id,
  index,
  command,
  isSelected,
  onSelect,
  onAddAfter,
  onRemove,
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

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
      ref={setNodeRef}
      style={style}
      className={`command-row ${isSelected ? 'selected' : ''} ${isDragging ? 'dragging' : ''}`}
      onClick={onSelect}
    >
      <span className="drag-handle" {...attributes} {...listeners}>
        ⠿
      </span>
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

### 4. Reorder Handler in useScenario

```typescript
// ui/src/hooks/useScenario.ts (add reorder function)
const reorderCommand = useCallback((fromIndex: number, toIndex: number) => {
  setScenario(prev => {
    if (!prev) return prev;
    const newScript = [...prev.script];
    const [removed] = newScript.splice(fromIndex, 1);
    newScript.splice(toIndex, 0, removed);
    return { ...prev, script: newScript };
  });
  setIsDirty(true);

  // Update selection if needed
  if (selectedIndex === fromIndex) {
    setSelectedIndex(toIndex);
  } else if (selectedIndex !== null) {
    if (fromIndex < selectedIndex && toIndex >= selectedIndex) {
      setSelectedIndex(selectedIndex - 1);
    } else if (fromIndex > selectedIndex && toIndex <= selectedIndex) {
      setSelectedIndex(selectedIndex + 1);
    }
  }
}, [selectedIndex]);
```

### 5. Asset Drop Zone

```typescript
// ui/src/components/CommandForm/AssetField.tsx
import { FC, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

interface Props {
  label: string;
  value: string | undefined;
  baseDir: string | null;
  accept: string[];
  onChange: (value: string | undefined) => void;
}

export const AssetField: FC<Props> = ({
  label,
  value,
  baseDir,
  accept,
  onChange,
}) => {
  const handleDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const files = e.dataTransfer.files;
      if (files.length === 0) return;

      const file = files[0];
      const filePath = (file as File & { path?: string }).path;

      if (filePath && baseDir) {
        // Calculate relative path
        const relativePath = await invoke<string>('get_relative_path', {
          baseDir,
          filePath,
        });
        onChange(relativePath);
      }
    },
    [baseDir, onChange]
  );

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  }, []);

  const handleBrowse = useCallback(async () => {
    const extensions = accept.map((ext) => ext.replace('.', ''));
    const path = await open({
      filters: [{ name: 'Assets', extensions }],
      directory: false,
      multiple: false,
    });

    if (path && baseDir) {
      const relativePath = await invoke<string>('get_relative_path', {
        baseDir,
        filePath: path as string,
      });
      onChange(relativePath);
    }
  }, [accept, baseDir, onChange]);

  const handleClear = useCallback(() => {
    onChange(undefined);
  }, [onChange]);

  return (
    <div className="asset-field">
      <label>{label}</label>
      <div
        className={`asset-drop-zone ${value ? 'has-value' : ''}`}
        onDrop={handleDrop}
        onDragOver={handleDragOver}
      >
        {value ? (
          <div className="asset-value">
            <span className="asset-path">{value}</span>
            <button className="clear-button" onClick={handleClear}>
              ×
            </button>
          </div>
        ) : (
          <div className="drop-placeholder">
            Drop file here or{' '}
            <button className="browse-button" onClick={handleBrowse}>
              browse
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
```

### 6. Relative Path Command

```rust
// src/commands/assets.rs (add)
use std::path::Path;

#[tauri::command]
pub fn get_relative_path(base_dir: &str, file_path: &str) -> Result<String, String> {
    let base = Path::new(base_dir);
    let file = Path::new(file_path);

    // Try to create relative path
    if let Ok(relative) = file.strip_prefix(base) {
        return Ok(relative.to_string_lossy().to_string());
    }

    // If not under base_dir, return absolute path
    Ok(file_path.to_string())
}
```

### 7. VisualSection with AssetField

```typescript
// ui/src/components/CommandForm/VisualSection.tsx
import { FC } from 'react';
import { AssetField } from './AssetField';

interface Props {
  background: string | undefined;
  character: string | undefined;
  charPos: string | undefined;
  baseDir: string | null;
  onBackgroundChange: (value: string | undefined) => void;
  onCharacterChange: (value: string | undefined) => void;
  onCharPosChange: (value: string | undefined) => void;
}

export const VisualSection: FC<Props> = ({
  background,
  character,
  charPos,
  baseDir,
  onBackgroundChange,
  onCharacterChange,
  onCharPosChange,
}) => {
  return (
    <section className="form-section">
      <h3>Visual</h3>

      <AssetField
        label="Background"
        value={background}
        baseDir={baseDir}
        accept={['.png', '.jpg', '.jpeg', '.webp']}
        onChange={onBackgroundChange}
      />

      <AssetField
        label="Character"
        value={character}
        baseDir={baseDir}
        accept={['.png', '.jpg', '.jpeg', '.webp']}
        onChange={onCharacterChange}
      />

      <div className="form-field">
        <label htmlFor="char_pos">Position</label>
        <select
          id="char_pos"
          value={charPos ?? 'center'}
          onChange={(e) => onCharPosChange(e.target.value || undefined)}
        >
          <option value="left">Left</option>
          <option value="center">Center</option>
          <option value="right">Right</option>
        </select>
      </div>
    </section>
  );
};
```

### 8. AudioSection with AssetField

```typescript
// ui/src/components/CommandForm/AudioSection.tsx
import { FC } from 'react';
import { AssetField } from './AssetField';

interface Props {
  bgm: string | undefined;
  se: string | undefined;
  voice: string | undefined;
  baseDir: string | null;
  onBgmChange: (value: string | undefined) => void;
  onSeChange: (value: string | undefined) => void;
  onVoiceChange: (value: string | undefined) => void;
}

export const AudioSection: FC<Props> = ({
  bgm,
  se,
  voice,
  baseDir,
  onBgmChange,
  onSeChange,
  onVoiceChange,
}) => {
  return (
    <section className="form-section">
      <h3>Audio</h3>

      <AssetField
        label="BGM"
        value={bgm}
        baseDir={baseDir}
        accept={['.mp3', '.ogg', '.wav']}
        onChange={onBgmChange}
      />

      <AssetField
        label="Sound Effect"
        value={se}
        baseDir={baseDir}
        accept={['.mp3', '.ogg', '.wav']}
        onChange={onSeChange}
      />

      <AssetField
        label="Voice"
        value={voice}
        baseDir={baseDir}
        accept={['.mp3', '.ogg', '.wav']}
        onChange={onVoiceChange}
      />
    </section>
  );
};
```

### 9. Styles

```css
/* ui/src/components/CommandList/styles.css (add) */
.command-row {
  display: grid;
  grid-template-columns: 30px 40px 80px 1fr 60px;
  align-items: center;
  padding: 8px;
  border-bottom: 1px solid #333;
  cursor: pointer;
  transition: background 0.2s;
}

.command-row:hover {
  background: #2a2a3e;
}

.command-row.selected {
  background: #3b82f6;
}

.command-row.dragging {
  background: #4a4a6a;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.drag-handle {
  cursor: grab;
  color: #666;
  font-size: 14px;
  text-align: center;
}

.drag-handle:active {
  cursor: grabbing;
}

/* ui/src/components/CommandForm/styles.css (add) */
.asset-field {
  margin-bottom: 12px;
}

.asset-field label {
  display: block;
  margin-bottom: 4px;
  font-weight: 500;
  color: #ccc;
}

.asset-drop-zone {
  border: 2px dashed #4a4a6a;
  border-radius: 8px;
  padding: 12px;
  text-align: center;
  transition: all 0.2s;
  min-height: 50px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.asset-drop-zone:hover {
  border-color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
}

.asset-drop-zone.has-value {
  border-style: solid;
  border-color: #22c55e;
  background: rgba(34, 197, 94, 0.1);
}

.asset-value {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
}

.asset-path {
  flex: 1;
  text-align: left;
  font-family: monospace;
  font-size: 12px;
  color: #fff;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.clear-button {
  background: #ef4444;
  border: none;
  color: white;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  cursor: pointer;
}

.drop-placeholder {
  color: #666;
  font-size: 13px;
}

.browse-button {
  background: none;
  border: none;
  color: #3b82f6;
  cursor: pointer;
  text-decoration: underline;
}
```

## Drag & Drop Features Summary

| Feature | Description |
|---------|-------------|
| Reorder commands | Drag handle on each row |
| Drop image files | Auto-fill background/character path |
| Drop audio files | Auto-fill bgm/se/voice path |
| Browse button | File dialog fallback |
| Clear button | Remove asset reference |

## Keyboard Accessibility

```typescript
// SortableCommandRow supports keyboard navigation via @dnd-kit
// - Enter/Space: Pick up item
// - Arrow keys: Move item
// - Escape: Cancel
```

## Verification

1. Drag command row by handle → reorder works
2. Drop image file on background field → path fills in
3. Drop audio file on BGM field → path fills in
4. Browse button → file dialog opens
5. Clear button → removes path
6. Keyboard navigation works
