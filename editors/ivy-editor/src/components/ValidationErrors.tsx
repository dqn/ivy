import type { ValidationResult, ValidationIssue } from "../types/scenario";

interface ValidationErrorsProps {
  result: ValidationResult | null;
  onSelectCommand: (index: number) => void;
}

const IssueItem: React.FC<{
  issue: ValidationIssue;
  onSelectCommand: (index: number) => void;
}> = ({ issue, onSelectCommand }) => {
  const isError = issue.severity === "error";

  return (
    <div
      className={`validation-issue ${isError ? "error" : "warning"}`}
      onClick={() => {
        if (issue.command_index !== undefined) {
          onSelectCommand(issue.command_index);
        }
      }}
      style={{
        cursor: issue.command_index !== undefined ? "pointer" : "default",
      }}
    >
      <span className="severity-icon">{isError ? "✕" : "⚠"}</span>
      <span className="message">{issue.message}</span>
      {issue.command_index !== undefined && (
        <span className="command-ref">#{issue.command_index}</span>
      )}
      {issue.label && <span className="label-ref">[{issue.label}]</span>}
    </div>
  );
};

export const ValidationErrors: React.FC<ValidationErrorsProps> = ({
  result,
  onSelectCommand,
}) => {
  if (!result) return null;

  const errors = result.issues.filter((i) => i.severity === "error");
  const warnings = result.issues.filter((i) => i.severity === "warning");

  if (result.issues.length === 0) {
    return (
      <div className="validation-panel valid">
        <span className="check-icon">✓</span>
        <span>No issues found</span>
      </div>
    );
  }

  return (
    <div className="validation-panel">
      <div className="validation-header">
        {errors.length > 0 && (
          <span className="error-count">{errors.length} errors</span>
        )}
        {warnings.length > 0 && (
          <span className="warning-count">{warnings.length} warnings</span>
        )}
      </div>
      <div className="validation-list">
        {result.issues.map((issue, index) => (
          <IssueItem
            key={index}
            issue={issue}
            onSelectCommand={onSelectCommand}
          />
        ))}
      </div>
    </div>
  );
};
