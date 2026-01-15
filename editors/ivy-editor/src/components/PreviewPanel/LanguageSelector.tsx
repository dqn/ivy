interface Props {
  language: string;
  onChange: (lang: string) => void;
}

const LANGUAGES = [
  { code: "en", name: "English" },
  { code: "ja", name: "Japanese" },
  { code: "zh", name: "Chinese" },
  { code: "ko", name: "Korean" },
  { code: "es", name: "Spanish" },
  { code: "fr", name: "French" },
  { code: "de", name: "German" },
];

export const LanguageSelector: React.FC<Props> = ({ language, onChange }) => {
  return (
    <div className="language-selector">
      <label>Preview Language:</label>
      <select value={language} onChange={(e) => onChange(e.target.value)}>
        {LANGUAGES.map((lang) => (
          <option key={lang.code} value={lang.code}>
            {lang.code.toUpperCase()} - {lang.name}
          </option>
        ))}
      </select>
    </div>
  );
};
