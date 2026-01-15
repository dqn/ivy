import { useState, useCallback } from "react";
import { CharacterList } from "./CharacterList";
import { CharacterEditor } from "./CharacterEditor";
import { SpeakerMapping } from "./SpeakerMapping";
import type { CharacterDatabase as CharacterDatabaseType, CharacterDef } from "../../types/character";
import type { LayerDef, Scenario } from "../../types/scenario";
import "./styles.css";

interface Props {
  database: CharacterDatabaseType;
  selectedCharacter: string | null;
  baseDir: string | null;
  scenario?: Scenario | null;
  onSelectCharacter: (name: string | null) => void;
  onAddCharacter: (name: string) => void;
  onUpdateCharacter: (name: string, def: CharacterDef) => void;
  onRemoveCharacter: (name: string) => void;
  onAddLayer: (charName: string, layer?: LayerDef) => void;
  onUpdateLayer: (charName: string, index: number, layer: LayerDef) => void;
  onRemoveLayer: (charName: string, index: number) => void;
  onReorderLayers: (charName: string, fromIndex: number, toIndex: number) => void;
}

export const CharacterDatabase: React.FC<Props> = ({
  database,
  selectedCharacter,
  baseDir,
  scenario,
  onSelectCharacter,
  onAddCharacter,
  onUpdateCharacter,
  onRemoveCharacter,
  onAddLayer,
  onUpdateLayer,
  onRemoveLayer,
  onReorderLayers,
}) => {
  const [showAddDialog, setShowAddDialog] = useState(false);
  const [newCharacterName, setNewCharacterName] = useState("");
  const [showDeleteConfirm, setShowDeleteConfirm] = useState<string | null>(
    null
  );

  const handleAdd = useCallback(() => {
    setNewCharacterName("");
    setShowAddDialog(true);
  }, []);

  const handleConfirmAdd = useCallback(() => {
    const name = newCharacterName.trim();
    if (!name) return;

    if (database.characters[name]) {
      alert(`Character '${name}' already exists`);
      return;
    }

    onAddCharacter(name);
    setShowAddDialog(false);
    setNewCharacterName("");
  }, [newCharacterName, database.characters, onAddCharacter]);

  const handleRemove = useCallback((name: string) => {
    setShowDeleteConfirm(name);
  }, []);

  const handleConfirmRemove = useCallback(() => {
    if (showDeleteConfirm) {
      onRemoveCharacter(showDeleteConfirm);
      setShowDeleteConfirm(null);
    }
  }, [showDeleteConfirm, onRemoveCharacter]);

  const selectedCharDef = selectedCharacter
    ? database.characters[selectedCharacter]
    : null;

  return (
    <div className="character-database">
      <div className="character-database-left">
        <CharacterList
          database={database}
          selectedCharacter={selectedCharacter}
          onSelect={onSelectCharacter}
          onAdd={handleAdd}
          onRemove={handleRemove}
        />
      </div>

      <div className="character-database-right">
        {selectedCharacter && selectedCharDef ? (
          <CharacterEditor
            name={selectedCharacter}
            character={selectedCharDef}
            baseDir={baseDir}
            onChange={(updated) => onUpdateCharacter(selectedCharacter, updated)}
            onAddLayer={(layer) => onAddLayer(selectedCharacter, layer)}
            onUpdateLayer={(index, layer) => onUpdateLayer(selectedCharacter, index, layer)}
            onRemoveLayer={(index) => onRemoveLayer(selectedCharacter, index)}
            onReorderLayers={(from, to) => onReorderLayers(selectedCharacter, from, to)}
          />
        ) : (
          <>
            <SpeakerMapping
              database={database}
              scenario={scenario ?? null}
              onSelectCharacter={onSelectCharacter}
            />
            <div className="empty-state">
              <p>Select a character to edit</p>
            </div>
          </>
        )}
      </div>

      {/* Add Character Dialog */}
      {showAddDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <h2>Add Character</h2>
            <input
              type="text"
              value={newCharacterName}
              onChange={(e) => setNewCharacterName(e.target.value)}
              placeholder="Character name (e.g., sakura)"
              autoFocus
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleConfirmAdd();
                } else if (e.key === "Escape") {
                  setShowAddDialog(false);
                }
              }}
            />
            <div className="dialog-actions">
              <button onClick={() => setShowAddDialog(false)}>Cancel</button>
              <button
                onClick={handleConfirmAdd}
                disabled={!newCharacterName.trim()}
              >
                Add
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Delete Confirmation Dialog */}
      {showDeleteConfirm && (
        <div className="dialog-overlay">
          <div className="dialog">
            <h2>Delete Character</h2>
            <p>
              Are you sure you want to delete "{showDeleteConfirm}"? This action
              cannot be undone.
            </p>
            <div className="dialog-actions">
              <button onClick={() => setShowDeleteConfirm(null)}>Cancel</button>
              <button className="danger" onClick={handleConfirmRemove}>
                Delete
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
