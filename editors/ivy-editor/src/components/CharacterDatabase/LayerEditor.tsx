import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { LayerDef } from "../../types/scenario";

interface Props {
  layer: LayerDef;
  baseDir: string | null;
  selectedVariant?: number;
  onChange: (layer: LayerDef) => void;
  onRemove: () => void;
  onMoveUp?: () => void;
  onMoveDown?: () => void;
  onVariantSelect?: (variantIndex: number) => void;
}

export const LayerEditor: React.FC<Props> = ({
  layer,
  baseDir,
  selectedVariant = 0,
  onChange,
  onRemove,
  onMoveUp,
  onMoveDown,
  onVariantSelect,
}) => {
  const [isExpanded, setIsExpanded] = useState(true);
  const [editingName, setEditingName] = useState(false);
  const [tempName, setTempName] = useState(layer.name);

  const handleNameSave = useCallback(() => {
    const newName = tempName.trim();
    if (newName && newName !== layer.name) {
      onChange({ ...layer, name: newName });
    }
    setEditingName(false);
  }, [tempName, layer, onChange]);

  const handleAddVariant = useCallback(async () => {
    const paths = await open({
      filters: [
        { name: "Images", extensions: ["png", "jpg", "jpeg", "webp", "gif"] },
      ],
      directory: false,
      multiple: true,
    });

    if (!paths) return;

    const pathArray = Array.isArray(paths) ? paths : [paths];
    const newImages = [...layer.images];

    for (const path of pathArray) {
      if (baseDir) {
        try {
          const relativePath = await invoke<string>("get_relative_path", {
            baseDir,
            filePath: path as string,
          });
          newImages.push(relativePath);
        } catch {
          newImages.push(path as string);
        }
      } else {
        newImages.push(path as string);
      }
    }

    onChange({ ...layer, images: newImages });
  }, [baseDir, layer, onChange]);

  const handleRemoveVariant = useCallback(
    (variantIndex: number) => {
      const newImages = layer.images.filter((_, i) => i !== variantIndex);
      onChange({ ...layer, images: newImages });
    },
    [layer, onChange]
  );

  return (
    <div className="layer-editor">
      <div className="layer-header">
        <button
          className="expand-toggle"
          onClick={() => setIsExpanded(!isExpanded)}
        >
          {isExpanded ? "▼" : "▶"}
        </button>

        {editingName ? (
          <input
            type="text"
            className="layer-name-input"
            value={tempName}
            onChange={(e) => setTempName(e.target.value)}
            onBlur={handleNameSave}
            onKeyDown={(e) => {
              if (e.key === "Enter") handleNameSave();
              if (e.key === "Escape") {
                setTempName(layer.name);
                setEditingName(false);
              }
            }}
            autoFocus
          />
        ) : (
          <span
            className="layer-name"
            onDoubleClick={() => {
              setTempName(layer.name);
              setEditingName(true);
            }}
            title="Double-click to edit"
          >
            {layer.name}
          </span>
        )}

        <span className="variant-count">{layer.images.length} variants</span>

        <div className="layer-actions">
          {onMoveUp && (
            <button className="move-button" onClick={onMoveUp} title="Move up">
              ↑
            </button>
          )}
          {onMoveDown && (
            <button
              className="move-button"
              onClick={onMoveDown}
              title="Move down"
            >
              ↓
            </button>
          )}
          <button className="remove-layer" onClick={onRemove} title="Remove layer">
            ×
          </button>
        </div>
      </div>

      {isExpanded && (
        <div className="layer-content">
          <div className="variants-grid">
            {layer.images.map((imagePath, variantIndex) => (
              <div
                key={variantIndex}
                className={`variant-item${variantIndex === selectedVariant ? " selected" : ""}`}
                onClick={() => onVariantSelect?.(variantIndex)}
              >
                <span className="variant-index">{variantIndex}</span>
                <span className="variant-path" title={imagePath}>
                  {imagePath.split("/").pop()}
                </span>
                <button
                  className="remove-variant"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRemoveVariant(variantIndex);
                  }}
                >
                  ×
                </button>
              </div>
            ))}
          </div>
          <button
            className="add-variant-button"
            onClick={() => void handleAddVariant()}
          >
            + Add Variant
          </button>
        </div>
      )}
    </div>
  );
};
