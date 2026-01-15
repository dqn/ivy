import { useMemo } from "react";
import type { CharacterDatabase } from "../../types/character";
import type { Scenario } from "../../types/scenario";

interface Props {
  database: CharacterDatabase;
  scenario: Scenario | null;
  onSelectCharacter?: (name: string) => void;
}

interface SpeakerInfo {
  name: string;
  mappedCharacter: string | null;
  usageCount: number;
}

export const SpeakerMapping: React.FC<Props> = ({
  database,
  scenario,
  onSelectCharacter,
}) => {
  const speakerInfos = useMemo((): SpeakerInfo[] => {
    if (!scenario) return [];

    // Extract all speakers from scenario
    const speakerCounts = new Map<string, number>();
    for (const cmd of scenario.script) {
      if (cmd.speaker) {
        const speaker =
          typeof cmd.speaker === "string"
            ? cmd.speaker
            : Object.values(cmd.speaker)[0] ?? "";
        if (speaker) {
          speakerCounts.set(speaker, (speakerCounts.get(speaker) ?? 0) + 1);
        }
      }
    }

    // Map speakers to characters
    const infos: SpeakerInfo[] = [];
    for (const [speaker, count] of speakerCounts) {
      let mappedCharacter: string | null = null;

      // Check if speaker matches a character name or alias
      for (const [charName, charDef] of Object.entries(database.characters)) {
        if (
          charName === speaker ||
          (charDef.aliases ?? []).includes(speaker)
        ) {
          mappedCharacter = charName;
          break;
        }
      }

      infos.push({
        name: speaker,
        mappedCharacter,
        usageCount: count,
      });
    }

    // Sort: unmapped first, then by usage count
    infos.sort((a, b) => {
      if (a.mappedCharacter === null && b.mappedCharacter !== null) return -1;
      if (a.mappedCharacter !== null && b.mappedCharacter === null) return 1;
      return b.usageCount - a.usageCount;
    });

    return infos;
  }, [database, scenario]);

  const unmappedCount = speakerInfos.filter(
    (info) => info.mappedCharacter === null
  ).length;

  if (speakerInfos.length === 0) {
    return (
      <div className="speaker-mapping empty">
        <p>No speakers in scenario</p>
      </div>
    );
  }

  return (
    <div className="speaker-mapping">
      <div className="speaker-mapping-header">
        <span>Speaker Mapping</span>
        {unmappedCount > 0 && (
          <span className="unmapped-badge">{unmappedCount} unmapped</span>
        )}
      </div>
      <div className="speaker-mapping-list">
        {speakerInfos.map((info) => (
          <div
            key={info.name}
            className={`speaker-mapping-item ${info.mappedCharacter ? "mapped" : "unmapped"}`}
          >
            <span className="speaker-name">{info.name}</span>
            <span className="speaker-usage">({info.usageCount})</span>
            {info.mappedCharacter ? (
              <button
                className="mapped-character"
                onClick={() => onSelectCharacter?.(info.mappedCharacter!)}
                title={`Go to ${info.mappedCharacter}`}
              >
                → {info.mappedCharacter}
              </button>
            ) : (
              <span className="no-mapping">Not mapped</span>
            )}
          </div>
        ))}
      </div>
      {unmappedCount > 0 && (
        <div className="speaker-mapping-hint">
          Tip: Add speaker names as aliases in character definitions
        </div>
      )}
    </div>
  );
};
