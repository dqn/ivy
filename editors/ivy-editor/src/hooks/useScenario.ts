import { useState, useCallback } from "react";
import { open, save } from "@tauri-apps/plugin-dialog";
import type { Command, Scenario, ValidationResult } from "../types/scenario";
import { invokeCommand, invokeCommandSafe } from "../lib";

type ShowToast = (message: string, type?: "error" | "success" | "info" | "warning") => void;

interface UseScenarioOptions {
  showToast?: ShowToast;
}

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
  updateCommand: (index: number, command: Command) => Promise<void>;
  addCommand: (afterIndex?: number) => Promise<void>;
  removeCommand: (index: number) => Promise<void>;
  reorderCommand: (fromIndex: number, toIndex: number) => Promise<void>;

  // Validation
  validate: () => Promise<void>;
}

function createEmptyCommand(): Command {
  return {};
}

export function useScenario(options: UseScenarioOptions = {}): UseScenarioReturn {
  const { showToast } = options;
  const [scenario, setScenario] = useState<Scenario | null>(null);
  const [filePath, setFilePath] = useState<string | null>(null);
  const [isDirty, setIsDirty] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);
  const [validationResult, setValidationResult] =
    useState<ValidationResult | null>(null);
  const [yamlPreview, setYamlPreview] = useState("");

  const updateYamlPreview = useCallback(async (s: Scenario) => {
    const yaml = await invokeCommandSafe<string>("scenario_to_yaml", {
      scenario: s,
    });
    if (yaml !== null) {
      setYamlPreview(yaml);
    }
  }, []);

  const openFile = useCallback(async () => {
    const selected = await open({
      multiple: false,
      filters: [{ name: "Scenario", extensions: ["yaml", "yml"] }],
    });

    if (selected) {
      const loaded = await invokeCommandSafe<Scenario>("load_scenario", {
        path: selected,
      }, { showToast });
      if (loaded) {
        setScenario(loaded);
        setFilePath(selected);
        setIsDirty(false);
        setSelectedIndex(null);
        setValidationResult(null);
        await updateYamlPreview(loaded);
      }
    }
  }, [updateYamlPreview, showToast]);

  const saveFile = useCallback(async () => {
    if (!scenario) {return;}

    if (!filePath) {
      await saveFileAs();
      return;
    }

    const result = await invokeCommandSafe("save_scenario", {
      path: filePath,
      scenario,
    }, { showToast });
    if (result !== null) {
      setIsDirty(false);
    }
  }, [scenario, filePath, showToast]);

  const saveFileAs = useCallback(async () => {
    if (!scenario) {return;}

    const selected = await save({
      filters: [{ name: "Scenario", extensions: ["yaml", "yml"] }],
      defaultPath: filePath ?? undefined,
    });

    if (selected) {
      const result = await invokeCommandSafe("save_scenario", {
        path: selected,
        scenario,
      }, { showToast });
      if (result !== null) {
        setFilePath(selected);
        setIsDirty(false);
      }
    }
  }, [scenario, filePath, showToast]);

  const newScenario = useCallback(
    async (title: string) => {
      const created = await invokeCommand<Scenario>("create_empty_scenario", {
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
      if (!scenario) {return;}

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
      if (!scenario) {return;}

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
      if (!scenario) {return;}

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

  const reorderCommand = useCallback(
    async (fromIndex: number, toIndex: number) => {
      if (!scenario) {return;}
      if (fromIndex === toIndex) {return;}

      const newScript = [...scenario.script];
      const [removed] = newScript.splice(fromIndex, 1);
      newScript.splice(toIndex, 0, removed);

      const newScenario = { ...scenario, script: newScript };
      setScenario(newScenario);
      setIsDirty(true);

      // Update selection to follow the moved item
      if (selectedIndex !== null) {
        if (selectedIndex === fromIndex) {
          setSelectedIndex(toIndex);
        } else if (fromIndex < selectedIndex && toIndex >= selectedIndex) {
          setSelectedIndex(selectedIndex - 1);
        } else if (fromIndex > selectedIndex && toIndex <= selectedIndex) {
          setSelectedIndex(selectedIndex + 1);
        }
      }

      await updateYamlPreview(newScenario);
    },
    [scenario, selectedIndex, updateYamlPreview]
  );

  const validate = useCallback(async () => {
    if (!scenario) {return;}

    const result = await invokeCommandSafe<ValidationResult>("validate", {
      scenario,
    });
    if (result !== null) {
      setValidationResult(result);
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
    reorderCommand,
    validate,
  };
}
