import { useState, useEffect, useMemo } from "react";
import { useScenario } from "./hooks/useScenario";
import { CommandList } from "./components/CommandList";
import { CommandForm } from "./components/CommandForm";
import { YamlPreview } from "./components/YamlPreview";
import { ValidationErrors } from "./components/ValidationErrors";
import "./App.css";

const App: React.FC = () => {
  const {
    scenario,
    filePath,
    isDirty,
    selectedIndex,
    validationResult,
    yamlPreview,
    openFile,
    saveFile,
    saveFileAs,
    newScenario,
    selectCommand,
    updateCommand,
    addCommand,
    removeCommand,
    validate,
  } = useScenario();

  const [showNewDialog, setShowNewDialog] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [activeTab, setActiveTab] = useState<"form" | "yaml">("form");

  const labels = useMemo(() => {
    if (!scenario) return [];
    return scenario.script
      .filter((cmd) => cmd.label)
      .map((cmd) => cmd.label as string);
  }, [scenario]);

  const selectedCommand = useMemo(() => {
    if (!scenario || selectedIndex === null) return null;
    return scenario.script[selectedIndex];
  }, [scenario, selectedIndex]);

  useEffect(() => {
    if (scenario) {
      void validate();
    }
  }, [scenario, validate]);

  const handleNew = () => {
    setNewTitle("");
    setShowNewDialog(true);
  };

  const handleCreateNew = () => {
    if (newTitle.trim()) {
      void newScenario(newTitle.trim());
      setShowNewDialog(false);
    }
  };

  const title = filePath
    ? `${filePath.split("/").pop()}${isDirty ? " *" : ""}`
    : scenario
      ? `Untitled${isDirty ? " *" : ""}`
      : "ivy Editor";

  return (
    <div className="app">
      {/* Header */}
      <header className="app-header">
        <div className="header-title">{title}</div>
        <div className="header-actions">
          <button onClick={handleNew}>New</button>
          <button onClick={() => void openFile()}>Open</button>
          <button onClick={() => void saveFile()} disabled={!scenario}>
            Save
          </button>
          <button onClick={() => void saveFileAs()} disabled={!scenario}>
            Save As
          </button>
          <button onClick={() => void validate()} disabled={!scenario}>
            Validate
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className="app-main">
        {/* Left Panel: Command List */}
        <div className="panel panel-left">
          {scenario ? (
            <CommandList
              commands={scenario.script}
              selectedIndex={selectedIndex}
              onSelect={selectCommand}
              onAdd={addCommand}
              onRemove={removeCommand}
            />
          ) : (
            <div className="empty-state">
              <p>No scenario loaded</p>
              <button onClick={handleNew}>Create New</button>
              <button onClick={() => void openFile()}>Open File</button>
            </div>
          )}
        </div>

        {/* Center Panel: Editor / Preview */}
        <div className="panel panel-center">
          {scenario && selectedCommand ? (
            <>
              <div className="tab-bar">
                <button
                  className={activeTab === "form" ? "active" : ""}
                  onClick={() => {
                    setActiveTab("form");
                  }}
                >
                  Form
                </button>
                <button
                  className={activeTab === "yaml" ? "active" : ""}
                  onClick={() => {
                    setActiveTab("yaml");
                  }}
                >
                  YAML
                </button>
              </div>
              {activeTab === "form" ? (
                <CommandForm
                  command={selectedCommand}
                  labels={labels}
                  onChange={(cmd) => {
                    updateCommand(selectedIndex!, cmd);
                  }}
                />
              ) : (
                <YamlPreview yaml={yamlPreview} />
              )}
            </>
          ) : scenario ? (
            <div className="empty-state">
              <p>Select a command to edit</p>
            </div>
          ) : null}
        </div>

        {/* Right Panel: Validation */}
        <div className="panel panel-right">
          <ValidationErrors
            result={validationResult}
            onSelectCommand={selectCommand}
          />
        </div>
      </main>

      {/* New Scenario Dialog */}
      {showNewDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <h2>New Scenario</h2>
            <input
              type="text"
              value={newTitle}
              onChange={(e) => {
                setNewTitle(e.target.value);
              }}
              placeholder="Scenario title"
              autoFocus
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleCreateNew();
                }
              }}
            />
            <div className="dialog-actions">
              <button
                onClick={() => {
                  setShowNewDialog(false);
                }}
              >
                Cancel
              </button>
              <button onClick={handleCreateNew} disabled={!newTitle.trim()}>
                Create
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default App;
