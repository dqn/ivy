import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { useExport } from "../../hooks/useExport";
import type {
  ExportOptions,
  ExportTarget,
  BuildEnvironment,
  PackageFormat,
} from "../../types/export";
import {
  EXPORT_TARGET_LABELS,
  PACKAGE_FORMAT_LABELS,
  getDefaultExportOptions,
  getDefaultPackageFormat,
} from "../../types/export";
import "./styles.css";

interface Props {
  projectPath: string;
  projectName: string;
  onClose: () => void;
}

type WizardStep = "target" | "options" | "build";

export const ExportWizard: React.FC<Props> = ({
  projectPath,
  projectName,
  onClose,
}) => {
  const [step, setStep] = useState<WizardStep>("target");
  const [options, setOptions] = useState<ExportOptions>(getDefaultExportOptions);
  const [envChecked, setEnvChecked] = useState(false);

  const {
    isExporting,
    progress,
    result,
    environment,
    checkEnvironment,
    startExport,
    cancelExport,
    clearResult,
  } = useExport();

  useEffect(() => {
    void checkEnvironment().then(() => setEnvChecked(true));
  }, [checkEnvironment]);

  const handleSelectOutputDir = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      setOptions((prev) => ({ ...prev, output_dir: selected }));
    }
  };

  const handleTargetChange = (target: ExportTarget) => {
    setOptions((prev) => ({
      ...prev,
      target,
      package_format: getDefaultPackageFormat(target),
    }));
  };

  const handleStartExport = async () => {
    setStep("build");
    await startExport(projectPath, options);
  };

  const handleClose = () => {
    if (isExporting) {
      void cancelExport();
    }
    clearResult();
    onClose();
  };

  const canProceedToOptions =
    envChecked &&
    environment &&
    (environment.has_rust || options.target === "web");

  const canStartExport =
    options.output_dir !== "" &&
    (options.target !== "web" || (environment?.has_wasm_pack ?? false));

  const renderEnvironmentStatus = (env: BuildEnvironment) => {
    return (
      <div className="environment-status">
        <h4>Build Environment</h4>
        <ul>
          <li className={env.has_rust ? "ok" : "missing"}>
            Rust: {env.has_rust ? `✓ ${env.rust_version}` : "✗ Not found"}
          </li>
          <li className={env.has_cargo ? "ok" : "missing"}>
            Cargo: {env.has_cargo ? "✓ Installed" : "✗ Not found"}
          </li>
          <li className={env.has_wasm_pack ? "ok" : "optional"}>
            wasm-pack: {env.has_wasm_pack ? "✓ Installed" : "○ Not found (required for Web export)"}
          </li>
        </ul>
        {!env.has_rust && (
          <p className="warning">
            Rust is required for native builds.{" "}
            <a
              href="https://rustup.rs"
              target="_blank"
              rel="noopener noreferrer"
            >
              Install Rust
            </a>
          </p>
        )}
      </div>
    );
  };

  const renderTargetStep = () => {
    if (!envChecked) {
      return (
        <div className="wizard-loading">
          <p>Checking build environment...</p>
        </div>
      );
    }

    return (
      <div className="wizard-step">
        <h3>Select Export Target</h3>

        {environment && renderEnvironmentStatus(environment)}

        <div className="target-grid">
          {(Object.keys(EXPORT_TARGET_LABELS) as ExportTarget[]).map(
            (target) => {
              const isDisabled =
                (target !== "web" && target !== "current_platform" && !environment?.has_rust) ||
                (target === "web" && !environment?.has_wasm_pack);
              const isSelected = options.target === target;

              return (
                <button
                  key={target}
                  className={`target-button ${isSelected ? "selected" : ""} ${
                    isDisabled ? "disabled" : ""
                  }`}
                  onClick={() => !isDisabled && handleTargetChange(target)}
                  disabled={isDisabled}
                >
                  <span className="target-icon">
                    {target === "windows" && "🪟"}
                    {target === "macos" && "🍎"}
                    {target === "linux" && "🐧"}
                    {target === "web" && "🌐"}
                    {target === "current_platform" && "💻"}
                  </span>
                  <span className="target-label">
                    {EXPORT_TARGET_LABELS[target]}
                  </span>
                  {target === "current_platform" && environment && (
                    <span className="target-hint">
                      ({environment.current_platform})
                    </span>
                  )}
                </button>
              );
            }
          )}
        </div>

        <div className="wizard-actions">
          <button onClick={handleClose}>Cancel</button>
          <button
            className="primary"
            onClick={() => setStep("options")}
            disabled={!canProceedToOptions}
          >
            Next
          </button>
        </div>
      </div>
    );
  };

  const renderOptionsStep = () => {
    return (
      <div className="wizard-step">
        <h3>Export Options</h3>

        <div className="option-group">
          <label>Output Directory</label>
          <div className="path-input">
            <input
              type="text"
              value={options.output_dir}
              onChange={(e) =>
                setOptions((prev) => ({ ...prev, output_dir: e.target.value }))
              }
              placeholder="Select output directory..."
              readOnly
            />
            <button onClick={() => void handleSelectOutputDir()}>
              Browse...
            </button>
          </div>
        </div>

        <div className="option-group">
          <label>
            <input
              type="checkbox"
              checked={options.release_build}
              onChange={(e) =>
                setOptions((prev) => ({
                  ...prev,
                  release_build: e.target.checked,
                }))
              }
            />
            Release Build (optimized, slower to build)
          </label>
        </div>

        <div className="option-group">
          <label>
            <input
              type="checkbox"
              checked={options.optimize_assets}
              onChange={(e) =>
                setOptions((prev) => ({
                  ...prev,
                  optimize_assets: e.target.checked,
                }))
              }
            />
            Optimize Assets
          </label>
        </div>

        {options.optimize_assets && (
          <>
            <div className="option-group nested">
              <label>
                <input
                  type="checkbox"
                  checked={options.image_compression !== null}
                  onChange={(e) =>
                    setOptions((prev) => ({
                      ...prev,
                      image_compression: e.target.checked
                        ? { format: "webp", quality: 85 }
                        : null,
                    }))
                  }
                />
                Compress Images
              </label>
              {options.image_compression && (
                <div className="sub-options">
                  <label>
                    Format:
                    <select
                      value={options.image_compression.format}
                      onChange={(e) =>
                        setOptions((prev) => ({
                          ...prev,
                          image_compression: {
                            ...prev.image_compression!,
                            format: e.target.value as "webp" | "jpeg" | "png",
                          },
                        }))
                      }
                    >
                      <option value="webp">WebP (recommended)</option>
                      <option value="jpeg">JPEG</option>
                      <option value="png">PNG</option>
                    </select>
                  </label>
                  <label>
                    Quality: {options.image_compression.quality}%
                    <input
                      type="range"
                      min="10"
                      max="100"
                      value={options.image_compression.quality}
                      onChange={(e) =>
                        setOptions((prev) => ({
                          ...prev,
                          image_compression: {
                            ...prev.image_compression!,
                            quality: parseInt(e.target.value),
                          },
                        }))
                      }
                    />
                  </label>
                </div>
              )}
            </div>

            <div className="option-group nested">
              <label>
                <input
                  type="checkbox"
                  checked={options.audio_conversion !== null}
                  onChange={(e) =>
                    setOptions((prev) => ({
                      ...prev,
                      audio_conversion: e.target.checked
                        ? { format: "ogg", bitrate: 128 }
                        : null,
                    }))
                  }
                />
                Convert Audio
              </label>
              {options.audio_conversion && (
                <div className="sub-options">
                  <label>
                    Format:
                    <select
                      value={options.audio_conversion.format}
                      onChange={(e) =>
                        setOptions((prev) => ({
                          ...prev,
                          audio_conversion: {
                            ...prev.audio_conversion!,
                            format: e.target.value as "ogg" | "mp3",
                          },
                        }))
                      }
                    >
                      <option value="ogg">OGG (recommended)</option>
                      <option value="mp3">MP3</option>
                    </select>
                  </label>
                  <label>
                    Bitrate: {options.audio_conversion.bitrate} kbps
                    <input
                      type="range"
                      min="64"
                      max="320"
                      step="32"
                      value={options.audio_conversion.bitrate}
                      onChange={(e) =>
                        setOptions((prev) => ({
                          ...prev,
                          audio_conversion: {
                            ...prev.audio_conversion!,
                            bitrate: parseInt(e.target.value),
                          },
                        }))
                      }
                    />
                  </label>
                </div>
              )}
            </div>

            <div className="option-group nested">
              <label>
                <input
                  type="checkbox"
                  checked={options.exclude_unused_assets}
                  onChange={(e) =>
                    setOptions((prev) => ({
                      ...prev,
                      exclude_unused_assets: e.target.checked,
                    }))
                  }
                />
                Exclude Unused Assets
              </label>
            </div>
          </>
        )}

        <div className="option-group">
          <label>Package Format</label>
          <select
            value={options.package_format}
            onChange={(e) =>
              setOptions((prev) => ({
                ...prev,
                package_format: e.target.value as PackageFormat,
              }))
            }
            className="package-select"
          >
            {(Object.keys(PACKAGE_FORMAT_LABELS) as PackageFormat[])
              .filter((format) => {
                // Filter based on target
                if (format === "app_bundle" && options.target !== "macos") {
                  return false;
                }
                if (format === "tar_gz" && options.target === "windows") {
                  return false;
                }
                return true;
              })
              .map((format) => (
                <option key={format} value={format}>
                  {PACKAGE_FORMAT_LABELS[format]}
                </option>
              ))}
          </select>
        </div>

        <div className="wizard-actions">
          <button onClick={() => setStep("target")}>Back</button>
          <button onClick={handleClose}>Cancel</button>
          <button
            className="primary"
            onClick={() => void handleStartExport()}
            disabled={!canStartExport}
          >
            Export
          </button>
        </div>
      </div>
    );
  };

  const renderBuildStep = () => {
    return (
      <div className="wizard-step">
        <h3>
          {result
            ? result.success
              ? "Export Complete"
              : "Export Failed"
            : "Exporting..."}
        </h3>

        {progress && !result && (
          <div className="build-progress">
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${progress.progress}%` }}
              />
            </div>
            <p className="progress-message">{progress.message}</p>
            <p className="progress-stage">Stage: {progress.stage}</p>
          </div>
        )}

        {result && (
          <div className={`build-result ${result.success ? "success" : "error"}`}>
            {result.success ? (
              <>
                <p className="result-message">
                  ✓ {projectName} has been exported successfully!
                </p>
                <p className="output-path">
                  Output: <code>{result.output_path}</code>
                </p>
              </>
            ) : (
              <>
                <p className="result-message">✗ Export failed</p>
                <p className="error-message">{result.error}</p>
              </>
            )}

            {result.warnings.length > 0 && (
              <div className="warnings">
                <h4>Warnings:</h4>
                <ul>
                  {result.warnings.map((w, i) => (
                    <li key={i}>{w}</li>
                  ))}
                </ul>
              </div>
            )}
          </div>
        )}

        <div className="wizard-actions">
          {isExporting ? (
            <button onClick={() => void cancelExport()}>Cancel</button>
          ) : (
            <>
              {!result?.success && (
                <button onClick={() => setStep("options")}>Back</button>
              )}
              <button className="primary" onClick={handleClose}>
                Close
              </button>
            </>
          )}
        </div>
      </div>
    );
  };

  return (
    <div className="dialog-overlay">
      <div className="export-wizard">
        <div className="wizard-header">
          <h2>Export Project</h2>
          <div className="wizard-steps">
            <span className={step === "target" ? "active" : ""}>1. Target</span>
            <span className={step === "options" ? "active" : ""}>
              2. Options
            </span>
            <span className={step === "build" ? "active" : ""}>3. Build</span>
          </div>
        </div>

        <div className="wizard-content">
          {step === "target" && renderTargetStep()}
          {step === "options" && renderOptionsStep()}
          {step === "build" && renderBuildStep()}
        </div>
      </div>
    </div>
  );
};
