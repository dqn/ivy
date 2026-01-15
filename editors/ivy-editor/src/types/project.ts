export interface Resolution {
  width: number;
  height: number;
}

export interface ScenarioRef {
  path: string;
  chapter?: string;
}

export interface ProjectConfig {
  name: string;
  version: string;
  author: string;
  description: string;
  resolution: Resolution;
  entry_scenario: string;
  scenarios: ScenarioRef[];
}

export interface Project {
  config: ProjectConfig;
  root_path: string;
}

export interface RecentProject {
  path: string;
  name: string;
  last_opened: number;
}

export const DEFAULT_RESOLUTION: Resolution = {
  width: 1920,
  height: 1080,
};

export const RESOLUTION_PRESETS: { label: string; resolution: Resolution }[] = [
  { label: "1920x1080 (Full HD)", resolution: { width: 1920, height: 1080 } },
  { label: "1280x720 (HD)", resolution: { width: 1280, height: 720 } },
  { label: "2560x1440 (2K)", resolution: { width: 2560, height: 1440 } },
  { label: "3840x2160 (4K)", resolution: { width: 3840, height: 2160 } },
];

export function createDefaultProjectConfig(name: string): ProjectConfig {
  return {
    name,
    version: "1.0.0",
    author: "",
    description: "",
    resolution: DEFAULT_RESOLUTION,
    entry_scenario: "scenarios/main.ivy.yaml",
    scenarios: [
      {
        path: "scenarios/main.ivy.yaml",
        chapter: "Main",
      },
    ],
  };
}
