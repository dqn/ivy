import { useTranslation } from "react-i18next";
import type { ValidationResult, ValidationIssue } from "../types/scenario";
import { Tooltip } from "./Tooltip";

interface ValidationErrorsProps {
  result: ValidationResult | null;
  onSelectCommand: (index: number) => void;
}

function getHintKey(message: string): string | null {
  if (message.includes("undefined label") || message.includes("Undefined label")) {
    return "undefinedLabel";
  }
  if (message.includes("duplicate") || message.includes("Duplicate")) {
    return "duplicateLabel";
  }
  if (message.includes("unused") || message.includes("Unused")) {
    return "unusedLabel";
  }
  if (message.includes("self") || message.includes("Self")) {
    return "selfJump";
  }
  if (message.includes("circular") || message.includes("Circular")) {
    return "circularPath";
  }
  return null;
}

const IssueItem: React.FC<{
  issue: ValidationIssue;
  onSelectCommand: (index: number) => void;
}> = ({ issue, onSelectCommand }) => {
  const { t } = useTranslation();
  const isError = issue.severity === "error";
  const hintKey = getHintKey(issue.message);
  const isClickable = issue.command_index !== undefined;

  return (
    <Tooltip
      content={isClickable ? t("validation.clickToNavigate") : ""}
      position="top"
    >
      <div
        className={`validation-issue ${isError ? "error" : "warning"} ${isClickable ? "clickable" : ""}`}
        onClick={() => {
          if (issue.command_index !== undefined) {
            onSelectCommand(issue.command_index);
          }
        }}
      >
        <span className="severity-icon">{isError ? "❌" : "⚠️"}</span>
        <div className="issue-content">
          <span className="message">{issue.message}</span>
          {hintKey && (
            <span className="hint">💡 {t(`validation.hints.${hintKey}`)}</span>
          )}
        </div>
        <div className="issue-meta">
          {issue.command_index !== undefined && (
            <span className="command-ref">#{issue.command_index}</span>
          )}
          {issue.label && <span className="label-ref">[{issue.label}]</span>}
        </div>
      </div>
    </Tooltip>
  );
};

export const ValidationErrors: React.FC<ValidationErrorsProps> = ({
  result,
  onSelectCommand,
}) => {
  const { t } = useTranslation();

  if (!result) {return null;}

  const errors = result.issues.filter((i) => i.severity === "error");
  const warnings = result.issues.filter((i) => i.severity === "warning");

  if (result.issues.length === 0) {
    return (
      <div className="validation-panel valid">
        <span className="check-icon">✅</span>
        <span>{t("validation.noIssues")}</span>
      </div>
    );
  }

  return (
    <div className="validation-panel">
      <div className="validation-header">
        {errors.length > 0 && (
          <span className="error-count">
            ❌ {errors.length} {t("validation.errors")}
          </span>
        )}
        {warnings.length > 0 && (
          <span className="warning-count">
            ⚠️ {warnings.length} {t("validation.warnings")}
          </span>
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
