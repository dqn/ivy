import { useState, useEffect, useMemo, useCallback } from "react";
import { Group, Panel, Separator } from "react-resizable-panels";
import { useTranslation } from "react-i18next";
import { useScenario } from "./hooks/useScenario";
import { usePreview } from "./hooks/usePreview";
import { usePlaytest } from "./hooks/usePlaytest";
import { usePlaytestKeyboard } from "./hooks/usePlaytestKeyboard";
import { useProject } from "./hooks/useProject";
import { useRecentProjects } from "./hooks/useRecentProjects";
import { useCharacters } from "./hooks/useCharacters";
import { useToast } from "./components/Toast";
import { CommandList } from "./components/CommandList";
import { CommandForm } from "./components/CommandForm";
import { YamlPreview } from "./components/YamlPreview";
import { ValidationErrors } from "./components/ValidationErrors";
import { FlowchartView } from "./components/FlowchartView";
import { PreviewPanel } from "./components/PreviewPanel";
import { AssetBrowser } from "./components/AssetBrowser";
import { CharacterDatabase } from "./components/CharacterDatabase";
import { TranslationTable } from "./components/TranslationTable";
import { VariableWatcher } from "./components/VariableWatcher";
import { StoryPathAnalyzer } from "./components/StoryPathAnalyzer";
import { SaveDataValidator } from "./components/SaveDataValidator";
import { ExportWizard } from "./components/ExportWizard";
import { WelcomeScreen } from "./components/WelcomeScreen";
import { ProjectWizard } from "./components/ProjectWizard";
import { ProjectSettings } from "./components/ProjectSettings";
import { ScenarioList } from "./components/ScenarioList";
import { PlaytestDebugPanel } from "./components/PlaytestDebugPanel";
import { LanguageSwitcher } from "./components/LanguageSwitcher";
import { Tooltip } from "./components/Tooltip";
import type { Resolution, ProjectConfig } from "./types/project";
import "./i18n";
import "./App.css";

type EditorMode =
  | { type: "welcome" }
  | { type: "project" }
  | { type: "standalone" };

