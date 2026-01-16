import { useState } from "react";
import { useTranslation } from "react-i18next";
import {
  RESOLUTION_PRESETS,
  type ProjectConfig,
  type Resolution,
} from "../../types/project";
import "./ProjectSettings.css";

interface Props {
  config: ProjectConfig;
  onClose: () => void;
  onSave: (config: ProjectConfig) => void;
}

type Tab = "general" | "display" | "scenarios";

export const ProjectSettings: React.FC<Props> = ({
  config,
  onClose,
  onSave,
}) => {
  const { t } = useTranslation();
  const [activeTab, setActiveTab] = useState<Tab>("general");
  const [name, setName] = useState(config.name);
  const [version, setVersion] = useState(config.version);
  const [author, setAuthor] = useState(config.author);
  const [description, setDescription] = useState(config.description);
  const [resolution, setResolution] = useState<Resolution>(config.resolution);
  const [customWidth, setCustomWidth] = useState(config.resolution.width);
  const [customHeight, setCustomHeight] = useState(config.resolution.height);
  const [useCustomResolution, setUseCustomResolution] = useState(() => {
    return !RESOLUTION_PRESETS.some(
      (p) =>
        p.resolution.width === config.resolution.width &&
        p.resolution.height === config.resolution.height
    );
  });
  const [entryScenario, setEntryScenario] = useState(config.entry_scenario);

  const handleSave = () => {
    const finalResolution = useCustomResolution
      ? { width: customWidth, height: customHeight }
      : resolution;

    onSave({
      ...config,
      name,
      version,
      author,
      description,
      resolution: finalResolution,
      entry_scenario: entryScenario,
    });
  };

  const hasChanges =
    name !== config.name ||
    version !== config.version ||
    author !== config.author ||
    description !== config.description ||
    (useCustomResolution &&
      (customWidth !== config.resolution.width ||
        customHeight !== config.resolution.height)) ||
    (!useCustomResolution &&
      (resolution.width !== config.resolution.width ||
        resolution.height !== config.resolution.height)) ||
    entryScenario !== config.entry_scenario;

  return (
    <div className="dialog-overlay">
      <div className="settings-dialog">
        <div className="settings-header">
          <h2>{t("projectSettings.title")}</h2>
          <div className="settings-tabs">
            <button
              className={activeTab === "general" ? "active" : ""}
              onClick={() => setActiveTab("general")}
            >
              {t("projectSettings.general")}
            </button>
            <button
              className={activeTab === "display" ? "active" : ""}
              onClick={() => setActiveTab("display")}
            >
              {t("projectSettings.display")}
            </button>
            <button
              className={activeTab === "scenarios" ? "active" : ""}
              onClick={() => setActiveTab("scenarios")}
            >
              {t("projectSettings.scenariosTab")}
            </button>
          </div>
        </div>

        <div className="settings-content">
          {activeTab === "general" && (
            <div className="settings-tab-content">
              <div className="form-field">
                <label>
                  {t("projectWizard.projectName")} <span className="required">{t("common.required")}</span>
                </label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                />
              </div>

              <div className="form-field">
                <label>{t("projectSettings.version")}</label>
                <input
                  type="text"
                  value={version}
                  onChange={(e) => setVersion(e.target.value)}
                />
              </div>

              <div className="form-field">
                <label>{t("projectWizard.author")}</label>
                <input
                  type="text"
                  value={author}
                  onChange={(e) => setAuthor(e.target.value)}
                />
              </div>

              <div className="form-field">
                <label>{t("projectWizard.description")}</label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  rows={4}
                />
              </div>
            </div>
          )}

          {activeTab === "display" && (
            <div className="settings-tab-content">
              <div className="form-field">
                <label>{t("projectWizard.resolution")}</label>
                <div className="resolution-options">
                  {RESOLUTION_PRESETS.map((preset) => (
                    <button
                      key={preset.label}
                      className={`resolution-option ${
                        !useCustomResolution &&
                        resolution.width === preset.resolution.width &&
                        resolution.height === preset.resolution.height
                          ? "selected"
                          : ""
                      }`}
                      onClick={() => {
                        setResolution(preset.resolution);
                        setUseCustomResolution(false);
                      }}
                    >
                      {preset.label}
                    </button>
                  ))}
                  <button
                    className={`resolution-option ${useCustomResolution ? "selected" : ""}`}
                    onClick={() => setUseCustomResolution(true)}
                  >
                    {t("projectWizard.custom")}
                  </button>
                </div>
              </div>

              {useCustomResolution && (
                <div className="custom-resolution">
                  <div className="form-field">
                    <label>{t("projectWizard.width")}</label>
                    <input
                      type="number"
                      value={customWidth}
                      onChange={(e) => setCustomWidth(Number(e.target.value))}
                      min={640}
                      max={7680}
                    />
                  </div>
                  <div className="form-field">
                    <label>{t("projectWizard.height")}</label>
                    <input
                      type="number"
                      value={customHeight}
                      onChange={(e) => setCustomHeight(Number(e.target.value))}
                      min={480}
                      max={4320}
                    />
                  </div>
                </div>
              )}
            </div>
          )}

          {activeTab === "scenarios" && (
            <div className="settings-tab-content">
              <div className="form-field">
                <label>{t("projectSettings.entryScenario")}</label>
                <select
                  value={entryScenario}
                  onChange={(e) => setEntryScenario(e.target.value)}
                >
                  {config.scenarios.map((s) => (
                    <option key={s.path} value={s.path}>
                      {s.chapter || s.path}
                    </option>
                  ))}
                </select>
                <p className="field-help">
                  {t("projectSettings.entryScenarioHelp")}
                </p>
              </div>

              <div className="scenarios-list">
                <label>{t("projectSettings.scenariosInProject")}</label>
                <div className="scenario-items">
                  {config.scenarios.map((s) => (
                    <div
                      key={s.path}
                      className={`scenario-item ${s.path === entryScenario ? "entry" : ""}`}
                    >
                      <div className="scenario-info">
                        <span className="scenario-chapter">
                          {s.chapter || t("projectSettings.untitled")}
                        </span>
                        <span className="scenario-path">{s.path}</span>
                      </div>
                      {s.path === entryScenario && (
                        <span className="entry-badge">{t("projectSettings.entry")}</span>
                      )}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>

        <div className="settings-actions">
          <button onClick={onClose}>{t("common.cancel")}</button>
          <button
            className="primary"
            onClick={handleSave}
            disabled={!name.trim() || !hasChanges}
          >
            {t("projectSettings.saveChanges")}
          </button>
        </div>
      </div>
    </div>
  );
};
