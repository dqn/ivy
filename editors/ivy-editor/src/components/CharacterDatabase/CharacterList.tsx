import type { CharacterDatabase } from "../../types/character";

interface Props {
  database: CharacterDatabase;
  selectedCharacter: string | null;
  onSelect: (name: string) => void;
  onAdd: () => void;
  onRemove: (name: string) => void;
}

export const CharacterList: React.FC<Props> = ({
  database,
  selectedCharacter,
  onSelect,
  onAdd,
  onRemove,
}) => {
  const characterNames = Object.keys(database.characters).sort();

  return (
    <div className="character-list">
      <div className="character-list-header">
        <span>Characters ({characterNames.length})</span>
        <button className="add-button" onClick={onAdd}>
          + Add
        </button>
      </div>

      <div className="character-list-items">
        {characterNames.length === 0 ? (
          <div className="empty-message">No characters defined</div>
        ) : (
          characterNames.map((name) => {
            const charDef = database.characters[name];
            const layerCount = charDef?.layers?.length ?? 0;
            const aliasCount = charDef?.aliases?.length ?? 0;

            return (
              <div
                key={name}
                className={`character-item ${selectedCharacter === name ? "selected" : ""}`}
                onClick={() => onSelect(name)}
              >
                <div className="character-item-main">
                  <span className="character-name">{name}</span>
                  <span className="character-meta">
                    {layerCount} layer{layerCount !== 1 ? "s" : ""}
                    {aliasCount > 0 && ` · ${aliasCount} alias${aliasCount !== 1 ? "es" : ""}`}
                  </span>
                </div>
                <button
                  className="remove-button"
                  onClick={(e) => {
                    e.stopPropagation();
                    onRemove(name);
                  }}
                  title="Remove character"
                >
                  ×
                </button>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};
