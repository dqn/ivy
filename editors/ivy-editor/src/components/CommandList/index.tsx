import { useCallback, useMemo } from "react";
import {
  DndContext,
  closestCenter,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from "@dnd-kit/core";
import {
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import type { Command } from "../../types/scenario";
import type { CharacterDatabase } from "../../types/character";
import { SortableCommandRow } from "./SortableCommandRow";

interface CommandListProps {
  commands: Command[];
  selectedIndex: number | null;
  highlightedIndices?: number[];
  characterDatabase?: CharacterDatabase;
  onSelect: (index: number) => void;
  onAdd: (afterIndex?: number) => void;
  onRemove: (index: number) => void;
  onReorder: (fromIndex: number, toIndex: number) => void;
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

export const CommandList: React.FC<CommandListProps> = ({
  commands,
  selectedIndex,
  highlightedIndices = [],
  characterDatabase,
  onSelect,
  onAdd,
  onRemove,
  onReorder,
}) => {
  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    }),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  // Map speakers to character names
  const speakerCharacterMap = useMemo(() => {
    const map = new Map<string, string>();
    if (!characterDatabase) return map;

    for (const [charName, charDef] of Object.entries(
      characterDatabase.characters
    )) {
      // Character name itself
      map.set(charName, charName);
      // Aliases
      for (const alias of charDef.aliases ?? []) {
        map.set(alias, charName);
      }
    }
    return map;
  }, [characterDatabase]);

  const handleDragEnd = useCallback(
    (event: DragEndEvent) => {
      const { active, over } = event;

      if (over && active.id !== over.id) {
        const oldIndex = commands.findIndex(
          (_, i) => `cmd-${i}` === active.id
        );
        const newIndex = commands.findIndex((_, i) => `cmd-${i}` === over.id);

        if (oldIndex !== -1 && newIndex !== -1) {
          onReorder(oldIndex, newIndex);
        }
      }
    },
    [commands, onReorder]
  );

  const items = commands.map((_, i) => `cmd-${i}`);

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
        <DndContext
          sensors={sensors}
          collisionDetection={closestCenter}
          onDragEnd={handleDragEnd}
        >
          <SortableContext
            items={items}
            strategy={verticalListSortingStrategy}
          >
            {commands.map((cmd, index) => {
              const type = getCommandType(cmd);
              return (
                <SortableCommandRow
                  key={`cmd-${index}`}
                  id={`cmd-${index}`}
                  index={index}
                  command={cmd}
                  isSelected={selectedIndex === index}
                  isHighlighted={highlightedIndices.includes(index)}
                  typeColor={typeColors[type]}
                  speakerCharacterMap={speakerCharacterMap}
                  onSelect={() => {
                    onSelect(index);
                  }}
                  onAddAfter={() => {
                    onAdd(index);
                  }}
                  onRemove={() => {
                    onRemove(index);
                  }}
                />
              );
            })}
          </SortableContext>
        </DndContext>
      </div>
    </div>
  );
};
