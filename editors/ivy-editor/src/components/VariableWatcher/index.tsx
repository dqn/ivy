import { useState, useMemo } from "react";
import type { Scenario } from "../../types/scenario";
import "./styles.css";

interface VariableWatcherProps {
  scenario: Scenario | null;
  variables: Record<string, string>;
  currentIndex: number;
}

interface VariableInfo {
  name: string;
  value: string;
  setAtIndex: number | null;
  usedAtIndices: number[];
}

export const VariableWatcher: React.FC<VariableWatcherProps> = ({
  scenario,
  variables,
  currentIndex,
}) => {
  const [isExpanded, setIsExpanded] = useState(true);
  const [filter, setFilter] = useState("");

  // Analyze variable usage in scenario
  const variableInfo = useMemo<VariableInfo[]>(() => {
    if (!scenario) return [];

    const info: Map<string, VariableInfo> = new Map();

    // Scan all commands for variable definitions and usage
    scenario.script.forEach((cmd, index) => {
      // Check for set commands
      if (cmd.set) {
        const name = typeof cmd.set === "object" && "name" in cmd.set
          ? (cmd.set as { name: string }).name
          : "";
        if (name) {
          if (!info.has(name)) {
            info.set(name, {
              name,
              value: variables[name] || "",
              setAtIndex: index,
              usedAtIndices: [],
            });
          } else {
            const existing = info.get(name)!;
            if (existing.setAtIndex === null || index < existing.setAtIndex) {
              existing.setAtIndex = index;
            }
          }
        }
      }

      // Check for if conditions
      if (cmd.if) {
        const varName = typeof cmd.if === "object" && "var" in cmd.if
          ? (cmd.if as { var: string }).var
          : "";
        if (varName) {
          if (!info.has(varName)) {
            info.set(varName, {
              name: varName,
              value: variables[varName] || "",
              setAtIndex: null,
              usedAtIndices: [index],
            });
          } else {
            info.get(varName)!.usedAtIndices.push(index);
          }
        }
      }
    });

    // Add any variables from runtime that weren't found in static analysis
    Object.keys(variables).forEach((name) => {
      if (!info.has(name)) {
        info.set(name, {
          name,
          value: variables[name],
          setAtIndex: null,
          usedAtIndices: [],
        });
      } else {
        info.get(name)!.value = variables[name];
      }
    });

    return Array.from(info.values()).sort((a, b) => a.name.localeCompare(b.name));
  }, [scenario, variables]);

  const filteredVariables = useMemo(() => {
    if (!filter) return variableInfo;
    const lowerFilter = filter.toLowerCase();
    return variableInfo.filter(
      (v) =>
        v.name.toLowerCase().includes(lowerFilter) ||
        v.value.toLowerCase().includes(lowerFilter)
    );
  }, [variableInfo, filter]);

  if (!scenario) {
    return null;
  }

  return (
    <div className="variable-watcher">
      <div
        className="variable-watcher-header"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <span className="toggle-icon">{isExpanded ? "▼" : "▶"}</span>
        <h4>Variables ({variableInfo.length})</h4>
      </div>

      {isExpanded && (
        <div className="variable-watcher-content">
          {variableInfo.length > 3 && (
            <div className="variable-filter">
              <input
                type="text"
                placeholder="Filter variables..."
                value={filter}
                onChange={(e) => setFilter(e.target.value)}
                onClick={(e) => e.stopPropagation()}
              />
            </div>
          )}

          {filteredVariables.length === 0 ? (
            <div className="no-variables">
              {variableInfo.length === 0
                ? "No variables defined"
                : "No matching variables"}
            </div>
          ) : (
            <div className="variable-list">
              {filteredVariables.map((v) => (
                <div
                  key={v.name}
                  className={`variable-item ${
                    v.setAtIndex !== null && v.setAtIndex <= currentIndex
                      ? "defined"
                      : "undefined"
                  }`}
                >
                  <span className="variable-name">{v.name}</span>
                  <span className="variable-value">
                    {v.value || <em>undefined</em>}
                  </span>
                  <div className="variable-meta">
                    {v.setAtIndex !== null && (
                      <span
                        className={`set-indicator ${
                          v.setAtIndex <= currentIndex ? "active" : ""
                        }`}
                        title={`Set at command ${v.setAtIndex + 1}`}
                      >
                        Set: #{v.setAtIndex + 1}
                      </span>
                    )}
                    {v.usedAtIndices.length > 0 && (
                      <span
                        className="usage-indicator"
                        title={`Used at: ${v.usedAtIndices.map((i) => `#${i + 1}`).join(", ")}`}
                      >
                        Used: {v.usedAtIndices.length}x
                      </span>
                    )}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
    </div>
  );
};
