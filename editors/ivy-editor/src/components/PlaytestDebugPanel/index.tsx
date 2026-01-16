import { useState } from "react";
import type {
  PlaytestState,
  PlaytestHistoryEntry,
  Value,
} from "../../types/playtest";
import "./styles.css";

interface Props {
  state: PlaytestState | null;
  onJumpToLabel: (label: string) => void;
  onSetVariable: (name: string, value: Value) => void;
  onRollbackToIndex: (steps: number) => void;
}

export const PlaytestDebugPanel: React.FC<Props> = ({
  state,
  onJumpToLabel,
  onSetVariable,
  onRollbackToIndex,
}) => {
  const [activeTab, setActiveTab] = useState<"history" | "labels" | "variables">(
    "history"
  );
  const [editingVar, setEditingVar] = useState<string | null>(null);
  const [editValue, setEditValue] = useState("");

  if (!state) {
    return null;
  }

  const handleVariableEdit = (name: string) => {
    const currentValue = state.variables[name];
    setEditingVar(name);
    setEditValue(
      typeof currentValue === "string"
        ? currentValue
        : JSON.stringify(currentValue)
    );
  };

  const handleVariableSave = () => {
    if (!editingVar) return;

    // Try to parse the value
    let value: Value;
    if (editValue === "true") {
      value = true;
    } else if (editValue === "false") {
      value = false;
    } else if (/^-?\d+$/.test(editValue)) {
      value = parseInt(editValue, 10);
    } else {
      value = editValue;
    }

    onSetVariable(editingVar, value);
    setEditingVar(null);
    setEditValue("");
  };

  const handleHistoryClick = (_entry: PlaytestHistoryEntry, historyIndex: number) => {
    // Calculate how many steps to rollback
    const stepsBack = state.history.length - historyIndex;
    if (stepsBack > 0) {
      onRollbackToIndex(stepsBack);
    }
  };

  return (
    <div className="playtest-debug-panel">
      <div className="debug-panel-header">
        <span className="debug-panel-title">Debug</span>
        <div className="debug-tabs">
          <button
            className={activeTab === "history" ? "active" : ""}
            onClick={() => setActiveTab("history")}
          >
            History ({state.history.length})
          </button>
          <button
            className={activeTab === "labels" ? "active" : ""}
            onClick={() => setActiveTab("labels")}
          >
            Labels ({state.labels.length})
          </button>
          <button
            className={activeTab === "variables" ? "active" : ""}
            onClick={() => setActiveTab("variables")}
          >
            Vars ({Object.keys(state.variables).length})
          </button>
        </div>
      </div>

      <div className="debug-panel-content">
        {activeTab === "history" && (
          <div className="history-list">
            {state.history.length === 0 ? (
              <div className="empty-message">No history yet</div>
            ) : (
              state.history.map((entry, idx) => (
                <div
                  key={idx}
                  className="history-entry"
                  onClick={() => handleHistoryClick(entry, idx)}
                  title="Click to rollback to this point"
                >
                  <span className="history-index">#{entry.index}</span>
                  {entry.speaker && (
                    <span className="history-speaker">{entry.speaker}:</span>
                  )}
                  <span className="history-text">
                    {entry.text.length > 50
                      ? `${entry.text.substring(0, 50)}...`
                      : entry.text}
                  </span>
                </div>
              ))
            )}
          </div>
        )}

        {activeTab === "labels" && (
          <div className="labels-list">
            {state.labels.length === 0 ? (
              <div className="empty-message">No labels defined</div>
            ) : (
              state.labels.map((label) => (
                <div
                  key={label}
                  className={`label-entry ${label === state.current_label ? "current" : ""}`}
                  onClick={() => onJumpToLabel(label)}
                  title="Click to jump to this label"
                >
                  <span className="label-name">{label}</span>
                  {label === state.current_label && (
                    <span className="label-current-badge">current</span>
                  )}
                </div>
              ))
            )}
          </div>
        )}

        {activeTab === "variables" && (
          <div className="variables-list">
            {Object.keys(state.variables).length === 0 ? (
              <div className="empty-message">No variables set</div>
            ) : (
              Object.entries(state.variables).map(([name, value]) => (
                <div key={name} className="variable-entry">
                  {editingVar === name ? (
                    <div className="variable-edit">
                      <span className="variable-name">{name}:</span>
                      <input
                        type="text"
                        value={editValue}
                        onChange={(e) => setEditValue(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === "Enter") {
                            handleVariableSave();
                          } else if (e.key === "Escape") {
                            setEditingVar(null);
                          }
                        }}
                        autoFocus
                      />
                      <button onClick={handleVariableSave}>Save</button>
                      <button onClick={() => setEditingVar(null)}>Cancel</button>
                    </div>
                  ) : (
                    <div
                      className="variable-display"
                      onClick={() => handleVariableEdit(name)}
                      title="Click to edit"
                    >
                      <span className="variable-name">{name}:</span>
                      <span className="variable-value">
                        {typeof value === "string" ? `"${value}"` : String(value)}
                      </span>
                    </div>
                  )}
                </div>
              ))
            )}
          </div>
        )}
      </div>
    </div>
  );
};
