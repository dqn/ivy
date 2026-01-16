import { useState, useCallback } from "react";

interface Props {
  prompt: string;
  defaultValue: string | null;
  onSubmit: (value: string) => void;
}

export const InputForm: React.FC<Props> = ({ prompt, defaultValue, onSubmit }) => {
  const [value, setValue] = useState(defaultValue ?? "");

  const handleSubmit = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onSubmit(value);
    },
    [value, onSubmit]
  );

  const handleKeyDown = useCallback(
    (e: React.KeyboardEvent) => {
      if (e.key === "Enter" && !e.shiftKey) {
        e.preventDefault();
        onSubmit(value);
      }
    },
    [value, onSubmit]
  );

  return (
    <div className="input-form-overlay">
      <form className="input-form" onSubmit={handleSubmit}>
        <label className="input-form-prompt">{prompt}</label>
        <input
          type="text"
          className="input-form-field"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          autoFocus
        />
        <button type="submit" className="input-form-submit">
          OK
        </button>
      </form>
    </div>
  );
};
