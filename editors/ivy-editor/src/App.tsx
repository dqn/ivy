import { useState, useEffect, useMemo, useCallback } from "react";
import { useScenario } from "./hooks/useScenario";
import { usePreview } from "./hooks/usePreview";
import { CommandList } from "./components/CommandList";
import { CommandForm } from "./components/CommandForm";
import { YamlPreview } from "./components/YamlPreview";
import { ValidationErrors } from "./components/ValidationErrors";
import { FlowchartView } from "./components/FlowchartView";
import { PreviewPanel } from "./components/PreviewPanel";
import { AssetBrowser } from "./components/AssetBrowser";
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
    reorderCommand,
    validate,
  } = useScenario();

  const [showNewDialog, setShowNewDialog] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [activeTab, setActiveTab] = useState<"form" | "yaml">("form");
  const [view, setView] = useState<"list" | "flowchart">("list");
  const [sidebarTab, setSidebarTab] = useState<"commands" | "assets">(
    "commands",
  );
  const [highlightedIndices, setHighlightedIndices] = useState<number[]>([]);

  // Compute base directory for asset loading
  const baseDir = useMemo(() => {
    if (!filePath) return null;
    const lastSlash = filePath.lastIndexOf("/");
    return lastSlash >= 0 ? filePath.substring(0, lastSlash) : null;
  }, [filePath]);

  // Preview state
  const {
    state: previewState,
    backgroundUrl,
    characterUrl,
    goto: previewGoto,
    next: previewNext,
    prev: previewPrev,
  } = usePreview(scenario, baseDir, selectedIndex);

  // Sync: when preview navigates, update editor selection
  const handlePreviewGoto = useCallback(
    (index: number) => {
      previewGoto(index);
      selectCommand(index);
    },
    [previewGoto, selectCommand],
  );

  // Asset browser handlers
  const handleSelectAsset = useCallback(
    (path: string) => {
      if (selectedIndex === null || !scenario) {
        return;
      }
      const cmd = scenario.script[selectedIndex];

      // Determine field based on file extension
      const ext = path.split(".").pop()?.toLowerCase();
      const isImage = ["png", "jpg", "jpeg", "gif", "webp", "svg"].includes(
        ext || "",
      );
      const isAudio = ["mp3", "ogg", "wav", "flac"].includes(ext || "");

      if (isImage) {
        // Default to background, user can move to character in form
        updateCommand(selectedIndex, { ...cmd, background: path });
      } else if (isAudio) {
        // Default to bgm
        updateCommand(selectedIndex, { ...cmd, bgm: path });
      }

      // Switch to commands tab to show the update
      setSidebarTab("commands");
    },
    [selectedIndex, scenario, updateCommand],
  );

  const handleShowUsages = useCallback(
    (indices: number[]) => {
      setHighlightedIndices(indices);
      setSidebarTab("commands");
      if (indices.length > 0) {
        selectCommand(indices[0]);
      }
    },
    [selectCommand],
  );

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
          <div className="view-toggle">
            <button
              className={view === "list" ? "active" : ""}
              onClick={() => {
                setView("list");
              }}
              disabled={!scenario}
            >
              List
            </button>
            <button
              className={view === "flowchart" ? "active" : ""}
              onClick={() => {
                setView("flowchart");
              }}
              disabled={!scenario}
            >
              Flowchart
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="app-main">
        {/* Left Panel: Command List / Flowchart / Assets */}
        <div className="panel panel-left">
          {scenario ? (
            <>
              <div className="sidebar-tabs">
                <button
                  className={sidebarTab === "commands" ? "active" : ""}
                  onClick={() => setSidebarTab("commands")}
                >
                  Commands
                </button>
                <button
                  className={sidebarTab === "assets" ? "active" : ""}
                  onClick={() => setSidebarTab("assets")}
                >
                  Assets
                </button>
              </div>
              {sidebarTab === "commands" ? (
                view === "list" ? (
                  <CommandList
                    commands={scenario.script}
                    selectedIndex={selectedIndex}
                    highlightedIndices={highlightedIndices}
                    onSelect={selectCommand}
                    onAdd={addCommand}
                    onRemove={removeCommand}
                    onReorder={reorderCommand}
                  />
                ) : (
                  <FlowchartView
                    scenario={scenario}
                    onNodeClick={selectCommand}
                  />
                )
              ) : (
                <AssetBrowser
                  baseDir={baseDir}
                  scenario={scenario}
                  onSelectAsset={handleSelectAsset}
                  onShowUsages={handleShowUsages}
                />
              )}
            </>
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
                  baseDir={baseDir}
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

        {/* Right Panel: Preview + Validation */}
        <div className="panel panel-right">
          <PreviewPanel
            state={previewState}
            backgroundUrl={backgroundUrl}
            characterUrl={characterUrl}
            onPrev={previewPrev}
            onNext={previewNext}
            onGoto={handlePreviewGoto}
          />
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
