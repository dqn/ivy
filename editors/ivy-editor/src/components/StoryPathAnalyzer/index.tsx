import { useState, useMemo } from "react";
import type { Scenario } from "../../types/scenario";
import "./styles.css";

interface StoryPathAnalyzerProps {
  scenario: Scenario | null;
  onSelectCommand: (index: number) => void;
}

interface LabelInfo {
  name: string;
  definedAt: number;
  referencedFrom: number[];
  isReachable: boolean;
}

interface PathIssue {
  type: "unreachable" | "dead_end" | "orphan_choice" | "infinite_loop";
  message: string;
  commandIndex?: number;
  label?: string;
}

export const StoryPathAnalyzer: React.FC<StoryPathAnalyzerProps> = ({
  scenario,
  onSelectCommand,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const analysis = useMemo(() => {
    if (!scenario) {
      return { labels: [], issues: [], reachableCount: 0, totalCount: 0 };
    }

    const labels: Map<string, LabelInfo> = new Map();
    const issues: PathIssue[] = [];

    // First pass: collect all label definitions
    scenario.script.forEach((cmd, index) => {
      if (cmd.label) {
        labels.set(cmd.label, {
          name: cmd.label,
          definedAt: index,
          referencedFrom: [],
          isReachable: false,
        });
      }
    });

    // Second pass: collect all references to labels
    scenario.script.forEach((cmd, index) => {
      // Direct jump
      if (cmd.jump) {
        const label = labels.get(cmd.jump);
        if (label) {
          label.referencedFrom.push(index);
        }
      }

      // Conditional jump
      if (cmd.if && typeof cmd.if === "object" && "jump" in cmd.if) {
        const jump = (cmd.if as { jump: string }).jump;
        const label = labels.get(jump);
        if (label) {
          label.referencedFrom.push(index);
        }
      }

      // Choice jumps
      if (cmd.choices) {
        cmd.choices.forEach((choice) => {
          const label = labels.get(choice.jump);
          if (label) {
            label.referencedFrom.push(index);
          }
        });
      }
    });

    // Mark reachable labels using BFS from start
    const visited = new Set<number>();
    const queue: number[] = [0]; // Start from first command

    while (queue.length > 0) {
      const idx = queue.shift()!;
      if (visited.has(idx) || idx < 0 || idx >= scenario.script.length) {
        continue;
      }
      visited.add(idx);

      const cmd = scenario.script[idx];

      // Mark this label as reachable if it has one
      if (cmd.label) {
        const label = labels.get(cmd.label);
        if (label) {
          label.isReachable = true;
        }
      }

      // If not a jump/choice, flow continues to next command
      if (!cmd.jump && !cmd.choices) {
        queue.push(idx + 1);
      }

      // Handle jump
      if (cmd.jump) {
        const targetLabel = labels.get(cmd.jump);
        if (targetLabel) {
          queue.push(targetLabel.definedAt);
        }
      }

      // Handle conditional jump (both paths)
      if (cmd.if && typeof cmd.if === "object" && "jump" in cmd.if) {
        const jump = (cmd.if as { jump: string }).jump;
        const targetLabel = labels.get(jump);
        if (targetLabel) {
          queue.push(targetLabel.definedAt);
        }
        // Also continue to next command (condition not met)
        queue.push(idx + 1);
      }

      // Handle choices
      if (cmd.choices) {
        cmd.choices.forEach((choice) => {
          const targetLabel = labels.get(choice.jump);
          if (targetLabel) {
            queue.push(targetLabel.definedAt);
          }
        });
      }
    }

    // Analyze issues
    labels.forEach((label) => {
      // Check for unreachable labels
      if (!label.isReachable && label.referencedFrom.length === 0) {
        issues.push({
          type: "unreachable",
          message: `Label "${label.name}" is never referenced and unreachable from start`,
          commandIndex: label.definedAt,
          label: label.name,
        });
      } else if (!label.isReachable) {
        issues.push({
          type: "unreachable",
          message: `Label "${label.name}" is referenced but unreachable from start`,
          commandIndex: label.definedAt,
          label: label.name,
        });
      }
    });

    // Check for dead ends (commands without flow continuation)
    scenario.script.forEach((cmd, index) => {
      // Last command without jump or choices is potentially a dead end
      if (index === scenario.script.length - 1) {
        if (!cmd.jump && !cmd.choices) {
          // This is the natural end of the scenario, not an issue
        }
      } else {
        // Check for orphan choices (choices where jump target doesn't exist)
        if (cmd.choices) {
          cmd.choices.forEach((choice, choiceIdx) => {
            if (!labels.has(choice.jump)) {
              issues.push({
                type: "orphan_choice",
                message: `Choice ${choiceIdx + 1} jumps to undefined label "${choice.jump}"`,
                commandIndex: index,
              });
            }
          });
        }
      }
    });

    const reachableCount = visited.size;
    const totalCount = scenario.script.length;

    return {
      labels: Array.from(labels.values()),
      issues,
      reachableCount,
      totalCount,
    };
  }, [scenario]);

  if (!scenario) {
    return null;
  }

  const unreachableLabels = analysis.labels.filter((l) => !l.isReachable);
  const hasIssues = analysis.issues.length > 0;

  return (
    <div className="story-path-analyzer">
      <div
        className="analyzer-header"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className="toggle-icon">{isExpanded ? "▼" : "▶"}</span>
        <h4>Story Path Analysis</h4>
        {hasIssues && (
          <span className="issue-badge">{analysis.issues.length}</span>
        )}
      </div>

      {isExpanded && (
        <div className="analyzer-content">
          {/* Statistics */}
          <div className="analyzer-stats">
            <div className="stat">
              <span className="stat-value">{analysis.reachableCount}</span>
              <span className="stat-label">/ {analysis.totalCount} reachable</span>
            </div>
            <div className="stat">
              <span className="stat-value">{analysis.labels.length}</span>
              <span className="stat-label">labels</span>
            </div>
            <div className="stat">
              <span className={`stat-value ${unreachableLabels.length > 0 ? "warning" : ""}`}>
                {unreachableLabels.length}
              </span>
              <span className="stat-label">unreachable</span>
            </div>
          </div>

          {/* Issues */}
          {analysis.issues.length > 0 && (
            <div className="analyzer-issues">
              <h5>Issues</h5>
              {analysis.issues.map((issue, idx) => (
                <div
                  key={idx}
                  className={`issue-item issue-${issue.type}`}
                  onClick={() => {
                    if (issue.commandIndex !== undefined) {
                      onSelectCommand(issue.commandIndex);
                    }
                  }}
                >
                  <span className="issue-icon">
                    {issue.type === "unreachable" && "⚠"}
                    {issue.type === "dead_end" && "🛑"}
                    {issue.type === "orphan_choice" && "❌"}
                    {issue.type === "infinite_loop" && "🔄"}
                  </span>
                  <span className="issue-message">{issue.message}</span>
                  {issue.commandIndex !== undefined && (
                    <span className="issue-location">#{issue.commandIndex + 1}</span>
                  )}
                </div>
              ))}
            </div>
          )}

          {/* Unreachable Labels */}
          {unreachableLabels.length > 0 && (
            <div className="unreachable-labels">
              <h5>Unreachable Labels</h5>
              {unreachableLabels.map((label) => (
                <div
                  key={label.name}
                  className="label-item unreachable"
                  onClick={() => onSelectCommand(label.definedAt)}
                >
                  <span className="label-name">{label.name}</span>
                  <span className="label-location">#{label.definedAt + 1}</span>
                </div>
              ))}
            </div>
          )}

          {/* No Issues */}
          {analysis.issues.length === 0 && unreachableLabels.length === 0 && (
            <div className="no-issues">
              ✓ All story paths are reachable
            </div>
          )}
        </div>
      )}
    </div>
  );
};
