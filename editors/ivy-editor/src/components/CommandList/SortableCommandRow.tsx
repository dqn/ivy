import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import type { Command, LocalizedString } from "../../types/scenario";

interface SortableCommandRowProps {
  id: string;
  index: number;
  command: Command;
  isSelected: boolean;
  isHighlighted?: boolean;
  typeColor: string;
  onSelect: () => void;
  onAddAfter: () => void;
  onRemove: () => void;
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

function getCommandPreview(cmd: Command): string {
  if (cmd.label) return `[${cmd.label}]`;
  if (cmd.choices) return `Choices (${cmd.choices.length})`;
  if (cmd.jump) return `-> ${cmd.jump}`;
  if (cmd.if) return `if ${cmd.if.var} == ${cmd.if.is} -> ${cmd.if.jump}`;
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

export const SortableCommandRow: React.FC<SortableCommandRowProps> = ({
  id,
  index,
  command,
  isSelected,
  isHighlighted = false,
  typeColor,
  onSelect,
  onAddAfter,
  onRemove,
}) => {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style = {
    transform: CSS.Transform.toString(transform),
    transition,
    opacity: isDragging ? 0.5 : 1,
  };

  const type = getCommandType(command);

  return (
    <div
      ref={setNodeRef}
      style={style}
      className={`command-row ${isSelected ? "selected" : ""} ${isHighlighted ? "highlighted" : ""} ${isDragging ? "dragging" : ""}`}
      onClick={onSelect}
    >
      <span className="drag-handle" {...attributes} {...listeners}>
        &#x2807;
      </span>
      <span className="command-index">{index}</span>
      <span className="command-type" style={{ backgroundColor: typeColor }}>
        {type}
      </span>
      <span className="command-preview">{getCommandPreview(command)}</span>
      <div className="command-actions">
        <button
          className="action-button"
          onClick={(e) => {
            e.stopPropagation();
            onAddAfter();
          }}
          title="Add after"
        >
          +
        </button>
        <button
          className="action-button delete"
          onClick={(e) => {
            e.stopPropagation();
            onRemove();
          }}
          title="Remove"
        >
          x
        </button>
      </div>
    </div>
  );
};
