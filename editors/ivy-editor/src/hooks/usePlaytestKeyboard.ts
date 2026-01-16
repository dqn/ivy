import { useEffect, useCallback } from "react";

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
      const target = event.target as HTMLElement;
      if (
        target.tagName === "INPUT" ||
        target.tagName === "TEXTAREA" ||
        target.isContentEditable
      ) {
        return;
      }

      switch (event.key) {
        case "Enter":
        case " ":
          // Advance or select first choice
          if (hasChoices) {
            // Enter selects first choice when choices are shown
            if (event.key === "Enter") {
              onSelectChoice(0);
            }
          } else if (!isEnded) {
            event.preventDefault();
            onAdvance();
          }
          break;

        case "Backspace":
        case "ArrowUp":
          // Rollback
          if (canRollback) {
            event.preventDefault();
            onRollback();
          }
          break;

        case "ArrowDown":
        case "ArrowRight":
          // Advance
          if (!isEnded && !hasChoices) {
            event.preventDefault();
            onAdvance();
          }
          break;

        case "r":
        case "R":
          // Restart (with Ctrl/Cmd)
          if (event.ctrlKey || event.metaKey) {
            event.preventDefault();
            onRestart();
          }
          break;

        case "a":
        case "A":
          // Toggle auto mode
          event.preventDefault();
          onToggleAutoMode?.();
          break;

        case "s":
        case "S":
          // Toggle skip mode (without Ctrl/Cmd to avoid browser save)
          if (!event.ctrlKey && !event.metaKey) {
            event.preventDefault();
            onToggleSkipMode?.();
          }
          break;

        case "F5":
          // Quick save
          event.preventDefault();
          onSave?.();
          break;

        case "F9":
          // Quick load
          event.preventDefault();
          onLoad?.();
          break;

        case "1":
        case "2":
        case "3":
        case "4":
        case "5":
        case "6":
        case "7":
        case "8":
        case "9":
          // Number keys for selecting choices
          if (hasChoices) {
            const choiceIndex = parseInt(event.key, 10) - 1;
            if (choiceIndex < choiceCount) {
              event.preventDefault();
              onSelectChoice(choiceIndex);
            }
          }
          break;

        default:
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
