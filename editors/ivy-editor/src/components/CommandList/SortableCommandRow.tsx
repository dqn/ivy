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
  typeIcon: string;
  typeName: string;
  speakerCharacterMap?: Map<string, string>;
  onSelect: () => void;
  onAddAfter: () => void;
  onRemove: () => void;
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
  if (cmd.label) {return `[${cmd.label}]`;}
  if (cmd.choices) {return `Choices (${cmd.choices.length})`;}
  if (cmd.jump) {return `-> ${cmd.jump}`;}
  if (cmd.if) {return `if ${cmd.if.var} == ${cmd.if.is} -> ${cmd.if.jump}`;}
  if (cmd.set) {return `${cmd.set.name} = ${cmd.set.value}`;}
  if (cmd.background) {return `BG: ${cmd.background || "(clear)"}`;}
  if (cmd.character) {return `Char: ${cmd.character || "(clear)"}`;}
  if (cmd.bgm) {return `BGM: ${cmd.bgm || "(stop)"}`;}
  if (cmd.se) {return `SE: ${cmd.se}`;}
  if (cmd.video) {return `Video: ${cmd.video.path}`;}
  if (cmd.wait !== undefined) {return `Wait ${cmd.wait}s`;}
  if (cmd.text) {return getLocalizedText(cmd.text);}
  return "(empty)";
}

function getSpeakerText(speaker: LocalizedString): string {
  if (typeof speaker === "string") {return speaker;}
  return Object.values(speaker)[0] ?? "";
}

export const SortableCommandRow: React.FC<SortableCommandRowProps> = ({
  id,
  index,
  command,
  isSelected,
  isHighlighted = false,
  typeColor,
  typeIcon,
  typeName,
  speakerCharacterMap,
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

  // Check if speaker is mapped to a character
  const speakerText = command.speaker ? getSpeakerText(command.speaker) : null;
  const mappedCharacter =
    speakerText && speakerCharacterMap
      ? speakerCharacterMap.get(speakerText)
      : null;

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
      <span className="command-type" style={{ backgroundColor: typeColor }} title={typeName}>
        <span className="command-type-icon">{typeIcon}</span>
      </span>
      {speakerText && (
        <span
          className={`speaker-indicator ${mappedCharacter ? "mapped" : "unmapped"}`}
          title={
            mappedCharacter
              ? `${speakerText} → ${mappedCharacter}`
              : `${speakerText} (not mapped)`
          }
        >
          {mappedCharacter ? "●" : "○"}
        </span>
      )}
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
