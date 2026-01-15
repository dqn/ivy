import { useState } from "react";
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
          <h2>Create New Project</h2>
          <div className="wizard-steps">
            <div className={`wizard-step ${step === "basic" ? "active" : ""}`}>
              1. Basic Info
            </div>
            <div
              className={`wizard-step ${step === "resolution" ? "active" : ""}`}
            >
              2. Display
            </div>
            <div
              className={`wizard-step ${step === "summary" ? "active" : ""}`}
            >
              3. Summary
            </div>
          </div>
        </div>

        <div className="wizard-content">
          {step === "basic" && (
            <div className="wizard-step-content">
              <div className="form-field">
                <label>
                  Project Name <span className="required">*</span>
                </label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="My Visual Novel"
                  autoFocus
                />
              </div>

              <div className="form-field">
                <label>
                  Location <span className="required">*</span>
                </label>
                <div className="folder-select">
                  <input
                    type="text"
                    value={rootPath}
                    onChange={(e) => setRootPath(e.target.value)}
                    placeholder="Select a folder..."
                    readOnly
                  />
                  <button onClick={() => void handleSelectFolder()}>
                    Browse
                  </button>
                </div>
                {rootPath && name && (
                  <p className="folder-preview">
                    Project will be created at: {rootPath}/{name}
                  </p>
                )}
              </div>

              <div className="form-field">
                <label>Author</label>
                <input
                  type="text"
                  value={author}
                  onChange={(e) => setAuthor(e.target.value)}
                  placeholder="Your name"
                />
              </div>

              <div className="form-field">
                <label>Description</label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  placeholder="A brief description of your project..."
                  rows={3}
                />
              </div>
            </div>
          )}

          {step === "resolution" && (
            <div className="wizard-step-content">
              <div className="form-field">
                <label>Display Resolution</label>
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
                    Custom
                  </button>
                </div>
              </div>

              {useCustomResolution && (
                <div className="custom-resolution">
                  <div className="form-field">
                    <label>Width</label>
                    <input
                      type="number"
                      value={customWidth}
                      onChange={(e) => setCustomWidth(Number(e.target.value))}
                      min={640}
                      max={7680}
                    />
                  </div>
                  <div className="form-field">
                    <label>Height</label>
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
              <div className="summary-section">
                <h3>Project Summary</h3>
                <dl className="summary-list">
                  <dt>Name</dt>
                  <dd>{name}</dd>

                  <dt>Location</dt>
                  <dd>
                    {rootPath}/{name}
                  </dd>

                  {author && (
                    <>
                      <dt>Author</dt>
                      <dd>{author}</dd>
                    </>
                  )}

                  {description && (
                    <>
                      <dt>Description</dt>
                      <dd>{description}</dd>
                    </>
                  )}

                  <dt>Resolution</dt>
                  <dd>
                    {useCustomResolution
                      ? `${customWidth} x ${customHeight}`
                      : `${resolution.width} x ${resolution.height}`}
                  </dd>
                </dl>
              </div>

              <div className="summary-section">
                <h3>Files to be created</h3>
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
            Cancel
          </button>
          <div className="wizard-actions-right">
            {step !== "basic" && (
              <button onClick={handleBack} disabled={isCreating}>
                Back
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
                Next
              </button>
            ) : (
              <button
                className="primary"
                onClick={() => void handleCreate()}
                disabled={isCreating}
              >
                {isCreating ? "Creating..." : "Create Project"}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
