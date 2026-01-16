import { useState } from "react";
import type { SaveDataValidationResult, ValidationIssue } from "../../types/savedata";

interface Props {
  result: SaveDataValidationResult;
}

type TabType = "overview" | "visual" | "issues";

const IssueItem: React.FC<{ issue: ValidationIssue }> = ({ issue }) => (
  <div className={`issue-item ${issue.severity}`}>
    <span className={`issue-severity ${issue.severity}`}>
      {issue.severity.toUpperCase()}
    </span>
    <div className="issue-content">
      <div className="issue-message">{issue.message}</div>
      {issue.details && <div className="issue-details">{issue.details}</div>}
      <div className="issue-code">[{issue.code}]</div>
    </div>
  </div>
);

export const ValidationResultPanel: React.FC<Props> = ({ result }) => {
  const [activeTab, setActiveTab] = useState<TabType>("overview");

  const fileName = result.file_path.split("/").pop() || result.file_path;

  return (
    <div className="validation-result-panel">
      <div className="result-header">
        <span className="result-file-name" title={result.file_path}>
          {fileName}
        </span>
        <div className="result-status">
          <div className="issue-counts">
            {result.error_count > 0 && (
              <span className="issue-count error">
                <span className="issue-count-value">{result.error_count}</span>{" "}
                errors
              </span>
            )}
            {result.warning_count > 0 && (
              <span className="issue-count warning">
                <span className="issue-count-value">{result.warning_count}</span>{" "}
                warnings
              </span>
            )}
            {result.info_count > 0 && (
              <span className="issue-count info">
                <span className="issue-count-value">{result.info_count}</span>{" "}
                info
              </span>
            )}
          </div>
          <span className={`status-badge ${result.valid ? "valid" : "invalid"}`}>
            {result.valid ? "VALID" : "INVALID"}
          </span>
        </div>
      </div>

      <div className="result-tabs">
        <button
          className={activeTab === "overview" ? "active" : ""}
          onClick={() => setActiveTab("overview")}
        >
          Overview
        </button>
        <button
          className={activeTab === "visual" ? "active" : ""}
          onClick={() => setActiveTab("visual")}
        >
          Visual
        </button>
        <button
          className={activeTab === "issues" ? "active" : ""}
          onClick={() => setActiveTab("issues")}
        >
          Issues ({result.issues.length})
        </button>
      </div>

      <div className="result-content">
        {activeTab === "overview" && result.summary && (
          <div>
            <div className="overview-section">
              <div className="overview-section-title">Position</div>
              <div className="overview-grid">
                <div className="overview-item">
                  <span className="overview-label">Scenario</span>
                  <span className="overview-value" title={result.summary.scenario_path}>
                    {result.summary.scenario_path.split("/").pop() ||
                      result.summary.scenario_path}
                  </span>
                </div>
                <div className="overview-item">
                  <span className="overview-label">Index</span>
                  <span className="overview-value">
                    {result.summary.current_index}
                    {result.summary.total_commands !== null &&
                      ` / ${result.summary.total_commands}`}
                  </span>
                </div>
              </div>
            </div>

            <div className="overview-section">
              <div className="overview-section-title">Metadata</div>
              <div className="overview-grid">
                <div className="overview-item">
                  <span className="overview-label">Timestamp</span>
                  <span className="overview-value">
                    {result.summary.formatted_time}
                  </span>
                </div>
                <div className="overview-item">
                  <span className="overview-label">Variables</span>
                  <span className="overview-value">
                    {result.summary.variable_count}
                  </span>
                </div>
              </div>
            </div>
          </div>
        )}

        {activeTab === "visual" && result.summary && (
          <div className="visual-preview">
            <div className="visual-item">
              <span className="visual-label">Background</span>
              <span
                className={`visual-value ${
                  result.summary.visual.background ? "" : "empty"
                }`}
              >
                {result.summary.visual.background || "(none)"}
              </span>
            </div>
            <div className="visual-item">
              <span className="visual-label">Character</span>
              <span
                className={`visual-value ${
                  result.summary.visual.character ? "" : "empty"
                }`}
              >
                {result.summary.visual.character || "(none)"}
              </span>
            </div>
            <div className="visual-item">
              <span className="visual-label">Position</span>
              <span className="visual-value">
                {result.summary.visual.char_pos || "center"}
              </span>
            </div>
          </div>
        )}

        {activeTab === "issues" && (
          <div className="issues-list">
            {result.issues.length === 0 ? (
              <div className="empty-message">No issues found</div>
            ) : (
              result.issues.map((issue, idx) => (
                <IssueItem key={idx} issue={issue} />
              ))
            )}
          </div>
        )}
      </div>
    </div>
  );
};
