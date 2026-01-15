import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Scenario } from "../types/scenario";
import type { PreviewState } from "../types/preview";

interface UsePreviewReturn {
  state: PreviewState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  language: string;
  setLanguage: (lang: string) => void;
  goto: (index: number) => void;
  next: () => void;
  prev: () => void;
}

export function usePreview(
  scenario: Scenario | null,
  baseDir: string | null,
  selectedIndex: number | null
): UsePreviewReturn {
  const [state, setState] = useState<PreviewState | null>(null);
  const [backgroundUrl, setBackgroundUrl] = useState<string | null>(null);
  const [characterUrl, setCharacterUrl] = useState<string | null>(null);
  const [index, setIndex] = useState(0);
  const [language, setLanguage] = useState("en");

  // Sync with selected index from editor
  useEffect(() => {
    if (selectedIndex !== null) {
      setIndex(selectedIndex);
    }
  }, [selectedIndex]);

  // Fetch preview state when scenario, index, or language changes
  useEffect(() => {
    if (!scenario) {
      setState(null);
      return;
    }

    const fetchState = async () => {
      try {
        const previewState = await invoke<PreviewState>("get_preview_state", {
          scenario,
          index,
          variables: {},
          lang: language,
        });
        setState(previewState);
      } catch (e) {
        console.error("Failed to get preview state:", e);
      }
    };

    void fetchState();
  }, [scenario, index, language]);

  // Load assets when state changes
  useEffect(() => {
    if (!state || !baseDir) {
      setBackgroundUrl(null);
      setCharacterUrl(null);
      return;
    }

    const loadAssets = async () => {
      if (state.background) {
        try {
          const url = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: state.background,
          });
          setBackgroundUrl(url);
        } catch {
          setBackgroundUrl(null);
        }
      } else {
        setBackgroundUrl(null);
      }

      if (state.character) {
        try {
          const url = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: state.character,
          });
          setCharacterUrl(url);
        } catch {
          setCharacterUrl(null);
        }
      } else {
        setCharacterUrl(null);
      }
    };

    void loadAssets();
  }, [state?.background, state?.character, baseDir]);

  const goto = useCallback(
    (newIndex: number) => {
      if (!state) return;
      const clamped = Math.max(0, Math.min(newIndex, state.total_commands - 1));
      setIndex(clamped);
    },
    [state]
  );

  const next = useCallback(() => {
    goto(index + 1);
  }, [index, goto]);

  const prev = useCallback(() => {
    goto(index - 1);
  }, [index, goto]);

  return {
    state,
    backgroundUrl,
    characterUrl,
    language,
    setLanguage,
    goto,
    next,
    prev,
  };
}
