import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { LayerEditor } from "./LayerEditor";
import { CharacterPreview } from "./CharacterPreview";
import { useToast } from "../Toast";
import type { CharacterDef } from "../../types/character";
import type { LayerDef } from "../../types/scenario";

interface Props {
  name: string;
  character: CharacterDef;
  baseDir: string | null;
  onChange: (character: CharacterDef) => void;
  onAddLayer: (layer?: LayerDef) => void;
  onUpdateLayer: (index: number, layer: LayerDef) => void;
  onRemoveLayer: (index: number) => void;
  onReorderLayers: (fromIndex: number, toIndex: number) => void;
}

export const CharacterEditor: React.FC<Props> = ({
  name,
  character,
  baseDir,
  onChange,
  onAddLayer,
  onUpdateLayer,
  onRemoveLayer,
  onReorderLayers,
}) => {
  const { showToast } = useToast();
  const [newAlias, setNewAlias] = useState("");
  const [previewVariants, setPreviewVariants] = useState<Record<string, number>>(
    {}
  );

  const handleVariantChange = useCallback(
    (layerName: string, variantIndex: number) => {
      setPreviewVariants((prev) => ({ ...prev, [layerName]: variantIndex }));
    },
    []
  );

  const handleBaseChange = useCallback(
    (path: string | undefined) => {
      onChange({ ...character, base: path ?? "" });
    },
    [character, onChange]
  );

  const handleBrowseBase = useCallback(async () => {
    const path = await open({
      filters: [
        { name: "Images", extensions: ["png", "jpg", "jpeg", "webp", "gif"] },
      ],
      directory: false,
      multiple: false,
    });

    if (path && baseDir) {
      try {
        const relativePath = await invoke<string>("get_relative_path", {
          baseDir,
          filePath: path,
        });
        handleBaseChange(relativePath);
      } catch {
        handleBaseChange(path);
      }
    } else if (path) {
      handleBaseChange(path);
    }
  }, [baseDir, handleBaseChange]);

  const handleAddAlias = useCallback(() => {
    const alias = newAlias.trim();
    if (!alias) {return;}

    const currentAliases = character.aliases ?? [];
    if (currentAliases.includes(alias)) {
      showToast("This alias already exists", "warning");
      return;
    }

    onChange({ ...character, aliases: [...currentAliases, alias] });
    setNewAlias("");
  }, [newAlias, character, onChange, showToast]);

  const handleRemoveAlias = useCallback(
    (index: number) => {
      const newAliases = (character.aliases ?? []).filter((_, i) => i !== index);
      onChange({ ...character, aliases: newAliases });
    },
    [character, onChange]
  );

  return (
    <div className="character-editor">
      <h3>Edit: {name}</h3>

      {/* Preview */}
      <CharacterPreview
        character={character}
        baseDir={baseDir}
        variantSelections={previewVariants}
      />

      {/* Base Image */}
      <div className="editor-section">
        <label>Base Image</label>
        <div className="base-image-field">
          <input
            type="text"
            value={character.base}
            onChange={(e) => { handleBaseChange(e.target.value || undefined); }}
            placeholder="assets/characters/base.png"
          />
          <button onClick={() => { void handleBrowseBase(); }}>Browse</button>
          {character.base && (
            <button onClick={() => { handleBaseChange(undefined); }}>Clear</button>
          )}
        </div>
      </div>

      {/* Aliases (Speaker Names) */}
      <div className="editor-section">
        <label>
          Speaker Aliases
          <span className="hint">Names that map to this character</span>
        </label>
        <div className="aliases-list">
          {(character.aliases ?? []).map((alias, index) => (
            <div key={index} className="alias-item">
              <span>{alias}</span>
              <button
                className="remove-alias"
                onClick={() => handleRemoveAlias(index)}
              >
                ×
              </button>
            </div>
          ))}
        </div>
        <div className="add-alias-row">
          <input
            type="text"
            value={newAlias}
            onChange={(e) => setNewAlias(e.target.value)}
            placeholder="Add alias..."
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleAddAlias();
              }
            }}
          />
          <button onClick={handleAddAlias} disabled={!newAlias.trim()}>
            Add
          </button>
        </div>
      </div>

      {/* Layers */}
      <div className="editor-section">
        <div className="section-header">
          <label>Layers</label>
          <button className="add-layer-button" onClick={() => onAddLayer()}>
            + Add Layer
          </button>
        </div>
        <div className="layers-list">
          {(character.layers ?? []).length === 0 ? (
            <div className="empty-layers">
              No layers defined. Add layers for expression, outfit, etc.
            </div>
          ) : (
            (character.layers ?? []).map((layer, index) => (
              <LayerEditor
                key={index}
                layer={layer}
                baseDir={baseDir}
                selectedVariant={previewVariants[layer.name] ?? 0}
                onChange={(updated: LayerDef) => onUpdateLayer(index, updated)}
                onRemove={() => onRemoveLayer(index)}
                onMoveUp={
                  index > 0 ? () => onReorderLayers(index, index - 1) : undefined
                }
                onMoveDown={
                  index < (character.layers?.length ?? 0) - 1
                    ? () => onReorderLayers(index, index + 1)
                    : undefined
                }
                onVariantSelect={(variantIndex: number) =>
                  handleVariantChange(layer.name, variantIndex)
                }
              />
            ))
          )}
        </div>
      </div>
    </div>
  );
};
