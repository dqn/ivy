import { useState, useEffect, useRef, useCallback } from "react";
import type { ChoiceInfo } from "../../types/preview";

interface Props {
  choices: ChoiceInfo[];
  disabled?: boolean;
  timeout?: number | null;
  defaultChoice?: number | null;
  onSelect?: (index: number) => void;
}

export const ChoiceButtons: React.FC<Props> = ({
  choices,
  disabled = true,
  timeout = null,
  defaultChoice = null,
  onSelect,
}) => {
  const [remaining, setRemaining] = useState<number | null>(timeout);
  const startTimeRef = useRef<number>(Date.now());
  const completedRef = useRef(false);

  useEffect(() => {
    if (timeout === null || disabled) {
      setRemaining(null);
      return;
    }

    startTimeRef.current = Date.now();
    completedRef.current = false;
    setRemaining(timeout);

    const interval = setInterval(() => {
      const elapsed = (Date.now() - startTimeRef.current) / 1000;
      const newRemaining = Math.max(0, timeout - elapsed);
      setRemaining(newRemaining);

      if (newRemaining <= 0 && !completedRef.current) {
        completedRef.current = true;
        clearInterval(interval);
        const choiceIndex = defaultChoice ?? 0;
        onSelect?.(choiceIndex);
      }
    }, 50);

    return () => clearInterval(interval);
  }, [timeout, defaultChoice, disabled, onSelect]);

  const handleSelect = useCallback(
    (index: number) => {
      if (!completedRef.current) {
        completedRef.current = true;
        onSelect?.(index);
      }
    },
    [onSelect]
  );

  const progress = timeout !== null && remaining !== null
    ? ((timeout - remaining) / timeout) * 100
    : 0;

  return (
    <div className="choice-buttons">
      {timeout !== null && remaining !== null && !disabled && (
        <div className="choice-timeout">
          <div className="choice-timeout-bar">
            <div
              className="choice-timeout-fill"
              style={{ width: `${100 - progress}%` }}
            />
          </div>
          <span className="choice-timeout-text">{remaining.toFixed(1)}s</span>
        </div>
      )}
      {choices.map((choice, index) => (
        <button
          key={index}
          className={`choice-button ${defaultChoice === index ? "default-choice" : ""}`}
          disabled={disabled}
          onClick={() => handleSelect(index)}
        >
          {choice.label}
          {defaultChoice === index && !disabled && timeout !== null && (
            <span className="default-indicator">*</span>
          )}
        </button>
      ))}
    </div>
  );
};
