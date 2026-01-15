import { useState, useCallback, useEffect } from "react";
import type { RecentProject } from "../types/project";

const STORAGE_KEY = "ivy-recent-projects";
const MAX_RECENT_PROJECTS = 10;

function loadRecentProjects(): RecentProject[] {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return JSON.parse(stored);
    }
  } catch (e) {
    console.error("Failed to load recent projects:", e);
  }
  return [];
}

function saveRecentProjects(projects: RecentProject[]): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(projects));
  } catch (e) {
    console.error("Failed to save recent projects:", e);
  }
}

interface UseRecentProjectsReturn {
  recentProjects: RecentProject[];
  addRecentProject: (path: string, name: string) => void;
  removeRecentProject: (path: string) => void;
  clearRecentProjects: () => void;
}

export function useRecentProjects(): UseRecentProjectsReturn {
  const [recentProjects, setRecentProjects] = useState<RecentProject[]>(() =>
    loadRecentProjects()
  );

  useEffect(() => {
    saveRecentProjects(recentProjects);
  }, [recentProjects]);

  const addRecentProject = useCallback((path: string, name: string) => {
    setRecentProjects((prev) => {
      // Remove existing entry for this path.
      const filtered = prev.filter((p) => p.path !== path);

      // Add to the beginning.
      const newProject: RecentProject = {
        path,
        name,
        last_opened: Date.now(),
      };

      // Keep only the most recent projects.
      return [newProject, ...filtered].slice(0, MAX_RECENT_PROJECTS);
    });
  }, []);

  const removeRecentProject = useCallback((path: string) => {
    setRecentProjects((prev) => prev.filter((p) => p.path !== path));
  }, []);

  const clearRecentProjects = useCallback(() => {
    setRecentProjects([]);
  }, []);

  return {
    recentProjects,
    addRecentProject,
    removeRecentProject,
    clearRecentProjects,
  };
}
