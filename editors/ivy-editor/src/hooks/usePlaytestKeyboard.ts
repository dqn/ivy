import { useEffect, useCallback } from "react";
import { getActionForKey } from "../config/keybindings";

interface UsePlaytestKeyboardProps {
  isActive: boolean;
  canRollback: boolean;
  isEnded: boolean;
  hasChoices: boolean;
  choiceCount: number;
  onAdvance: () => void;
  onRollback: () => void;
  onSelectChoice: (index: number) => void;
  onRestart: () => void;
  onToggleAutoMode?: () => void;
  onToggleSkipMode?: () => void;
  onSave?: () => void;
  onLoad?: () => void;
}

export function usePlaytestKeyboard({
  isActive,
  canRollback,
  isEnded,
  hasChoices,
  choiceCount,
  onAdvance,
  onRollback,
  onSelectChoice,
  onRestart,
  onToggleAutoMode,
  onToggleSkipMode,
  onSave,
  onLoad,
}: UsePlaytestKeyboardProps) {
  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!isActive) return;

      // Don't handle if user is typing in an input field
      const target = event.target;
      if (!(target instanceof HTMLElement)) {
        return;
      }
      if (
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable
      ) {
        return;
      }

      const action = getActionForKey(event);
      if (!action) return;

      switch (action) {
        case "advance_or_select_first":
          if (hasChoices) {
            onSelectChoice(0);
          } else if (!isEnded) {
            event.preventDefault();
            onAdvance();
          }
          break;

        case "advance":
          if (!isEnded && !hasChoices) {
            event.preventDefault();
            onAdvance();
          }
          break;

        case "rollback":
          if (canRollback) {
            event.preventDefault();
            onRollback();
          }
          break;

        case "restart":
          event.preventDefault();
          onRestart();
          break;

        case "toggle_auto":
          event.preventDefault();
          onToggleAutoMode?.();
          break;

        case "toggle_skip":
          event.preventDefault();
          onToggleSkipMode?.();
          break;

        case "save":
          event.preventDefault();
          onSave?.();
          break;

        case "load":
          event.preventDefault();
          onLoad?.();
          break;

        case "select_choice":
          if (hasChoices) {
            const choiceIndex = parseInt(event.key, 10) - 1;
            if (choiceIndex < choiceCount) {
              event.preventDefault();
              onSelectChoice(choiceIndex);
            }
          }
          break;
      }
    },
    [
      isActive,
      canRollback,
      isEnded,
      hasChoices,
      choiceCount,
      onAdvance,
      onRollback,
      onSelectChoice,
      onRestart,
      onToggleAutoMode,
      onToggleSkipMode,
      onSave,
      onLoad,
    ]
  );

  useEffect(() => {
    if (isActive) {
      window.addEventListener("keydown", handleKeyDown);
      return () => {
        window.removeEventListener("keydown", handleKeyDown);
      };
    }
  }, [isActive, handleKeyDown]);
}
