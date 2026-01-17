import { useState, useMemo } from "react";
import type { Scenario, Command, LocalizedString } from "../../types/scenario";
import "./styles.css";

interface TranslationTableProps {
  scenario: Scenario | null;
  onUpdateCommand: (index: number, command: Command) => void;
  onSelectCommand: (index: number) => void;
}

interface TranslationEntry {
  commandIndex: number;
  field: "speaker" | "text" | "choice";
  choiceIndex?: number;
  value: LocalizedString;
}

const COMMON_LANGUAGES = [
  { code: "en", name: "English" },
  { code: "ja", name: "Japanese" },
  { code: "zh", name: "Chinese" },
  { code: "ko", name: "Korean" },
  { code: "es", name: "Spanish" },
  { code: "fr", name: "French" },
  { code: "de", name: "German" },
];

function isLocalizedMap(value: LocalizedString | undefined): value is Record<string, string> {
  return typeof value === "object" && value !== null;
}

function getSimpleValue(value: LocalizedString | undefined): string {
  if (!value) {return "";}
  if (typeof value === "string") {return value;}
  return Object.values(value)[0] || "";
}

function getLocalizedValue(value: LocalizedString | undefined, lang: string): string {
  if (!value) {return "";}
  if (typeof value === "string") {
    return lang === "en" ? value : "";
  }
  return value[lang] || "";
}

function setLocalizedValue(
  original: LocalizedString | undefined,
  lang: string,
  newValue: string,
  languages: string[]
): LocalizedString | undefined {
  // If we only have one language and it's the default, keep it simple
  if (languages.length === 1 && lang === "en" && !isLocalizedMap(original)) {
    return newValue || undefined;
  }

  // Convert to localized map
  const map: Record<string, string> = isLocalizedMap(original)
    ? { ...original }
    : original
      ? { en: original }
      : {};

  const updated: Record<string, string> = {};
  Object.keys(map).forEach((key) => {
    if (key !== lang) {
      updated[key] = map[key];
    }
  });
  if (newValue) {
    updated[lang] = newValue;
  }

  // If only one entry and it's "en", convert back to simple string
  const keys = Object.keys(updated);
  if (keys.length === 0) {
    return undefined;
  }
  if (keys.length === 1 && keys[0] === "en") {
    return updated.en;
  }

  return updated;
}

