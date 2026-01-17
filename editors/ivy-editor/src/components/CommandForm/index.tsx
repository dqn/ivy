import { useState } from "react";
import { useTranslation } from "react-i18next";
import type {
  Command,
  LocalizedString,
  CharPosition,
  Choice,
} from "../../types/scenario";
import type { CharacterDatabase } from "../../types/character";
import { AssetField } from "./AssetField";
import { ModularCharField } from "./ModularCharField";
import { TransitionPicker } from "./TransitionPicker";
import { CameraPicker } from "./CameraPicker";
import { ParticlePicker } from "./ParticlePicker";
import { VariableEditor } from "./VariableEditor";
import { LocalizedStringEditor } from "./LocalizedStringEditor";
import { Tooltip } from "../Tooltip";

interface CommandFormProps {
  command: Command;
  labels: string[];
  baseDir: string | null;
  characterDatabase: CharacterDatabase;
  onChange: (command: Command) => void;
}

function getTextValue(text: LocalizedString | undefined): string {
  if (!text) {return "";}
  if (typeof text === "string") {return text;}
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
  const { t } = useTranslation();
  const [simpleMode, setSimpleMode] = useState(true);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const updateField = <K extends keyof Command>(
    field: K,
    value: Command[K] | undefined
  ) => {
    if (value === undefined || value === "") {
      const updated = { ...command };
      delete (updated as Record<string, unknown>)[field];
      onChange(updated);
    } else {
      onChange({ ...command, [field]: value });
    }
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
      {/* Mode Toggle */}
      <div className="mode-toggle">
        <button
          className={simpleMode ? "active" : ""}
          onClick={() => setSimpleMode(true)}
        >
          {t("commandForm.simpleMode")}
        </button>
        <button
          className={!simpleMode ? "active" : ""}
          onClick={() => setSimpleMode(false)}
        >
          {t("commandForm.advancedMode")}
        </button>
      </div>

      {/* Basic Section */}
      <section className="form-section">
        <h3>📝 {t("commandForm.sections.basic")}</h3>

        {!simpleMode && (
          <div className="form-field optional">
            <Tooltip content={t("commandForm.tooltips.label")} position="right">
              <label>{t("commandForm.fields.label")}</label>
            </Tooltip>
            <input
              type="text"
              value={command.label || ""}
              onChange={(e) => {
                updateField("label", e.target.value || undefined);
              }}
              placeholder={t("commandForm.placeholders.label")}
            />
          </div>
        )}

        <LocalizedStringEditor
          label={t("commandForm.fields.speaker")}
          value={command.speaker}
          placeholder={t("commandForm.placeholders.speaker")}
          onChange={(value) => {
            updateField("speaker", value);
          }}
        />

        <LocalizedStringEditor
          label={t("commandForm.fields.text")}
          value={command.text}
          placeholder={t("commandForm.placeholders.text")}
          multiline
          required
          onChange={(value) => {
            updateField("text", value);
          }}
        />
      </section>

      {/* Visual Section */}
      <section className="form-section">
        <h3>🖼️ {t("commandForm.sections.visual")}</h3>

        <AssetField
          label={t("commandForm.fields.background")}
          value={command.background}
          baseDir={baseDir}
          accept={[".png", ".jpg", ".jpeg", ".webp"]}
          onChange={(value) => {
            updateField("background", value);
          }}
        />

        <AssetField
          label={t("commandForm.fields.character")}
          value={command.character}
          baseDir={baseDir}
          accept={[".png", ".jpg", ".jpeg", ".webp"]}
          onChange={(value) => {
            updateField("character", value);
          }}
        />

        <div className="form-field optional">
          <Tooltip content={t("commandForm.tooltips.position")} position="right">
            <label>{t("commandForm.fields.position")}</label>
          </Tooltip>
          <select
            value={command.char_pos || "center"}
            onChange={(e) => {
              updateField(
                "char_pos",
                (e.target.value as CharPosition) || undefined
              );
            }}
          >
            <option value="left">{t("commandForm.positions.left")}</option>
            <option value="center">{t("commandForm.positions.center")}</option>
            <option value="right">{t("commandForm.positions.right")}</option>
          </select>
        </div>

        {!simpleMode && (
          <>
            <ModularCharField
              value={command.modular_char}
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
              value={command.transition}
              onChange={(value) => {
                updateField("transition", value);
              }}
            />

            <CameraPicker
              value={command.camera}
              onChange={(value) => {
                updateField("camera", value);
              }}
            />
          </>
        )}
      </section>

      {/* Audio Section - only in advanced mode */}
      {!simpleMode && (
        <section className="form-section">
          <h3>🎵 {t("commandForm.sections.audio")}</h3>

          <AssetField
            label={t("commandForm.fields.bgm")}
            value={command.bgm}
            baseDir={baseDir}
            accept={[".mp3", ".ogg", ".wav"]}
            onChange={(value) => {
              updateField("bgm", value);
            }}
          />

          <AssetField
            label={t("commandForm.fields.se")}
            value={command.se}
            baseDir={baseDir}
            accept={[".mp3", ".ogg", ".wav"]}
            onChange={(value) => {
              updateField("se", value);
            }}
          />

          <AssetField
            label={t("commandForm.fields.voice")}
            value={command.voice}
            baseDir={baseDir}
            accept={[".mp3", ".ogg", ".wav"]}
            onChange={(value) => {
              updateField("voice", value);
            }}
          />
        </section>
      )}

      {/* Flow Section - only in advanced mode */}
      {!simpleMode && (
        <section className="form-section">
          <h3>🔀 {t("commandForm.sections.flow")}</h3>

          <div className="form-field optional">
            <Tooltip content={t("commandForm.tooltips.jump")} position="right">
              <label>{t("commandForm.fields.jump")}</label>
            </Tooltip>
            <select
              value={command.jump || ""}
              onChange={(e) => {
                updateField("jump", e.target.value || undefined);
              }}
            >
              <option value="">{t("commandForm.placeholders.noJump")}</option>
              {labels.map((label) => (
                <option key={label} value={label}>
                  {label}
                </option>
              ))}
            </select>
          </div>

          <div className="form-field optional">
            <Tooltip content={t("commandForm.tooltips.choices")} position="right">
              <label>{t("commandForm.fields.choices")}</label>
            </Tooltip>
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
                  placeholder={t("commandForm.placeholders.choiceText")}
                />
                <select
                  value={choice.jump}
                  onChange={(e) => {
                    updateChoice(index, { ...choice, jump: e.target.value });
                  }}
                >
                  <option value="">{t("commandForm.placeholders.jumpTo")}</option>
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
              {t("commandForm.addChoice")}
            </button>
          </div>

          <VariableEditor
            setVar={command.set}
            ifCondition={command.if}
            labels={labels}
            onSetChange={(value) => {
              updateField("set", value);
            }}
            onIfChange={(value) => {
              updateField("if", value);
            }}
          />
        </section>
      )}

      {/* Advanced Section (Collapsible) - only in advanced mode */}
      {!simpleMode && (
        <section className="form-section advanced">
          <h3
            onClick={() => {
              setShowAdvanced(!showAdvanced);
            }}
            style={{ cursor: "pointer" }}
          >
            {showAdvanced ? "▼" : "▶"} ⚙️ {t("commandForm.sections.advanced")}
          </h3>
          {showAdvanced && (
            <>
              <div className="form-field optional">
                <Tooltip content={t("commandForm.tooltips.wait")} position="right">
                  <label>{t("commandForm.fields.wait")}</label>
                </Tooltip>
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
                <Tooltip content={t("commandForm.tooltips.timeout")} position="right">
                  <label>{t("commandForm.fields.timeout")}</label>
                </Tooltip>
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
                  {t("commandForm.fields.nvlMode")}
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
                  {t("commandForm.fields.nvlClear")}
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
                  {t("commandForm.fields.cinematic")}
                </label>
              </div>
            </>
          )}
        </section>
      )}
    </div>
  );
};
