import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Project, ProjectConfig, Resolution } from "../types/project";

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

export function useProject(): UseProjectReturn {
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
      try {
        const created = await invoke<Project>("create_project", {
          rootPath,
          name,
          author,
          description,
          resolution,
        });
        setProject(created);
        setIsDirty(false);
        setActiveScenarioPath(created.config.entry_scenario);
      } catch (e) {
        console.error("Failed to create project:", e);
        throw e;
      }
    },
    []
  );

  const openProject = useCallback(async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      try {
        const loaded = await invoke<Project>("load_project", {
          projectPath: selected,
        });
        setProject(loaded);
        setIsDirty(false);
        setActiveScenarioPath(loaded.config.entry_scenario);
      } catch (e) {
        console.error("Failed to load project:", e);
        alert(`Failed to load project: ${e}`);
      }
    }
  }, []);

  const openProjectFromPath = useCallback(async (path: string) => {
    try {
      const loaded = await invoke<Project>("load_project", {
        projectPath: path,
      });
      setProject(loaded);
      setIsDirty(false);
      setActiveScenarioPath(loaded.config.entry_scenario);
    } catch (e) {
      console.error("Failed to load project:", e);
      throw e;
    }
  }, []);

  const saveProject = useCallback(async () => {
    if (!project) return;

    try {
      await invoke("save_project", {
        rootPath: project.root_path,
        config: project.config,
      });
      setIsDirty(false);
    } catch (e) {
      console.error("Failed to save project:", e);
      alert(`Failed to save project: ${e}`);
    }
  }, [project]);

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

      try {
        const updatedConfig = await invoke<ProjectConfig>(
          "add_scenario_to_project",
          {
            rootPath: project.root_path,
            scenarioPath,
            chapterName,
          }
        );
        setProject({ ...project, config: updatedConfig });
      } catch (e) {
        console.error("Failed to add scenario:", e);
        throw e;
      }
    },
    [project]
  );

  const removeScenario = useCallback(
    async (scenarioPath: string) => {
      if (!project) return;

      try {
        const updatedConfig = await invoke<ProjectConfig>(
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
      } catch (e) {
        console.error("Failed to remove scenario:", e);
        throw e;
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
