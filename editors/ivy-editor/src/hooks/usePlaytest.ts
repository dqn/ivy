import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Scenario } from "../types/scenario";
import type { PlaytestState, Value } from "../types/playtest";

export interface AssetError {
  type: "background" | "character";
  path: string;
  message: string;
}

interface UsePlaytestReturn {
  // State
  state: PlaytestState | null;
  isActive: boolean;
  backgroundUrl: string | null;
  characterUrl: string | null;
  assetErrors: AssetError[];
  language: string;
  isAutoMode: boolean;
  isSkipMode: boolean;
  autoModeDelay: number;

  // Actions
  start: (language?: string, scenarioPath?: string) => Promise<void>;
  stop: () => Promise<void>;
  advance: () => Promise<void>;
  selectChoice: (choiceIndex: number) => Promise<void>;
  rollback: () => Promise<void>;
  rollbackSteps: (steps: number) => Promise<void>;
  jumpToLabel: (label: string) => Promise<void>;
  setVariable: (name: string, value: Value) => Promise<void>;
  restart: () => Promise<void>;
  reloadScenario: () => Promise<void>;
  setLanguage: (lang: string) => Promise<void>;
  submitInput: (value: string) => Promise<void>;
  toggleAutoMode: () => void;
  toggleSkipMode: () => void;
  setAutoModeDelay: (delay: number) => void;
  save: (slot?: number) => Promise<void>;
  load: (slot?: number) => Promise<void>;
}

