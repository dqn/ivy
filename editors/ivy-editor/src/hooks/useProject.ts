import { useState, useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import type { Project, ProjectConfig, Resolution } from "../types/project";
import { invokeCommand, invokeCommandSafe } from "../lib";

type ShowToast = (message: string, type?: "error" | "success" | "info" | "warning") => void;

interface UseProjectOptions {
  showToast?: ShowToast;
}

interface UseProjectReturn {
  project: Project | null;
  isDirty: boolean;
  activeScenarioPath: string | null;

  // Project operations
  createProject: (
    rootPath: string,
    name: string,
    author: string,
    description: string,
    resolution: Resolution
  ) => Promise<void>;
  openProject: () => Promise<void>;
  openProjectFromPath: (path: string) => Promise<void>;
  saveProject: () => Promise<void>;
  closeProject: () => void;

  // Config operations
  updateConfig: (config: ProjectConfig) => Promise<void>;

  // Scenario operations
  setActiveScenario: (path: string) => void;
  addScenario: (scenarioPath: string, chapterName?: string) => Promise<void>;
  removeScenario: (scenarioPath: string) => Promise<void>;
}

export function useProject(options: UseProjectOptions = {}): UseProjectReturn {
  const { showToast } = options;
  const [project, setProject] = useState<Project | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [activeScenarioPath, setActiveScenarioPath] = useState<string | null>(
    null
  );

  const createProject = useCallback(
    async (
      rootPath: string,
      name: string,
      author: string,
      description: string,
      resolution: Resolution
    ) => {
      const created = await invokeCommand<Project>("create_project", {
        rootPath,
        name,
        author,
        description,
        resolution,
      });
      setProject(created);
      setIsDirty(false);
      setActiveScenarioPath(created.config.entry_scenario);
    },
    []
  );

  const openProject = useCallback(async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      const loaded = await invokeCommandSafe<Project>("load_project", {
        projectPath: selected,
      }, { showToast });
      if (loaded) {
        setProject(loaded);
        setIsDirty(false);
        setActiveScenarioPath(loaded.config.entry_scenario);
      }
    }
  }, [showToast]);

  const openProjectFromPath = useCallback(async (path: string) => {
    const loaded = await invokeCommand<Project>("load_project", {
      projectPath: path,
    });
    setProject(loaded);
    setIsDirty(false);
    setActiveScenarioPath(loaded.config.entry_scenario);
  }, []);

  const saveProject = useCallback(async () => {
    if (!project) return;

    const result = await invokeCommandSafe("save_project", {
      rootPath: project.root_path,
      config: project.config,
    }, { showToast });
    if (result !== null) {
      setIsDirty(false);
    }
  }, [project, showToast]);

  const closeProject = useCallback(() => {
    setProject(null);
    setIsDirty(false);
    setActiveScenarioPath(null);
  }, []);

  const updateConfig = useCallback(
    async (config: ProjectConfig) => {
      if (!project) return;

      const updatedProject = { ...project, config };
      setProject(updatedProject);
      setIsDirty(true);
    },
    [project]
  );

  const setActiveScenario = useCallback((path: string) => {
    setActiveScenarioPath(path);
  }, []);

  const addScenario = useCallback(
    async (scenarioPath: string, chapterName?: string) => {
      if (!project) return;

      const updatedConfig = await invokeCommand<ProjectConfig>(
        "add_scenario_to_project",
        {
          rootPath: project.root_path,
          scenarioPath,
          chapterName,
        }
      );
      setProject({ ...project, config: updatedConfig });
    },
    [project]
  );

  const removeScenario = useCallback(
    async (scenarioPath: string) => {
      if (!project) return;

      const updatedConfig = await invokeCommand<ProjectConfig>(
        "remove_scenario_from_project",
        {
          rootPath: project.root_path,
          scenarioPath,
        }
      );
      setProject({ ...project, config: updatedConfig });

      // If the removed scenario was active, switch to entry scenario.
      if (activeScenarioPath === scenarioPath) {
        setActiveScenarioPath(updatedConfig.entry_scenario);
      }
    },
    [project, activeScenarioPath]
  );

  return {
    project,
    isDirty,
    activeScenarioPath,
    createProject,
    openProject,
    openProjectFromPath,
    saveProject,
    closeProject,
    updateConfig,
    setActiveScenario,
    addScenario,
    removeScenario,
  };
}
