interface EditorHeaderProps {
  title: string;
  isProjectMode: boolean;
  isStandaloneMode: boolean;
  hasScenario: boolean;
  view: "list" | "flowchart";
  onBack: () => void;
  onNew: () => void;
  onOpen: () => void;
  onSave: () => void;
  onSaveAs: () => void;
  onValidate: () => void;
  onViewChange: (view: "list" | "flowchart") => void;
  onSettings?: () => void;
  onExport?: () => void;
}

export const EditorHeader: React.FC<EditorHeaderProps> = ({
  title,
  isProjectMode,
  isStandaloneMode,
  hasScenario,
  view,
  onBack,
  onNew,
  onOpen,
  onSave,
  onSaveAs,
  onValidate,
  onViewChange,
  onSettings,
  onExport,
}) => {
  return (
    <header className="app-header">
      <div className="header-left">
        <button className="back-button" onClick={onBack}>
          ←
        </button>
        <div className="header-title">{title}</div>
      </div>
      <div className="header-actions">
        {isProjectMode && (
          <>
            <button onClick={onSettings}>Settings</button>
            <button onClick={onExport}>Export</button>
          </>
        )}
        {isStandaloneMode && (
          <>
            <button onClick={onNew}>New</button>
            <button onClick={onOpen}>Open</button>
          </>
        )}
        <button onClick={onSave} disabled={!hasScenario}>
          Save
        </button>
        <button onClick={onSaveAs} disabled={!hasScenario}>
          Save As
        </button>
        <button onClick={onValidate} disabled={!hasScenario}>
          Validate
        </button>
        <div className="view-toggle">
          <button
            className={view === "list" ? "active" : ""}
            onClick={() => onViewChange("list")}
            disabled={!hasScenario}
          >
            List
          </button>
          <button
            className={view === "flowchart" ? "active" : ""}
            onClick={() => onViewChange("flowchart")}
            disabled={!hasScenario}
          >
            Flowchart
          </button>
        </div>
      </div>
    </header>
  );
};
