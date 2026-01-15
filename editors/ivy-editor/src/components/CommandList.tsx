import type { Command, LocalizedString } from "../types/scenario";

interface CommandListProps {
  commands: Command[];
  selectedIndex: number | null;
  onSelect: (index: number) => void;
  onAdd: (afterIndex?: number) => void;
  onRemove: (index: number) => void;
}

function getCommandType(cmd: Command): string {
  if (cmd.label) return "label";
  if (cmd.choices) return "choice";
  if (cmd.jump) return "jump";
  if (cmd.if) return "if";
  if (cmd.set) return "set";
  if (cmd.background) return "background";
  if (cmd.character) return "character";
  if (cmd.bgm) return "bgm";
  if (cmd.se) return "se";
  if (cmd.video) return "video";
  if (cmd.wait !== undefined) return "wait";
  if (cmd.text) return "text";
  return "empty";
}

function getCommandPreview(cmd: Command): string {
  if (cmd.label) return `[${cmd.label}]`;
  if (cmd.choices) return `Choices (${cmd.choices.length})`;
  if (cmd.jump) return `→ ${cmd.jump}`;
  if (cmd.if) return `if ${cmd.if.var} == ${cmd.if.is} → ${cmd.if.jump}`;
  if (cmd.set) return `${cmd.set.name} = ${cmd.set.value}`;
  if (cmd.background) return `BG: ${cmd.background || "(clear)"}`;
  if (cmd.character) return `Char: ${cmd.character || "(clear)"}`;
  if (cmd.bgm) return `BGM: ${cmd.bgm || "(stop)"}`;
  if (cmd.se) return `SE: ${cmd.se}`;
  if (cmd.video) return `Video: ${cmd.video.path}`;
  if (cmd.wait !== undefined) return `Wait ${cmd.wait}s`;
  if (cmd.text) return getLocalizedText(cmd.text);
  return "(empty)";
}

function getLocalizedText(text: LocalizedString): string {
  if (typeof text === "string") {
    return text.length > 50 ? text.slice(0, 50) + "..." : text;
  }
  const first = Object.values(text)[0];
  if (first) {
    return first.length > 50 ? first.slice(0, 50) + "..." : first;
  }
  return "";
}

const typeColors: Record<string, string> = {
  label: "#4a90d9",
  choice: "#9b59b6",
  jump: "#e67e22",
  if: "#e74c3c",
  set: "#16a085",
  background: "#27ae60",
  character: "#2ecc71",
  bgm: "#f39c12",
  se: "#f1c40f",
  video: "#8e44ad",
  wait: "#95a5a6",
  text: "#3498db",
  empty: "#bdc3c7",
};

export const CommandList: React.FC<CommandListProps> = ({
  commands,
  selectedIndex,
  onSelect,
  onAdd,
  onRemove,
}) => {
  return (
    <div className="command-list">
      <div className="command-list-header">
        <span>Commands ({commands.length})</span>
        <button
          className="add-button"
          onClick={() => {
            onAdd();
          }}
          title="Add command at end"
        >
          +
        </button>
      </div>
      <div className="command-list-content">
        {commands.map((cmd, index) => {
          const type = getCommandType(cmd);
          const isSelected = selectedIndex === index;

          return (
            <div
              key={index}
              className={`command-row ${isSelected ? "selected" : ""}`}
              onClick={() => {
                onSelect(index);
              }}
            >
              <span className="command-index">{index}</span>
              <span
                className="command-type"
                style={{ backgroundColor: typeColors[type] }}
              >
                {type}
              </span>
              <span className="command-preview">{getCommandPreview(cmd)}</span>
              <div className="command-actions">
                <button
                  className="action-button"
                  onClick={(e) => {
                    e.stopPropagation();
                    onAdd(index);
                  }}
                  title="Add after"
                >
                  +
                </button>
                <button
                  className="action-button delete"
                  onClick={(e) => {
                    e.stopPropagation();
                    onRemove(index);
                  }}
                  title="Remove"
                >
                  ×
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
