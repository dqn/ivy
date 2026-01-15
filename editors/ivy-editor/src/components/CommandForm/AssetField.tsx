import { useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

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
  const handleDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const files = e.dataTransfer.files;
      if (files.length === 0) return;

      const file = files[0];
      const filePath = (file as File & { path?: string }).path;

      if (filePath && baseDir) {
        try {
          const relativePath = await invoke<string>("get_relative_path", {
            baseDir,
            filePath,
          });
          onChange(relativePath);
        } catch (err) {
          console.error("Failed to get relative path:", err);
          onChange(filePath);
        }
      }
    },
    [baseDir, onChange]
  );

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "copy";
  }, []);

  const handleBrowse = useCallback(async () => {
    const extensions = accept.map((ext) => ext.replace(".", ""));
    const path = await open({
      filters: [{ name: "Assets", extensions }],
      directory: false,
      multiple: false,
    });

    if (path && baseDir) {
      try {
        const relativePath = await invoke<string>("get_relative_path", {
          baseDir,
          filePath: path as string,
        });
        onChange(relativePath);
      } catch (err) {
        console.error("Failed to get relative path:", err);
        onChange(path as string);
      }
    } else if (path) {
      onChange(path as string);
    }
  }, [accept, baseDir, onChange]);

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
