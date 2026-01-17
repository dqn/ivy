import { useState } from "react";
import { useTranslation } from "react-i18next";
import { open } from "@tauri-apps/plugin-dialog";
import {
  DEFAULT_RESOLUTION,
  RESOLUTION_PRESETS,
  type Resolution,
} from "../../types/project";
import "./ProjectWizard.css";

interface Props {
  onClose: () => void;
  onCreate: (
    rootPath: string,
    name: string,
    author: string,
    description: string,
    resolution: Resolution
  ) => Promise<void>;
}

type Step = "basic" | "resolution" | "summary";

export const ProjectWizard: React.FC<Props> = ({ onClose, onCreate }) => {
  const { t } = useTranslation();
  const [step, setStep] = useState<Step>("basic");
  const [name, setName] = useState("");
  const [author, setAuthor] = useState("");
  const [description, setDescription] = useState("");
  const [resolution, setResolution] = useState<Resolution>(DEFAULT_RESOLUTION);
  const [customWidth, setCustomWidth] = useState(1920);
  const [customHeight, setCustomHeight] = useState(1080);
  const [useCustomResolution, setUseCustomResolution] = useState(false);
  const [rootPath, setRootPath] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleSelectFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      setRootPath(selected);
    }
  };

  const handleNext = () => {
    if (step === "basic") {
      setStep("resolution");
    } else if (step === "resolution") {
      setStep("summary");
    }
  };

  const handleBack = () => {
    if (step === "resolution") {
      setStep("basic");
    } else if (step === "summary") {
      setStep("resolution");
    }
  };

  const handleCreate = async () => {
    setIsCreating(true);
    setError(null);

    try {
      const finalResolution = useCustomResolution
        ? { width: customWidth, height: customHeight }
        : resolution;
      const projectPath = rootPath.endsWith("/")
        ? `${rootPath}${name}`
        : `${rootPath}/${name}`;
      await onCreate(projectPath, name, author, description, finalResolution);
    } catch (e) {
      setError(String(e));
      setIsCreating(false);
    }
  };

  const canProceedBasic = name.trim() !== "" && rootPath !== "";
  const canProceedResolution = true;

  return (
    <div className="dialog-overlay">
      <div className="wizard-dialog">
        <div className="wizard-header">
          <h2>{t("projectWizard.title")}</h2>
          <div className="wizard-steps">
            <div className={`wizard-step ${step === "basic" ? "active" : ""}`}>
              {t("projectWizard.step1")}
            </div>
            <div
              className={`wizard-step ${step === "resolution" ? "active" : ""}`}
            >
              {t("projectWizard.step2")}
            </div>
            <div
              className={`wizard-step ${step === "summary" ? "active" : ""}`}
            >
              {t("projectWizard.step3")}
            </div>
          </div>
        </div>

        <div className="wizard-content">
          {step === "basic" && (
            <div className="wizard-step-content">
              <p className="step-description">{t("projectWizard.step1Description")}</p>
              <div className="form-field">
                <label>
                  {t("projectWizard.projectName")} <span className="required">{t("common.required")}</span>
                </label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder={t("projectWizard.projectNamePlaceholder")}
                  autoFocus
                />
              </div>

              <div className="form-field">
                <label>
                  {t("projectWizard.location")} <span className="required">{t("common.required")}</span>
                </label>
                <div className="folder-select">
                  <input
                    type="text"
                    value={rootPath}
                    onChange={(e) => setRootPath(e.target.value)}
                    placeholder={t("projectWizard.locationPlaceholder")}
                    readOnly
                  />
                  <button onClick={() => void handleSelectFolder()}>
                    {t("projectWizard.browse")}
                  </button>
                </div>
                {rootPath && name && (
                  <p className="folder-preview">
                    {t("projectWizard.projectWillBeCreated", { path: `${rootPath}/${name}` })}
                  </p>
                )}
              </div>

              <div className="form-field">
                <label>{t("projectWizard.author")}</label>
                <input
                  type="text"
                  value={author}
                  onChange={(e) => setAuthor(e.target.value)}
                  placeholder={t("projectWizard.authorPlaceholder")}
                />
              </div>

              <div className="form-field">
                <label>{t("projectWizard.description")}</label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder={t("projectWizard.descriptionPlaceholder")}
                  rows={3}
                />
              </div>
            </div>
          )}

          {step === "resolution" && (
            <div className="wizard-step-content">
              <p className="step-description">{t("projectWizard.step2Description")}</p>
              <div className="form-field">
                <label>{t("projectWizard.displayResolution")}</label>
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
                      title={t(`projectWizard.resolutionHint.${preset.hintKey}`)}
                    >
                      <span className="resolution-label">{preset.label}</span>
                      <span className="resolution-hint">
                        {t(`projectWizard.resolutionHint.${preset.hintKey}`)}
                      </span>
                    </button>
                  ))}
                  <button
                    className={`resolution-option ${useCustomResolution ? "selected" : ""}`}
                    onClick={() => setUseCustomResolution(true)}
                  >
                    <span className="resolution-label">{t("projectWizard.custom")}</span>
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

              <div className="resolution-preview">
                <div
                  className="preview-box"
                  style={{
                    aspectRatio: `${useCustomResolution ? customWidth : resolution.width} / ${useCustomResolution ? customHeight : resolution.height}`,
                  }}
                >
                  {useCustomResolution
                    ? `${customWidth} x ${customHeight}`
                    : `${resolution.width} x ${resolution.height}`}
                </div>
              </div>
            </div>
          )}

          {step === "summary" && (
            <div className="wizard-step-content">
              <p className="step-description">{t("projectWizard.step3Description")}</p>
              <div className="summary-section">
                <h3>{t("projectWizard.projectSummary")}</h3>
                <dl className="summary-list">
                  <dt>{t("projectWizard.name")}</dt>
                  <dd>{name}</dd>

                  <dt>{t("projectWizard.location")}</dt>
                  <dd>
                    {rootPath}/{name}
                  </dd>

                  {author && (
                    <>
                      <dt>{t("projectWizard.author")}</dt>
                      <dd>{author}</dd>
                    </>
                  )}

                  {description && (
                    <>
                      <dt>{t("projectWizard.description")}</dt>
                      <dd>{description}</dd>
                    </>
                  )}

                  <dt>{t("projectWizard.resolution")}</dt>
                  <dd>
                    {useCustomResolution
                      ? `${customWidth} x ${customHeight}`
                      : `${resolution.width} x ${resolution.height}`}
                  </dd>
                </dl>
              </div>

              <div className="summary-section">
                <h3>{t("projectWizard.filesToBeCreated")}</h3>
                <ul className="file-list">
                  <li>ivy-project.yaml</li>
                  <li>scenarios/main.ivy.yaml</li>
                  <li>assets/backgrounds/</li>
                  <li>assets/characters/</li>
                  <li>assets/audio/</li>
                </ul>
              </div>

              {error && <div className="wizard-error">{error}</div>}
            </div>
          )}
        </div>

        <div className="wizard-actions">
          <button onClick={onClose} disabled={isCreating}>
            {t("common.cancel")}
          </button>
          <div className="wizard-actions-right">
            {step !== "basic" && (
              <button onClick={handleBack} disabled={isCreating}>
                {t("common.back")}
              </button>
            )}
            {step !== "summary" ? (
              <button
                className="primary"
                onClick={handleNext}
                disabled={
                  step === "basic"
                    ? !canProceedBasic
                    : !canProceedResolution
                }
              >
                {t("common.next")}
              </button>
            ) : (
              <button
                className="primary"
                onClick={() => void handleCreate()}
                disabled={isCreating}
              >
                {isCreating ? t("projectWizard.creating") : t("projectWizard.createProject")}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