export function usePlaytest(
  scenario: Scenario | null,
  baseDir: string | null
): UsePlaytestReturn {
  const [state, setState] = useState<PlaytestState | null>(null);
  const [isActive, setIsActive] = useState(false);
  const [backgroundUrl, setBackgroundUrl] = useState<string | null>(null);
  const [characterUrl, setCharacterUrl] = useState<string | null>(null);
  const [assetErrors, setAssetErrors] = useState<AssetError[]>([]);
  const [language, setLanguageState] = useState("en");
  const [isAutoMode, setIsAutoMode] = useState(false);
  const [isSkipMode, setIsSkipMode] = useState(false);
  const [autoModeDelay, setAutoModeDelay] = useState(2.0);

  // Extract background and character from display state
  const getVisualState = useCallback(() => {
    if (!state) return { background: null, character: null };

    const display = state.display;
    if (
      display.type === "text" ||
      display.type === "choices" ||
      display.type === "input" ||
      display.type === "wait"
    ) {
      return {
        background: display.background,
        character: display.character,
      };
    }
    return { background: null, character: null };
  }, [state]);

  // Load assets when state changes
  useEffect(() => {
    const { background, character } = getVisualState();

    if (!baseDir) {
      setBackgroundUrl(null);
      setCharacterUrl(null);
      setAssetErrors([]);
      return;
    }

    const loadAssets = async () => {
      const errors: AssetError[] = [];

      if (background) {
        try {
          const url = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: background,
          });
          setBackgroundUrl(url);
        } catch (e) {
          setBackgroundUrl(null);
          errors.push({
            type: "background",
            path: background,
            message: e instanceof Error ? e.message : String(e),
          });
        }
      } else {
        setBackgroundUrl(null);
      }

      if (character) {
        try {
          const url = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: character,
          });
          setCharacterUrl(url);
        } catch (e) {
          setCharacterUrl(null);
          errors.push({
            type: "character",
            path: character,
            message: e instanceof Error ? e.message : String(e),
          });
        }
      } else {
        setCharacterUrl(null);
      }

      setAssetErrors(errors);
    };

    void loadAssets();
  }, [getVisualState, baseDir]);

  const start = useCallback(
    async (lang?: string, scenarioPath?: string) => {
      if (!scenario) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_start", {
          scenario,
          language: lang ?? language,
          scenarioPath: scenarioPath ?? null,
          baseDir: baseDir ?? null,
        });
        setState(newState);
        setIsActive(true);
      } catch (e) {
        console.error("Failed to start playtest:", e);
      }
    },
    [scenario, language, baseDir]
  );

  const stop = useCallback(async () => {
    try {
      await invoke("playtest_stop");
      setState(null);
      setIsActive(false);
    } catch (e) {
      console.error("Failed to stop playtest:", e);
    }
  }, []);

  const advance = useCallback(async () => {
    if (!isActive) return;

    try {
      const newState = await invoke<PlaytestState>("playtest_advance");
      setState(newState);
    } catch (e) {
      console.error("Failed to advance playtest:", e);
    }
  }, [isActive]);

  const selectChoice = useCallback(
    async (choiceIndex: number) => {
      if (!isActive) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_select_choice", {
          choiceIndex,
        });
        setState(newState);
      } catch (e) {
        console.error("Failed to select choice:", e);
      }
    },
    [isActive]
  );

  const rollback = useCallback(async () => {
    if (!isActive) return;

    try {
      const newState = await invoke<PlaytestState>("playtest_rollback");
      setState(newState);
    } catch (e) {
      console.error("Failed to rollback:", e);
    }
  }, [isActive]);

  const rollbackSteps = useCallback(
    async (steps: number) => {
      if (!isActive || steps <= 0) return;

      try {
        let newState: PlaytestState | null = null;
        for (let i = 0; i < steps; i++) {
          newState = await invoke<PlaytestState>("playtest_rollback");
        }
        if (newState) {
          setState(newState);
        }
      } catch (e) {
        console.error("Failed to rollback:", e);
      }
    },
    [isActive]
  );

  const jumpToLabel = useCallback(
    async (label: string) => {
      if (!isActive) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_jump_to_label", {
          label,
        });
        setState(newState);
      } catch (e) {
        console.error("Failed to jump to label:", e);
      }
    },
    [isActive]
  );

  const setVariable = useCallback(
    async (name: string, value: Value) => {
      if (!isActive) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_set_variable", {
          name,
          value,
        });
        setState(newState);
      } catch (e) {
        console.error("Failed to set variable:", e);
      }
    },
    [isActive]
  );

  const restart = useCallback(async () => {
    if (!isActive) return;

    try {
      const newState = await invoke<PlaytestState>("playtest_restart");
      setState(newState);
    } catch (e) {
      console.error("Failed to restart playtest:", e);
    }
  }, [isActive]);

  const reloadScenario = useCallback(async () => {
    if (!isActive || !scenario) return;

    try {
      const newState = await invoke<PlaytestState>(
        "playtest_reload_scenario",
        { scenario }
      );
      setState(newState);
    } catch (e) {
      console.error("Failed to reload scenario:", e);
    }
  }, [isActive, scenario]);

  const setLanguage = useCallback(
    async (lang: string) => {
      setLanguageState(lang);

      if (isActive) {
        try {
          const newState = await invoke<PlaytestState>(
            "playtest_set_language",
            { language: lang }
          );
          setState(newState);
        } catch (e) {
          console.error("Failed to set language:", e);
        }
      }
    },
    [isActive]
  );

  const submitInput = useCallback(
    async (value: string) => {
      if (!isActive) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_submit_input", {
          value,
        });
        setState(newState);
      } catch (e) {
        console.error("Failed to submit input:", e);
      }
    },
    [isActive]
  );

  const toggleAutoMode = useCallback(() => {
    setIsAutoMode((prev) => {
      if (!prev) {
        setIsSkipMode(false);
      }
      return !prev;
    });
  }, []);

  const toggleSkipMode = useCallback(() => {
    setIsSkipMode((prev) => {
      if (!prev) {
        setIsAutoMode(false);
      }
      return !prev;
    });
  }, []);

  const save = useCallback(
    async (slot: number = 1) => {
      if (!isActive) return;

      try {
        await invoke("playtest_save", { slot });
      } catch (e) {
        console.error("Failed to save:", e);
      }
    },
    [isActive]
  );

  const load = useCallback(
    async (slot: number = 1) => {
      if (!isActive) return;

      try {
        const newState = await invoke<PlaytestState>("playtest_load", { slot });
        setState(newState);
      } catch (e) {
        console.error("Failed to load:", e);
      }
    },
    [isActive]
  );

  // Auto mode effect
  useEffect(() => {
    if (!isActive || !isAutoMode || !state) return;

    const displayType = state.display.type;

    // Don't auto-advance on choices, input, video, wait, or end
    if (
      displayType === "choices" ||
      displayType === "input" ||
      displayType === "wait" ||
      displayType === "video" ||
      displayType === "end"
    ) {
      return;
    }

    const timer = setTimeout(() => {
      void advance();
    }, autoModeDelay * 1000);

    return () => clearTimeout(timer);
  }, [isActive, isAutoMode, state, autoModeDelay, advance]);

  // Skip mode effect
  useEffect(() => {
    if (!isActive || !isSkipMode || !state) return;

    const displayType = state.display.type;

    // Stop on choices, input, video, wait, or end
    if (
      displayType === "choices" ||
      displayType === "input" ||
      displayType === "wait" ||
      displayType === "video" ||
      displayType === "end"
    ) {
      setIsSkipMode(false);
      return;
    }

    const timer = setTimeout(() => {
      void advance();
    }, 50);

    return () => clearTimeout(timer);
  }, [isActive, isSkipMode, state, advance]);

  // Stop playtest when scenario changes
  useEffect(() => {
    if (isActive && scenario) {
      void reloadScenario();
    }
  }, [scenario]);

  // Clean up on unmount
  useEffect(() => {
    return () => {
      if (isActive) {
        void invoke("playtest_stop").catch(() => {
          // Ignore errors on cleanup
        });
      }
    };
  }, [isActive]);

  return {
    state,
    isActive,
    backgroundUrl,
    characterUrl,
    assetErrors,
    language,
    isAutoMode,
    isSkipMode,
    autoModeDelay,
    start,
    stop,
    advance,
    selectChoice,
    rollback,
    rollbackSteps,
    jumpToLabel,
    setVariable,
    restart,
    reloadScenario,
    setLanguage,
    submitInput,
    toggleAutoMode,
    toggleSkipMode,
    setAutoModeDelay,
    save,
    load,
  };
}