const App: React.FC = () => {
  const { t } = useTranslation();
  const { showToast } = useToast();
  const [mode, setMode] = useState<EditorMode>({ type: "welcome" });

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
  } = useScenario({ showToast });

  const {
    project,
    isDirty: projectIsDirty,
    activeScenarioPath,
    createProject,
    openProject,
    openProjectFromPath,
    saveProject,
    closeProject,
    updateConfig,
    setActiveScenario,
  } = useProject({ showToast });

  const {
    recentProjects,
    addRecentProject,
    removeRecentProject,
  } = useRecentProjects();

  const {
    database: characterDatabase,
    selectedCharacter,
    isDirty: charactersDirty,
    loadCharacters,
    saveCharacters,
    selectCharacter,
    addCharacter,
    updateCharacter,
    removeCharacter,
    addLayer,
    updateLayer,
    removeLayer,
    reorderLayers,
  } = useCharacters();

  const [showNewDialog, setShowNewDialog] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [activeTab, setActiveTab] = useState<"form" | "yaml">("form");
  const [view, setView] = useState<"list" | "flowchart">("list");
  const [sidebarTab, setSidebarTab] = useState<
    "commands" | "assets" | "scenarios" | "characters" | "translations"
  >("commands");
  const [highlightedIndices, setHighlightedIndices] = useState<number[]>([]);
  const [showProjectWizard, setShowProjectWizard] = useState(false);
  const [showProjectSettings, setShowProjectSettings] = useState(false);
  const [showExportWizard, setShowExportWizard] = useState(false);

  // Compute base directory for asset loading
  const baseDir = useMemo(() => {
    if (mode.type === "project" && project) {
      return project.root_path;
    }
    if (!filePath) {return null;}
    const lastSlash = filePath.lastIndexOf("/");
    return lastSlash >= 0 ? filePath.substring(0, lastSlash) : null;
  }, [mode.type, project, filePath]);

  // Preview/Playtest mode toggle
  const [previewMode, setPreviewMode] = useState<"preview" | "playtest">("preview");

  // Preview state
  const {
    state: previewState,
    backgroundUrl,
    characterUrl,
    language: previewLanguage,
    setLanguage: setPreviewLanguage,
    goto: previewGoto,
    next: previewNext,
    prev: previewPrev,
  } = usePreview(scenario, baseDir, selectedIndex);

  // Playtest state
  const {
    state: playtestState,
    backgroundUrl: playtestBackgroundUrl,
    characterUrl: playtestCharacterUrl,
    assetErrors: playtestAssetErrors,
    language: playtestLanguage,
    isAutoMode: playtestIsAutoMode,
    isSkipMode: playtestIsSkipMode,
    start: startPlaytest,
    stop: stopPlaytest,
    advance: playtestAdvance,
    selectChoice: playtestSelectChoice,
    rollback: playtestRollback,
    rollbackSteps: playtestRollbackSteps,
    jumpToLabel: playtestJumpToLabel,
    setVariable: playtestSetVariable,
    restart: playtestRestart,
    setLanguage: setPlaytestLanguage,
    submitInput: playtestSubmitInput,
    toggleAutoMode: playtestToggleAutoMode,
    toggleSkipMode: playtestToggleSkipMode,
    save: playtestSave,
    load: playtestLoad,
  } = usePlaytest(scenario, baseDir);

  // Handle mode toggle
  const handleTogglePreviewMode = useCallback(async () => {
    if (previewMode === "preview") {
      await startPlaytest(previewLanguage);
      setPreviewMode("playtest");
    } else {
      await stopPlaytest();
      setPreviewMode("preview");
    }
  }, [previewMode, startPlaytest, stopPlaytest, previewLanguage]);

  // Keyboard shortcuts for playtest mode
  const playtestChoiceCount =
    previewMode === "playtest" &&
    playtestState?.display.type === "choices"
      ? playtestState.display.choices.length
      : 0;

  usePlaytestKeyboard({
    isActive: previewMode === "playtest" && playtestState !== null,
    canRollback: playtestState?.can_rollback ?? false,
    isEnded: playtestState?.is_ended ?? false,
    hasChoices: playtestChoiceCount > 0,
    choiceCount: playtestChoiceCount,
    onAdvance: () => void playtestAdvance(),
    onRollback: () => void playtestRollback(),
    onSelectChoice: (index) => void playtestSelectChoice(index),
    onRestart: () => void playtestRestart(),
    onToggleAutoMode: playtestToggleAutoMode,
    onToggleSkipMode: playtestToggleSkipMode,
    onSave: () => void playtestSave(),
    onLoad: () => void playtestLoad(),
  });

  // Sync: when preview navigates, update editor selection
  const handlePreviewGoto = useCallback(
    (index: number) => {
      previewGoto(index);
      selectCommand(index);
    },
    [previewGoto, selectCommand]
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
        ext || ""
      );
      const isAudio = ["mp3", "ogg", "wav", "flac"].includes(ext || "");

      if (isImage) {
        void updateCommand(selectedIndex, { ...cmd, background: path });
      } else if (isAudio) {
        void updateCommand(selectedIndex, { ...cmd, bgm: path });
      }

      setSidebarTab("commands");
    },
    [selectedIndex, scenario, updateCommand]
  );

  const handleShowUsages = useCallback(
    (indices: number[]) => {
      setHighlightedIndices(indices);
      setSidebarTab("commands");
      if (indices.length > 0) {
        selectCommand(indices[0]);
      }
    },
    [selectCommand]
  );

  const labels = useMemo(() => {
    if (!scenario) {return [];}
    return scenario.script
      .filter((cmd) => cmd.label)
      .map((cmd) => cmd.label as string);
  }, [scenario]);

  const selectedCommand = useMemo(() => {
    if (!scenario || selectedIndex === null) {return null;}
    return scenario.script[selectedIndex];
  }, [scenario, selectedIndex]);

  useEffect(() => {
    if (scenario) {
      void validate();
    }
  }, [scenario, validate]);

  // Project mode handlers
  const handleCreateProject = async (
    rootPath: string,
    name: string,
    author: string,
    description: string,
    resolution: Resolution
  ) => {
    await createProject(rootPath, name, author, description, resolution);
    addRecentProject(rootPath, name);
    setShowProjectWizard(false);
    setMode({ type: "project" });
  };

  const handleOpenProject = async () => {
    await openProject();
    if (project) {
      addRecentProject(project.root_path, project.config.name);
      setMode({ type: "project" });
    }
  };

  const handleOpenRecentProject = async (path: string) => {
    try {
      await openProjectFromPath(path);
      setMode({ type: "project" });
    } catch (e) {
      console.error("Failed to open project:", e);
      removeRecentProject(path);
      const errorMessage = e instanceof Error ? e.message : String(e);
      showToast(`Failed to open project: ${errorMessage}`, "error");
    }
  };

  const handleOpenFile = async () => {
    await openFile();
    if (filePath) {
      setMode({ type: "standalone" });
    }
  };

  const handleBackToWelcome = () => {
    closeProject();
    setMode({ type: "welcome" });
  };

  const handleSaveProjectConfig = async (config: ProjectConfig) => {
    updateConfig(config);
    await saveProject();
    setShowProjectSettings(false);
  };

  // Save characters when dirty
  const handleSaveCharacters = useCallback(async () => {
    if (!project || !charactersDirty) {return;}
    try {
      await saveCharacters(project.root_path);
    } catch (e) {
      console.error("Failed to save characters:", e);
    }
  }, [project, charactersDirty, saveCharacters]);

  // Auto-save characters when switching away from characters tab
  useEffect(() => {
    if (sidebarTab !== "characters" && charactersDirty && project) {
      void handleSaveCharacters();
    }
  }, [sidebarTab, charactersDirty, project, handleSaveCharacters]);

  // Update mode when project is loaded
  useEffect(() => {
    if (project && mode.type === "welcome") {
      addRecentProject(project.root_path, project.config.name);
      setMode({ type: "project" });
    }
  }, [project, mode.type, addRecentProject]);

  // Load characters when project is opened
  useEffect(() => {
    if (project) {
      void loadCharacters(project.root_path);
    }
  }, [project, loadCharacters]);

  // Update mode when file is loaded in standalone mode
  useEffect(() => {
    if (filePath && mode.type === "welcome") {
      setMode({ type: "standalone" });
    }
  }, [filePath, mode.type]);

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

  // Welcome screen
  if (mode.type === "welcome" && !scenario && !project) {
    return (
      <>
        <WelcomeScreen
          recentProjects={recentProjects}
          onNewProject={() => setShowProjectWizard(true)}
          onOpenProject={() => void handleOpenProject()}
          onOpenRecentProject={(path) => void handleOpenRecentProject(path)}
          onRemoveRecentProject={removeRecentProject}
          onOpenFile={() => void handleOpenFile()}
        />
        {showProjectWizard && (
          <ProjectWizard
            onClose={() => setShowProjectWizard(false)}
            onCreate={handleCreateProject}
          />
        )}
      </>
    );
  }

  const title =
    mode.type === "project" && project
      ? `${project.config.name}${projectIsDirty ? " *" : ""}`
      : filePath
        ? `${filePath.split("/").pop()}${isDirty ? " *" : ""}`
        : scenario
          ? `Untitled${isDirty ? " *" : ""}`
          : "ivy Editor";

  return (
    <div className="app">
      {/* Header */}
      <header className="app-header">
        <div className="header-left">
          <button className="back-button" onClick={handleBackToWelcome}>
            ←
          </button>
          <div className="header-title">{title}</div>
        </div>
        <div className="header-actions">
          {mode.type === "project" && (
            <>
              <button onClick={() => setShowProjectSettings(true)}>
                {t("header.settings")}
              </button>
              <button onClick={() => setShowExportWizard(true)}>
                {t("header.export")}
              </button>
            </>
          )}
          {mode.type === "standalone" && (
            <>
              <button onClick={handleNew}>{t("header.new")}</button>
              <button onClick={() => void openFile()}>{t("header.open")}</button>
            </>
          )}
          <button onClick={() => void saveFile()} disabled={!scenario}>
            {t("header.save")}
          </button>
          <button onClick={() => void saveFileAs()} disabled={!scenario}>
            {t("header.saveAs")}
          </button>
          <button onClick={() => void validate()} disabled={!scenario}>
            {t("header.validate")}
          </button>
          <div className="view-toggle">
            <button
              className={view === "list" ? "active" : ""}
              onClick={() => setView("list")}
              disabled={!scenario}
            >
              {t("header.list")}
            </button>
            <button
              className={view === "flowchart" ? "active" : ""}
              onClick={() => setView("flowchart")}
              disabled={!scenario}
            >
              {t("header.flowchart")}
            </button>
          </div>
          <LanguageSwitcher />
        </div>
      </header>

      {/* Main Content */}
      <main className="app-main">
        <Group orientation="horizontal">
          {/* Left Panel: Scenarios / Commands / Assets */}
          <Panel defaultSize={20} minSize={15} maxSize={40}>
            <div className="panel panel-left">
          {mode.type === "project" && project ? (
            <>
              <div className="sidebar-tabs">
                <Tooltip content={t("sidebar.scenariosTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "scenarios" ? "active" : ""}
                    onClick={() => setSidebarTab("scenarios")}
                  >
                    📄 {t("sidebar.scenarios")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.commandsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "commands" ? "active" : ""}
                    onClick={() => setSidebarTab("commands")}
                  >
                    📋 {t("sidebar.commands")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.assetsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "assets" ? "active" : ""}
                    onClick={() => setSidebarTab("assets")}
                  >
                    📁 {t("sidebar.assets")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.charactersTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "characters" ? "active" : ""}
                    onClick={() => setSidebarTab("characters")}
                  >
                    👥 {t("sidebar.characters")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.translationsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "translations" ? "active" : ""}
                    onClick={() => setSidebarTab("translations")}
                  >
                    🌐 {t("sidebar.translations")}
                  </button>
                </Tooltip>
              </div>
              {sidebarTab === "scenarios" ? (
                <ScenarioList
                  scenarios={project.config.scenarios}
                  activeScenarioPath={activeScenarioPath}
                  entryScenario={project.config.entry_scenario}
                  onSelect={setActiveScenario}
                  onAdd={() => {
                    /* TODO: Add scenario dialog */
                  }}
                />
              ) : sidebarTab === "commands" && scenario ? (
                view === "list" ? (
                  <CommandList
                    commands={scenario.script}
                    selectedIndex={selectedIndex}
                    highlightedIndices={highlightedIndices}
                    characterDatabase={characterDatabase}
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
              ) : sidebarTab === "assets" ? (
                <AssetBrowser
                  baseDir={baseDir}
                  scenario={scenario}
                  onSelectAsset={handleSelectAsset}
                  onShowUsages={handleShowUsages}
                />
              ) : sidebarTab === "characters" ? (
                <CharacterDatabase
                  database={characterDatabase}
                  selectedCharacter={selectedCharacter}
                  baseDir={baseDir}
                  scenario={scenario}
                  onSelectCharacter={selectCharacter}
                  onAddCharacter={addCharacter}
                  onUpdateCharacter={updateCharacter}
                  onRemoveCharacter={removeCharacter}
                  onAddLayer={addLayer}
                  onUpdateLayer={updateLayer}
                  onRemoveLayer={removeLayer}
                  onReorderLayers={reorderLayers}
                />
              ) : sidebarTab === "translations" ? (
                <TranslationTable
                  scenario={scenario}
                  onUpdateCommand={updateCommand}
                  onSelectCommand={selectCommand}
                />
              ) : (
                <div className="empty-state">
                  <p>{t("emptyState.selectScenario")}</p>
                </div>
              )}
            </>
          ) : scenario ? (
            <>
              <div className="sidebar-tabs">
                <Tooltip content={t("sidebar.commandsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "commands" ? "active" : ""}
                    onClick={() => setSidebarTab("commands")}
                  >
                    📋 {t("sidebar.commands")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.assetsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "assets" ? "active" : ""}
                    onClick={() => setSidebarTab("assets")}
                  >
                    📁 {t("sidebar.assets")}
                  </button>
                </Tooltip>
                <Tooltip content={t("sidebar.translationsTooltip")} position="bottom">
                  <button
                    className={sidebarTab === "translations" ? "active" : ""}
                    onClick={() => setSidebarTab("translations")}
                  >
                    🌐 {t("sidebar.translations")}
                  </button>
                </Tooltip>
              </div>
              {sidebarTab === "commands" ? (
                view === "list" ? (
                  <CommandList
                    commands={scenario.script}
                    selectedIndex={selectedIndex}
                    highlightedIndices={highlightedIndices}
                    characterDatabase={characterDatabase}
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
              ) : sidebarTab === "assets" ? (
                <AssetBrowser
                  baseDir={baseDir}
                  scenario={scenario}
                  onSelectAsset={handleSelectAsset}
                  onShowUsages={handleShowUsages}
                />
              ) : (
                <TranslationTable
                  scenario={scenario}
                  onUpdateCommand={updateCommand}
                  onSelectCommand={selectCommand}
                />
              )}
            </>
          ) : (
            <div className="empty-state">
              <p>{t("emptyState.noScenarioLoaded")}</p>
              <button onClick={handleNew}>{t("emptyState.createNew")}</button>
              <button onClick={() => void openFile()}>{t("emptyState.openFile")}</button>
            </div>
          )}
            </div>
          </Panel>

          <Separator className="resize-handle" />

          {/* Center Panel: Editor */}
          <Panel defaultSize={50} minSize={30}>
            <div className="panel panel-center">
          {scenario && selectedCommand ? (
            <>
              <div className="tab-bar">
                <button
                  className={activeTab === "form" ? "active" : ""}
                  onClick={() => setActiveTab("form")}
                >
                  Form
                </button>
                <button
                  className={activeTab === "yaml" ? "active" : ""}
                  onClick={() => setActiveTab("yaml")}
                >
                  YAML
                </button>
              </div>
              {activeTab === "form" ? (
                <CommandForm
                  command={selectedCommand}
                  labels={labels}
                  baseDir={baseDir}
                  characterDatabase={characterDatabase}
                  onChange={(cmd) => {
                    void updateCommand(selectedIndex!, cmd);
                  }}
                />
              ) : (
                <YamlPreview yaml={yamlPreview} />
              )}
            </>
          ) : scenario ? (
            <div className="empty-state">
              <p>{t("emptyState.selectCommand")}</p>
            </div>
          ) : null}
            </div>
          </Panel>

          <Separator className="resize-handle" />

          {/* Right Panel: Preview + Validation */}
          <Panel defaultSize={30} minSize={20} maxSize={45}>
            <div className="panel panel-right">
              <div className="preview-mode-toggle">
                <Tooltip content={t("previewPanel.previewDescription")} position="bottom">
                  <button
                    className={previewMode === "preview" ? "active" : ""}
                    onClick={() => previewMode !== "preview" && void handleTogglePreviewMode()}
                  >
                    📖 {t("previewPanel.preview")}
                  </button>
                </Tooltip>
                <Tooltip content={t("previewPanel.playtestDescription")} position="bottom">
                  <button
                    className={previewMode === "playtest" ? "active" : ""}
                    onClick={() => previewMode !== "playtest" && void handleTogglePreviewMode()}
                    disabled={!scenario}
                  >
                    ▶️ {t("previewPanel.playtest")}
                  </button>
                </Tooltip>
              </div>
              <Group orientation="vertical">
                <Panel defaultSize={50} minSize={30}>
                  <div className="right-section preview-section">
                    {previewMode === "preview" ? (
                      <PreviewPanel
                        mode="preview"
                        state={previewState}
                        backgroundUrl={backgroundUrl}
                        characterUrl={characterUrl}
                        baseDir={baseDir}
                        language={previewLanguage}
                        onLanguageChange={setPreviewLanguage}
                        onPrev={previewPrev}
                        onNext={previewNext}
                        onGoto={handlePreviewGoto}
                      />
                    ) : (
                      <>
                        <PreviewPanel
                          mode="playtest"
                          state={playtestState}
                          backgroundUrl={playtestBackgroundUrl}
                          characterUrl={playtestCharacterUrl}
                          assetErrors={playtestAssetErrors}
                          baseDir={baseDir}
                          language={playtestLanguage}
                          isAutoMode={playtestIsAutoMode}
                          isSkipMode={playtestIsSkipMode}
                          onLanguageChange={(lang) => void setPlaytestLanguage(lang)}
                          onAdvance={() => void playtestAdvance()}
                          onSelectChoice={(index) => void playtestSelectChoice(index)}
                          onRollback={() => void playtestRollback()}
                          onRestart={() => void playtestRestart()}
                          onSubmitInput={(value) => void playtestSubmitInput(value)}
                          onToggleAutoMode={playtestToggleAutoMode}
                          onToggleSkipMode={playtestToggleSkipMode}
                        />
                        <PlaytestDebugPanel
                          state={playtestState}
                          onJumpToLabel={(label) => void playtestJumpToLabel(label)}
                          onSetVariable={(name, value) => void playtestSetVariable(name, value)}
                          onRollbackToIndex={(steps) => void playtestRollbackSteps(steps)}
                        />
                      </>
                    )}
                  </div>
                </Panel>
                <Separator className="resize-handle vertical" />
                <Panel defaultSize={50} minSize={20}>
                  <div className="right-section tools-section">
                    <VariableWatcher
                      scenario={scenario}
                      variables={
                        previewMode === "playtest" && playtestState
                          ? (playtestState.variables as Record<string, string>)
                          : previewState?.variables || {}
                      }
                      currentIndex={
                        previewMode === "playtest" && playtestState
                          ? playtestState.command_index
                          : previewState?.command_index || 0
                      }
                    />
                    <StoryPathAnalyzer
                      scenario={scenario}
                      onSelectCommand={selectCommand}
                    />
                    <SaveDataValidator baseDir={baseDir} />
                    <ValidationErrors
                      result={validationResult}
                      onSelectCommand={selectCommand}
                    />
                  </div>
                </Panel>
              </Group>
            </div>
          </Panel>
        </Group>
      </main>

      {/* New Scenario Dialog */}
      {showNewDialog && (
        <div className="dialog-overlay">
          <div className="dialog">
            <h2>{t("dialog.newScenario")}</h2>
            <input
              type="text"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              placeholder={t("dialog.scenarioTitle")}
              autoFocus
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleCreateNew();
                }
              }}
            />
            <div className="dialog-actions">
              <button onClick={() => setShowNewDialog(false)}>{t("common.cancel")}</button>
              <button onClick={handleCreateNew} disabled={!newTitle.trim()}>
                {t("common.create")}
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Project Settings Dialog */}
      {showProjectSettings && project && (
        <ProjectSettings
          config={project.config}
          onClose={() => setShowProjectSettings(false)}
          onSave={handleSaveProjectConfig}
        />
      )}

      {/* Export Wizard Dialog */}
      {showExportWizard && project && (
        <ExportWizard
          projectPath={project.root_path}
          projectName={project.config.name}
          onClose={() => setShowExportWizard(false)}
        />
      )}
    </div>
  );
};

export default App;
