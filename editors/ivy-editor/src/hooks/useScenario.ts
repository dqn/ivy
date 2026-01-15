import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { Command, Scenario, ValidationResult } from "../types/scenario";

interface UseScenarioReturn {
  scenario: Scenario | null;
  filePath: string | null;
  isDirty: boolean;
  selectedIndex: number | null;
  validationResult: ValidationResult | null;
  yamlPreview: string;

  // File operations
  openFile: () => Promise<void>;
  saveFile: () => Promise<void>;
  saveFileAs: () => Promise<void>;
  newScenario: (title: string) => Promise<void>;

  // Command operations
  selectCommand: (index: number | null) => void;
  updateCommand: (index: number, command: Command) => void;
  addCommand: (afterIndex?: number) => void;
  removeCommand: (index: number) => void;

  // Validation
  validate: () => Promise<void>;
}

function createEmptyCommand(): Command {
  return {};
}

export function useScenario(): UseScenarioReturn {
  const [scenario, setScenario] = useState<Scenario | null>(null);
  const [filePath, setFilePath] = useState<string | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [validationResult, setValidationResult] =
    useState<ValidationResult | null>(null);
  const [yamlPreview, setYamlPreview] = useState("");

  const updateYamlPreview = useCallback(async (s: Scenario) => {
    try {
      const yaml = await invoke<string>("scenario_to_yaml", { scenario: s });
      setYamlPreview(yaml);
    } catch (e) {
      console.error("Failed to generate YAML preview:", e);
    }
  }, []);

  const openFile = useCallback(async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Scenario", extensions: ["yaml", "yml"] }],
    });

    if (selected) {
      try {
        const loaded = await invoke<Scenario>("load_scenario", {
          path: selected,
        });
        setScenario(loaded);
        setFilePath(selected);
        setIsDirty(false);
        setSelectedIndex(null);
        setValidationResult(null);
        await updateYamlPreview(loaded);
      } catch (e) {
        console.error("Failed to load scenario:", e);
        alert(`Failed to load: ${e}`);
      }
    }
  }, [updateYamlPreview]);

  const saveFile = useCallback(async () => {
    if (!scenario) return;

    if (!filePath) {
      await saveFileAs();
      return;
    }

    try {
      await invoke("save_scenario", { path: filePath, scenario });
      setIsDirty(false);
    } catch (e) {
      console.error("Failed to save scenario:", e);
      alert(`Failed to save: ${e}`);
    }
  }, [scenario, filePath]);

  const saveFileAs = useCallback(async () => {
    if (!scenario) return;

    const selected = await save({
      filters: [{ name: "Scenario", extensions: ["yaml", "yml"] }],
      defaultPath: filePath ?? undefined,
    });

    if (selected) {
      try {
        await invoke("save_scenario", { path: selected, scenario });
        setFilePath(selected);
        setIsDirty(false);
      } catch (e) {
        console.error("Failed to save scenario:", e);
        alert(`Failed to save: ${e}`);
      }
    }
  }, [scenario, filePath]);

  const newScenario = useCallback(
    async (title: string) => {
      const created = await invoke<Scenario>("create_empty_scenario", {
        title,
      });
      setScenario(created);
      setFilePath(null);
      setIsDirty(false);
      setSelectedIndex(null);
      setValidationResult(null);
      await updateYamlPreview(created);
    },
    [updateYamlPreview]
  );

  const selectCommand = useCallback((index: number | null) => {
    setSelectedIndex(index);
  }, []);

  const updateCommand = useCallback(
    async (index: number, command: Command) => {
      if (!scenario) return;

      const newScript = [...scenario.script];
      newScript[index] = command;
      const newScenario = { ...scenario, script: newScript };
      setScenario(newScenario);
      setIsDirty(true);
      await updateYamlPreview(newScenario);
    },
    [scenario, updateYamlPreview]
  );

  const addCommand = useCallback(
    async (afterIndex?: number) => {
      if (!scenario) return;

      const newCommand = createEmptyCommand();
      const newScript = [...scenario.script];

      if (afterIndex !== undefined) {
        newScript.splice(afterIndex + 1, 0, newCommand);
        setSelectedIndex(afterIndex + 1);
      } else {
        newScript.push(newCommand);
        setSelectedIndex(newScript.length - 1);
      }

      const newScenario = { ...scenario, script: newScript };
      setScenario(newScenario);
      setIsDirty(true);
      await updateYamlPreview(newScenario);
    },
    [scenario, updateYamlPreview]
  );

  const removeCommand = useCallback(
    async (index: number) => {
      if (!scenario) return;

      const newScript = scenario.script.filter((_, i) => i !== index);
      const newScenario = { ...scenario, script: newScript };
      setScenario(newScenario);
      setIsDirty(true);

      if (selectedIndex !== null) {
        if (selectedIndex === index) {
          setSelectedIndex(null);
        } else if (selectedIndex > index) {
          setSelectedIndex(selectedIndex - 1);
        }
      }

      await updateYamlPreview(newScenario);
    },
    [scenario, selectedIndex, updateYamlPreview]
  );

  const validate = useCallback(async () => {
    if (!scenario) return;

    try {
      const result = await invoke<ValidationResult>("validate", { scenario });
      setValidationResult(result);
    } catch (e) {
      console.error("Failed to validate:", e);
    }
  }, [scenario]);

  return {
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
  };
}
