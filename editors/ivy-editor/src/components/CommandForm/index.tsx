import { useState } from "react";
import type {
  Command,
  LocalizedString,
  CharPosition,
  Choice,
  ModularCharRef,
  Transition,
  CameraCommand,
} from "../../types/scenario";
import type { CharacterDatabase } from "../../types/character";
import { AssetField } from "./AssetField";
import { ModularCharField } from "./ModularCharField";
import { TransitionPicker } from "./TransitionPicker";
import { CameraPicker } from "./CameraPicker";
import { ParticlePicker } from "./ParticlePicker";

interface CommandFormProps {
  command: Command;
  labels: string[];
  baseDir: string | null;
  characterDatabase: CharacterDatabase;
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
  baseDir,
  characterDatabase,
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

        <AssetField
          label="Background"
          value={command.background}
          baseDir={baseDir}
          accept={[".png", ".jpg", ".jpeg", ".webp"]}
          onChange={(value) => {
            updateField("background", value);
          }}
        />

        <AssetField
          label="Character"
          value={command.character}
          baseDir={baseDir}
          accept={[".png", ".jpg", ".jpeg", ".webp"]}
          onChange={(value) => {
            updateField("character", value);
          }}
        />

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

        <ModularCharField
          value={command.modular_char as ModularCharRef | undefined}
          charPos={command.char_pos}
          characterDatabase={characterDatabase}
          onChange={(value) => {
            updateField("modular_char", value);
          }}
          onCharPosChange={(pos) => {
            updateField("char_pos", pos);
          }}
        />

        <TransitionPicker
          value={command.transition as Transition | undefined}
          onChange={(value) => {
            updateField("transition", value);
          }}
        />

        <CameraPicker
          value={command.camera as CameraCommand | undefined}
          onChange={(value) => {
            updateField("camera", value);
          }}
        />
      </section>

      {/* Audio Section */}
      <section className="form-section">
        <h3>Audio</h3>

        <AssetField
          label="BGM"
          value={command.bgm}
          baseDir={baseDir}
          accept={[".mp3", ".ogg", ".wav"]}
          onChange={(value) => {
            updateField("bgm", value);
          }}
        />

        <AssetField
          label="Sound Effect"
          value={command.se}
          baseDir={baseDir}
          accept={[".mp3", ".ogg", ".wav"]}
          onChange={(value) => {
            updateField("se", value);
          }}
        />

        <AssetField
          label="Voice"
          value={command.voice}
          baseDir={baseDir}
          accept={[".mp3", ".ogg", ".wav"]}
          onChange={(value) => {
            updateField("voice", value);
          }}
        />
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
              <button onClick={() => removeChoice(index)}>x</button>
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
          {showAdvanced ? "v" : ">"} Advanced
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

            <ParticlePicker
              value={command.particles}
              intensity={command.particle_intensity}
              onChange={(value) => {
                updateField("particles", value);
              }}
              onIntensityChange={(value) => {
                updateField("particle_intensity", value);
              }}
            />

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