export const TranslationTable: React.FC<TranslationTableProps> = ({
  scenario,
  onUpdateCommand,
  onSelectCommand,
}) => {
  const [selectedLanguages, setSelectedLanguages] = useState<string[]>(["en", "ja"]);
  const [filter, setFilter] = useState<"all" | "missing" | "translated">("all");
  const [searchQuery, setSearchQuery] = useState("");

  // Extract all translatable strings
  const entries = useMemo<TranslationEntry[]>(() => {
    if (!scenario) {return [];}

    const result: TranslationEntry[] = [];

    scenario.script.forEach((cmd, index) => {
      if (cmd.speaker) {
        result.push({
          commandIndex: index,
          field: "speaker",
          value: cmd.speaker,
        });
      }

      if (cmd.text) {
        result.push({
          commandIndex: index,
          field: "text",
          value: cmd.text,
        });
      }

      if (cmd.choices) {
        cmd.choices.forEach((choice, choiceIdx) => {
          if (choice.label) {
            result.push({
              commandIndex: index,
              field: "choice",
              choiceIndex: choiceIdx,
              value: choice.label,
            });
          }
        });
      }
    });

    return result;
  }, [scenario]);

  // Filter entries
  const filteredEntries = useMemo(() => {
    let result = entries;

    // Filter by translation status
    if (filter === "missing") {
      result = result.filter((entry) => {
        return selectedLanguages.some((lang) => {
          const value = getLocalizedValue(entry.value, lang);
          return !value;
        });
      });
    } else if (filter === "translated") {
      result = result.filter((entry) => {
        return selectedLanguages.every((lang) => {
          const value = getLocalizedValue(entry.value, lang);
          return !!value;
        });
      });
    }

    // Filter by search query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter((entry) => {
        const simpleValue = getSimpleValue(entry.value).toLowerCase();
        return simpleValue.includes(query);
      });
    }

    return result;
  }, [entries, filter, selectedLanguages, searchQuery]);

  const handleUpdateEntry = (
    entry: TranslationEntry,
    lang: string,
    newValue: string
  ) => {
    if (!scenario) {return;}

    const cmd = scenario.script[entry.commandIndex];
    const updatedValue = setLocalizedValue(entry.value, lang, newValue, selectedLanguages);

    if (entry.field === "speaker") {
      onUpdateCommand(entry.commandIndex, { ...cmd, speaker: updatedValue });
    } else if (entry.field === "text") {
      onUpdateCommand(entry.commandIndex, { ...cmd, text: updatedValue });
    } else if (entry.field === "choice" && entry.choiceIndex !== undefined) {
      const choices = [...(cmd.choices || [])];
      choices[entry.choiceIndex] = {
        ...choices[entry.choiceIndex],
        label: updatedValue || "",
      };
      onUpdateCommand(entry.commandIndex, { ...cmd, choices });
    }
  };

  const toggleLanguage = (code: string) => {
    if (selectedLanguages.includes(code)) {
      if (selectedLanguages.length > 1) {
        setSelectedLanguages(selectedLanguages.filter((l) => l !== code));
      }
    } else {
      setSelectedLanguages([...selectedLanguages, code]);
    }
  };

  // Statistics
  const stats = useMemo(() => {
    const total = entries.length;
    const byLang: Record<string, { translated: number; missing: number }> = {};

    selectedLanguages.forEach((lang) => {
      byLang[lang] = { translated: 0, missing: 0 };
      entries.forEach((entry) => {
        if (getLocalizedValue(entry.value, lang)) {
          byLang[lang].translated++;
        } else {
          byLang[lang].missing++;
        }
      });
    });

    return { total, byLang };
  }, [entries, selectedLanguages]);

  if (!scenario) {
    return (
      <div className="translation-table empty">
        <p>No scenario loaded</p>
      </div>
    );
  }

  return (
    <div className="translation-table">
      {/* Header */}
      <div className="translation-header">
        <h3>Translation Table</h3>
        <div className="translation-stats">
          {selectedLanguages.map((lang) => (
            <span key={lang} className="lang-stat">
              {lang.toUpperCase()}: {stats.byLang[lang]?.translated || 0}/{stats.total}
            </span>
          ))}
        </div>
      </div>

      {/* Controls */}
      <div className="translation-controls">
        <div className="language-selector">
          {COMMON_LANGUAGES.map((lang) => (
            <button
              key={lang.code}
              className={`lang-button ${selectedLanguages.includes(lang.code) ? "active" : ""}`}
              onClick={() => toggleLanguage(lang.code)}
              title={lang.name}
            >
              {lang.code.toUpperCase()}
            </button>
          ))}
        </div>

        <div className="filter-controls">
          <select
            value={filter}
            onChange={(e) => setFilter(e.target.value as typeof filter)}
          >
            <option value="all">All ({entries.length})</option>
            <option value="missing">Missing translations</option>
            <option value="translated">Fully translated</option>
          </select>

          <input
            type="text"
            placeholder="Search..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </div>

      {/* Table */}
      <div className="translation-content">
        <table>
          <thead>
            <tr>
              <th className="col-index">#</th>
              <th className="col-type">Type</th>
              {selectedLanguages.map((lang) => (
                <th key={lang} className="col-lang">
                  {lang.toUpperCase()}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {filteredEntries.map((entry) => (
              <tr
                key={`${entry.commandIndex}-${entry.field}-${entry.choiceIndex ?? ""}`}
                onClick={() => onSelectCommand(entry.commandIndex)}
              >
                <td className="col-index">{entry.commandIndex + 1}</td>
                <td className="col-type">
                  <span className={`type-badge type-${entry.field}`}>
                    {entry.field === "choice"
                      ? `Choice ${(entry.choiceIndex ?? 0) + 1}`
                      : entry.field}
                  </span>
                </td>
                {selectedLanguages.map((lang) => {
                  const value = getLocalizedValue(entry.value, lang);
                  const isEmpty = !value;
                  return (
                    <td
                      key={lang}
                      className={`col-lang ${isEmpty ? "empty" : ""}`}
                    >
                      <input
                        type="text"
                        value={value}
                        placeholder={isEmpty ? "Missing" : ""}
                        onChange={(e) =>
                          handleUpdateEntry(entry, lang, e.target.value)
                        }
                        onClick={(e) => e.stopPropagation()}
                      />
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>

        {filteredEntries.length === 0 && (
          <div className="no-results">
            {searchQuery
              ? "No matching entries"
              : filter === "missing"
                ? "All translations complete!"
                : "No translatable content"}
          </div>
        )}
      </div>
    </div>
  );
};
