interface YamlPreviewProps {
  yaml: string;
}

export const YamlPreview: React.FC<YamlPreviewProps> = ({ yaml }) => {
  return (
    <div className="yaml-preview">
      <div className="yaml-header">
        <span>YAML Preview</span>
      </div>
      <pre className="yaml-content">{yaml || "// No scenario loaded"}</pre>
    </div>
  );
};
