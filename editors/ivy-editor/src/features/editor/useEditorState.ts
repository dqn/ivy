import { useState, useMemo, useCallback } from "react";

export type EditorMode =
  | { type: "welcome" }
  | { type: "project" }
  | { type: "standalone" };

export type SidebarTab =
  | "commands"
  | "assets"
  | "scenarios"
  | "characters"
  | "translations";

export interface EditorUIState {
  mode: EditorMode;
  activeTab: "form" | "yaml";
  view: "list" | "flowchart";
  sidebarTab: SidebarTab;
  previewMode: "preview" | "playtest";
  highlightedIndices: number[];
}

export interface DialogState {
  showNewDialog: boolean;
  showProjectWizard: boolean;
  showProjectSettings: boolean;
  showExportWizard: boolean;
  newTitle: string;
}

export function useEditorUIState() {
  const [mode, setMode] = useState<EditorMode>({ type: "welcome" });
  const [activeTab, setActiveTab] = useState<"form" | "yaml">("form");
  const [view, setView] = useState<"list" | "flowchart">("list");
  const [sidebarTab, setSidebarTab] = useState<SidebarTab>("commands");
  const [previewMode, setPreviewMode] = useState<"preview" | "playtest">(
    "preview"
  );
  const [highlightedIndices, setHighlightedIndices] = useState<number[]>([]);

  const state: EditorUIState = useMemo(
    () => ({
      mode,
      activeTab,
      view,
      sidebarTab,
      previewMode,
      highlightedIndices,
    }),
    [mode, activeTab, view, sidebarTab, previewMode, highlightedIndices]
  );

  return {
    state,
    setMode,
    setActiveTab,
    setView,
    setSidebarTab,
    setPreviewMode,
    setHighlightedIndices,
  };
}

export function useDialogState() {
  const [showNewDialog, setShowNewDialog] = useState(false);
  const [showProjectWizard, setShowProjectWizard] = useState(false);
  const [showProjectSettings, setShowProjectSettings] = useState(false);
  const [showExportWizard, setShowExportWizard] = useState(false);
  const [newTitle, setNewTitle] = useState("");

  const state: DialogState = useMemo(
    () => ({
      showNewDialog,
      showProjectWizard,
      showProjectSettings,
      showExportWizard,
      newTitle,
    }),
    [showNewDialog, showProjectWizard, showProjectSettings, showExportWizard, newTitle]
  );

  const openNewDialog = useCallback(() => {
    setNewTitle("");
    setShowNewDialog(true);
  }, []);

  const closeNewDialog = useCallback(() => {
    setShowNewDialog(false);
  }, []);

  return {
    state,
    setShowNewDialog,
    setShowProjectWizard,
    setShowProjectSettings,
    setShowExportWizard,
    setNewTitle,
    openNewDialog,
    closeNewDialog,
  };
}
