import { useState } from "react";
import type {
  Command,
  LocalizedString,
  CharPosition,
  Choice,
} from "../types/scenario";

interface CommandFormProps {
  command: Command;
  labels: string[];
  onChange: (command: Command) => void;
}

function getTextValue(text: LocalizedString | undefined): string {
  if (!text) return "";
  if (typeof text === "string") return text;
  return Object.values(text)[0] || "";
}

function setTextValue(value: string): LocalizedString {
  return value;
}

export const CommandForm: React.FC<CommandFormProps> = ({
  command,
  labels,
  onChange,
}) => {
  const [showAdvanced, setShowAdvanced] = useState(false);

  const updateField = <K extends keyof Command>(
    field: K,
    value: Command[K] | undefined
  ) => {
    const updated = { ...command };
    if (value === undefined || value === "") {
      delete updated[field];
    } else {
      updated[field] = value;
    }
    onChange(updated);
  };

  const updateChoice = (index: number, choice: Choice) => {
    const choices = [...(command.choices || [])];
    choices[index] = choice;
    updateField("choices", choices);
  };

  const addChoice = () => {
    const choices = [...(command.choices || []), { label: "", jump: "" }];
    updateField("choices", choices);
  };

  const removeChoice = (index: number) => {
    const choices = (command.choices || []).filter((_, i) => i !== index);
    updateField("choices", choices.length > 0 ? choices : undefined);
  };

  return (
    <div className="command-form">
      {/* Basic Section */}
      <section className="form-section">
        <h3>Basic</h3>

        <div className="form-field optional">
          <label>Label</label>
          <input
            type="text"
            value={command.label || ""}
            onChange={(e) => {
              updateField("label", e.target.value || undefined);
            }}
            placeholder="Label name for jump targets"
          />
        </div>

        <div className="form-field optional">
          <label>Speaker</label>
          <input
            type="text"
            value={getTextValue(command.speaker)}
            onChange={(e) => {
              updateField(
                "speaker",
                e.target.value ? setTextValue(e.target.value) : undefined
              );
            }}
            placeholder="Character name"
          />
        </div>

        <div className="form-field">
          <label>
            Text<span className="required">*</span>
          </label>
          <textarea
            value={getTextValue(command.text)}
            onChange={(e) => {
              updateField(
                "text",
                e.target.value ? setTextValue(e.target.value) : undefined
              );
            }}
            placeholder="Dialogue or narration text"
            rows={3}
          />
        </div>
      </section>

      {/* Visual Section */}
      <section className="form-section">
        <h3>Visual</h3>

        <div className="form-field optional">
          <label>Background</label>
          <input
            type="text"
            value={command.background ?? ""}
            onChange={(e) => {
              updateField("background", e.target.value || undefined);
            }}
            placeholder="assets/bg.png (empty to clear)"
          />
        </div>

        <div className="form-field optional">
          <label>Character</label>
          <input
            type="text"
            value={command.character ?? ""}
            onChange={(e) => {
              updateField("character", e.target.value || undefined);
            }}
            placeholder="assets/char.png (empty to clear)"
          />
        </div>

        <div className="form-field optional">
          <label>Position</label>
          <select
            value={command.char_pos || "center"}
            onChange={(e) => {
              updateField(
                "char_pos",
                (e.target.value as CharPosition) || undefined
              );
            }}
          >
            <option value="left">Left</option>
            <option value="center">Center</option>
            <option value="right">Right</option>
          </select>
        </div>
      </section>

      {/* Audio Section */}
      <section className="form-section">
        <h3>Audio</h3>

        <div className="form-field optional">
          <label>BGM</label>
          <input
            type="text"
            value={command.bgm ?? ""}
            onChange={(e) => {
              updateField("bgm", e.target.value || undefined);
            }}
            placeholder="assets/bgm.mp3 (empty to stop)"
          />
        </div>

        <div className="form-field optional">
          <label>Sound Effect</label>
          <input
            type="text"
            value={command.se ?? ""}
            onChange={(e) => {
              updateField("se", e.target.value || undefined);
            }}
            placeholder="assets/se.mp3"
          />
        </div>

        <div className="form-field optional">
          <label>Voice</label>
          <input
            type="text"
            value={command.voice ?? ""}
            onChange={(e) => {
              updateField("voice", e.target.value || undefined);
            }}
            placeholder="assets/voice.mp3"
          />
        </div>
      </section>

      {/* Flow Section */}
      <section className="form-section">
        <h3>Flow</h3>

        <div className="form-field optional">
          <label>Jump</label>
          <select
            value={command.jump || ""}
            onChange={(e) => {
              updateField("jump", e.target.value || undefined);
            }}
          >
            <option value="">-- No jump --</option>
            {labels.map((label) => (
              <option key={label} value={label}>
                {label}
              </option>
            ))}
          </select>
        </div>

        <div className="form-field optional">
          <label>Choices</label>
          {(command.choices || []).map((choice, index) => (
            <div key={index} className="choice-row">
              <input
                type="text"
                value={getTextValue(choice.label)}
                onChange={(e) => {
                  updateChoice(index, {
                    ...choice,
                    label: setTextValue(e.target.value),
                  });
                }}
                placeholder="Choice text *"
              />
              <select
                value={choice.jump}
                onChange={(e) => {
                  updateChoice(index, { ...choice, jump: e.target.value });
                }}
              >
                <option value="">-- Jump to * --</option>
                {labels.map((label) => (
                  <option key={label} value={label}>
                    {label}
                  </option>
                ))}
              </select>
              <button onClick={() => removeChoice(index)}>×</button>
            </div>
          ))}
          <button className="add-choice" onClick={addChoice}>
            + Add Choice
          </button>
        </div>
      </section>

      {/* Advanced Section (Collapsible) */}
      <section className="form-section advanced">
        <h3
          onClick={() => {
            setShowAdvanced(!showAdvanced);
          }}
          style={{ cursor: "pointer" }}
        >
          {showAdvanced ? "▼" : "▶"} Advanced
        </h3>
        {showAdvanced && (
          <>
            <div className="form-field optional">
              <label>Wait (seconds)</label>
              <input
                type="number"
                step="0.1"
                value={command.wait ?? ""}
                onChange={(e) => {
                  updateField(
                    "wait",
                    e.target.value ? parseFloat(e.target.value) : undefined
                  );
                }}
                placeholder="0.5"
              />
            </div>

            <div className="form-field optional">
              <label>Timeout (seconds)</label>
              <input
                type="number"
                step="0.1"
                value={command.timeout ?? ""}
                onChange={(e) => {
                  updateField(
                    "timeout",
                    e.target.value ? parseFloat(e.target.value) : undefined
                  );
                }}
                placeholder="10"
              />
            </div>

            <div className="form-field optional">
              <label>Particles</label>
              <select
                value={command.particles || ""}
                onChange={(e) => {
                  updateField("particles", e.target.value || undefined);
                }}
              >
                <option value="">-- None --</option>
                <option value="snow">Snow</option>
                <option value="rain">Rain</option>
                <option value="sakura">Sakura</option>
                <option value="sparkle">Sparkle</option>
                <option value="leaves">Leaves</option>
              </select>
            </div>

            <div className="form-field optional">
              <label>
                <input
                  type="checkbox"
                  checked={command.nvl ?? false}
                  onChange={(e) => {
                    updateField("nvl", e.target.checked || undefined);
                  }}
                />
                NVL Mode
              </label>
            </div>

            <div className="form-field optional">
              <label>
                <input
                  type="checkbox"
                  checked={command.nvl_clear ?? false}
                  onChange={(e) => {
                    updateField("nvl_clear", e.target.checked || undefined);
                  }}
                />
                NVL Clear
              </label>
            </div>

            <div className="form-field optional">
              <label>
                <input
                  type="checkbox"
                  checked={command.cinematic ?? false}
                  onChange={(e) => {
                    updateField("cinematic", e.target.checked || undefined);
                  }}
                />
                Cinematic Mode
              </label>
            </div>
          </>
        )}
      </section>
    </div>
  );
};
