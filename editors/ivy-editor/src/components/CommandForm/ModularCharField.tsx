import { useCallback, useMemo } from "react";
import type { ModularCharRef, CharPosition } from "../../types/scenario";
import type { CharacterDatabase, CharacterDef } from "../../types/character";

interface Props {
  value: ModularCharRef | undefined;
  charPos: CharPosition | undefined;
  characterDatabase: CharacterDatabase;
  onChange: (value: ModularCharRef | undefined) => void;
  onCharPosChange: (pos: CharPosition | undefined) => void;
}

export const ModularCharField: React.FC<Props> = ({
  value,
  charPos,
  characterDatabase,
  onChange,
  onCharPosChange,
}) => {
  const characterNames = useMemo(() => {
    return Object.keys(characterDatabase.characters).sort();
  }, [characterDatabase]);

  const selectedCharDef: CharacterDef | null = useMemo(() => {
    if (!value?.name) return null;
    return characterDatabase.characters[value.name] ?? null;
  }, [value?.name, characterDatabase]);

  const handleNameChange = useCallback(
    (newName: string) => {
      if (!newName) {
        onChange(undefined);
        return;
      }

      const charDef = characterDatabase.characters[newName];
      if (!charDef) {
        onChange(undefined);
        return;
      }

      // Initialize with default variants (all 0)
      const variants: Record<string, number> = {};
      for (const layer of charDef.layers ?? []) {
        variants[layer.name] = 0;
      }

      onChange({ name: newName, ...variants });
    },
    [characterDatabase, onChange]
  );

  const handleVariantChange = useCallback(
    (layerName: string, variantIndex: number) => {
      if (!value) return;

      const updated: ModularCharRef = {
        ...value,
        [layerName]: variantIndex,
      };
      onChange(updated);
    },
    [value, onChange]
  );

  const handleClear = useCallback(() => {
    onChange(undefined);
  }, [onChange]);

  if (characterNames.length === 0) {
    return (
      <div className="modular-char-field">
        <label>Modular Character</label>
        <div className="empty-characters">
          No characters defined. Create characters in the Characters tab first.
        </div>
      </div>
    );
  }

  return (
    <div className="modular-char-field">
      <label>Modular Character</label>

      <div className="modular-char-select">
        <select
          value={value?.name ?? ""}
          onChange={(e) => handleNameChange(e.target.value)}
        >
          <option value="">-- None --</option>
          {characterNames.map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
        </select>

        {value?.name && (
          <button className="clear-button" onClick={handleClear}>
            ×
          </button>
        )}
      </div>

      {value?.name && selectedCharDef && (
        <>
          <div className="modular-char-position">
            <label>Position</label>
            <select
              value={charPos ?? "center"}
              onChange={(e) =>
                onCharPosChange(
                  e.target.value === "center"
                    ? undefined
                    : (e.target.value as CharPosition)
                )
              }
            >
              <option value="left">Left</option>
              <option value="center">Center</option>
              <option value="right">Right</option>
            </select>
          </div>

          <div className="modular-char-layers">
            {(selectedCharDef.layers ?? []).length === 0 ? (
              <div className="no-layers">No layers defined for this character</div>
            ) : (
              (selectedCharDef.layers ?? []).map((layer) => {
                const currentVariant =
                  typeof value[layer.name] === "number"
                    ? (value[layer.name] as number)
                    : 0;

                return (
                  <div key={layer.name} className="layer-variant-row">
                    <span className="layer-label">{layer.name}</span>
                    <select
                      value={currentVariant}
                      onChange={(e) =>
                        handleVariantChange(layer.name, parseInt(e.target.value))
                      }
                    >
                      {layer.images.map((imagePath, index) => (
                        <option key={index} value={index}>
                          {index}: {imagePath.split("/").pop()}
                        </option>
                      ))}
                    </select>
                  </div>
                );
              })
            )}
          </div>
        </>
      )}
    </div>
  );
};
