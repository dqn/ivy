import { useState } from "react";
import type { LocalizedString } from "../../types/scenario";

interface LocalizedStringEditorProps {
  label: string;
  value: LocalizedString | undefined;
  placeholder?: string;
  multiline?: boolean;
  required?: boolean;
  onChange: (value: LocalizedString | undefined) => void;
}

const COMMON_LANGUAGES = [
  { code: "en", name: "English" },
  { code: "ja", name: "Japanese" },
  { code: "zh", name: "Chinese" },
  { code: "ko", name: "Korean" },
  { code: "es", name: "Spanish" },
  { code: "fr", name: "French" },
  { code: "de", name: "German" },
  { code: "pt", name: "Portuguese" },
  { code: "ru", name: "Russian" },
  { code: "it", name: "Italian" },
];

function isLocalizedMap(value: LocalizedString | undefined): value is Record<string, string> {
  return typeof value === "object" && value !== null;
}

function getSimpleValue(value: LocalizedString | undefined): string {
  if (!value) return "";
  if (typeof value === "string") return value;
  return Object.values(value)[0] || "";
}

function getLocalizedMap(value: LocalizedString | undefined): Record<string, string> {
  if (!value) return {};
  if (typeof value === "string") return { en: value };
  return value;
}

export const LocalizedStringEditor: React.FC<LocalizedStringEditorProps> = ({
  label,
  value,
  placeholder,
  multiline = false,
  required = false,
  onChange,
}) => {
  const [isMultilingual, setIsMultilingual] = useState(isLocalizedMap(value));
  const [languages, setLanguages] = useState<string[]>(() => {
    if (isLocalizedMap(value)) {
      return Object.keys(value);
    }
    return ["en"];
  });

  const handleModeToggle = () => {
    if (isMultilingual) {
      // Convert to simple string
      const simpleValue = getSimpleValue(value);
      setIsMultilingual(false);
      onChange(simpleValue || undefined);
    } else {
      // Convert to localized map
      const currentValue = getSimpleValue(value);
      setIsMultilingual(true);
      setLanguages(["en"]);
      onChange(currentValue ? { en: currentValue } : undefined);
    }
  };

  const handleSimpleChange = (newValue: string) => {
    onChange(newValue || undefined);
  };

  const handleLocalizedChange = (langCode: string, newValue: string) => {
    const currentMap = getLocalizedMap(value);
    const updated = { ...currentMap, [langCode]: newValue };

    // Remove empty entries
    Object.keys(updated).forEach((key) => {
      if (!updated[key]) {
        delete updated[key];
      }
    });

    if (Object.keys(updated).length === 0) {
      onChange(undefined);
    } else {
      onChange(updated);
    }
  };

  const addLanguage = (langCode: string) => {
    if (!languages.includes(langCode)) {
      setLanguages([...languages, langCode]);
    }
  };

  const removeLanguage = (langCode: string) => {
    if (languages.length <= 1) return;

    setLanguages(languages.filter((l) => l !== langCode));

    const currentMap = getLocalizedMap(value);
    const updated = { ...currentMap };
    delete updated[langCode];

    if (Object.keys(updated).length === 0) {
      onChange(undefined);
    } else {
      onChange(updated);
    }
  };

  const availableLanguages = COMMON_LANGUAGES.filter(
    (lang) => !languages.includes(lang.code)
  );

  const localizedMap = getLocalizedMap(value);

  return (
    <div className="localized-string-editor">
      <div className="localized-header">
        <label>
          {label}
          {required && <span className="required">*</span>}
        </label>
        <button
          type="button"
          className={`multilingual-toggle ${isMultilingual ? "active" : ""}`}
          onClick={handleModeToggle}
          title={isMultilingual ? "Switch to simple text" : "Enable translations"}
        >
          {isMultilingual ? "🌐" : "A"}
        </button>
      </div>

      {!isMultilingual ? (
        // Simple mode
        <div className="simple-input">
          {multiline ? (
            <textarea
              value={getSimpleValue(value)}
              onChange={(e) => handleSimpleChange(e.target.value)}
              placeholder={placeholder}
              rows={3}
            />
          ) : (
            <input
              type="text"
              value={getSimpleValue(value)}
              onChange={(e) => handleSimpleChange(e.target.value)}
              placeholder={placeholder}
            />
          )}
        </div>
      ) : (
        // Multilingual mode
        <div className="multilingual-input">
          {languages.map((langCode) => {
            const langInfo = COMMON_LANGUAGES.find((l) => l.code === langCode);
            return (
              <div key={langCode} className="language-row">
                <div className="language-label">
                  <span className="lang-code">{langCode.toUpperCase()}</span>
                  <span className="lang-name">{langInfo?.name || langCode}</span>
                </div>
                <div className="language-input">
                  {multiline ? (
                    <textarea
                      value={localizedMap[langCode] || ""}
                      onChange={(e) => handleLocalizedChange(langCode, e.target.value)}
                      placeholder={`${placeholder || "Text"} (${langInfo?.name || langCode})`}
                      rows={2}
                    />
                  ) : (
                    <input
                      type="text"
                      value={localizedMap[langCode] || ""}
                      onChange={(e) => handleLocalizedChange(langCode, e.target.value)}
                      placeholder={`${placeholder || "Text"} (${langInfo?.name || langCode})`}
                    />
                  )}
                </div>
                {languages.length > 1 && (
                  <button
                    type="button"
                    className="remove-lang-button"
                    onClick={() => removeLanguage(langCode)}
                    title="Remove language"
                  >
                    ×
                  </button>
                )}
              </div>
            );
          })}

          {availableLanguages.length > 0 && (
            <div className="add-language">
              <select
                value=""
                onChange={(e) => {
                  if (e.target.value) {
                    addLanguage(e.target.value);
                  }
                }}
              >
                <option value="">+ Add language...</option>
                {availableLanguages.map((lang) => (
                  <option key={lang.code} value={lang.code}>
                    {lang.name} ({lang.code})
                  </option>
                ))}
              </select>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
