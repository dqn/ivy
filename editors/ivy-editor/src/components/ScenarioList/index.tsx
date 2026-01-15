import type { ScenarioRef } from "../../types/project";
import "./ScenarioList.css";

interface Props {
  scenarios: ScenarioRef[];
  activeScenarioPath: string | null;
  entryScenario: string;
  onSelect: (path: string) => void;
  onAdd: () => void;
}

export const ScenarioList: React.FC<Props> = ({
  scenarios,
  activeScenarioPath,
  entryScenario,
  onSelect,
  onAdd,
}) => {
  return (
    <div className="scenario-list">
      <div className="scenario-list-header">
        <span>Scenarios</span>
        <button className="add-button" onClick={onAdd} title="Add scenario">
          +
        </button>
      </div>
      <div className="scenario-list-content">
        {scenarios.map((scenario) => (
          <div
            key={scenario.path}
            className={`scenario-row ${activeScenarioPath === scenario.path ? "selected" : ""}`}
            onClick={() => onSelect(scenario.path)}
          >
            <div className="scenario-icon">
              {scenario.path === entryScenario ? "▶" : "📄"}
            </div>
            <div className="scenario-info">
              <span className="scenario-name">
                {scenario.chapter || getFileName(scenario.path)}
              </span>
              <span className="scenario-path">{scenario.path}</span>
            </div>
            {scenario.path === entryScenario && (
              <span className="entry-indicator" title="Entry point">
                ★
              </span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

function getFileName(path: string): string {
  const parts = path.split("/");
  return parts[parts.length - 1] ?? path;
}
