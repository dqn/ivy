import { useCallback } from "react";
import { useAssetPicker } from "../../hooks/useAssetPicker";

interface AssetFieldProps {
  label: string;
  value: string | undefined;
  baseDir: string | null;
  accept: string[];
  onChange: (value: string | undefined) => void;
}

export const AssetField: React.FC<AssetFieldProps> = ({
  label,
  value,
  baseDir,
  accept,
  onChange,
}) => {
  const { handleDrop, handleDragOver, handleBrowse } = useAssetPicker({
    baseDir,
    accept,
    onSelect: onChange,
  });

  const handleClear = useCallback(() => {
    onChange(undefined);
  }, [onChange]);

  return (
    <div className="asset-field">
      <label>{label}</label>
      <div
        className={`asset-drop-zone ${value ? "has-value" : ""}`}
        onDrop={(e) => {
          void handleDrop(e);
        }}
        onDragOver={handleDragOver}
      >
        {value ? (
          <div className="asset-value">
            <span className="asset-path">{value}</span>
            <button
              type="button"
              className="clear-button"
              onClick={handleClear}
            >
              x
            </button>
          </div>
        ) : (
          <div className="drop-placeholder">
            Drop file here or{" "}
            <button
              type="button"
              className="browse-button"
              onClick={() => {
                void handleBrowse();
              }}
            >
              browse
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
