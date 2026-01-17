import { useCallback } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { invokeCommandSafe } from "../lib";

interface UseAssetPickerOptions {
  baseDir: string | null;
  accept?: string[];
  onSelect: (path: string) => void;
}

export function useAssetPicker({
  baseDir,
  accept = [],
  onSelect,
}: UseAssetPickerOptions) {
  const getRelativePath = useCallback(
    async (filePath: string): Promise<string> => {
      if (!baseDir) {return filePath;}

      const result = await invokeCommandSafe<string>("get_relative_path", {
        baseDir,
        filePath,
      });
      return result ?? filePath;
    },
    [baseDir]
  );

  const handleDrop = useCallback(
    async (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();

      const files = e.dataTransfer.files;
      if (files.length === 0) {return;}

      const file = files[0];
      const filePath =
        "path" in file && typeof file.path === "string" ? file.path : undefined;

      if (filePath) {
        const relativePath = await getRelativePath(filePath);
        onSelect(relativePath);
      }
    },
    [getRelativePath, onSelect]
  );

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = "copy";
  }, []);

  const handleBrowse = useCallback(async () => {
    const extensions = accept.map((ext) => ext.replace(".", ""));
    const path = await open({
      filters: extensions.length > 0 ? [{ name: "Assets", extensions }] : undefined,
      directory: false,
      multiple: false,
    });

    if (typeof path !== "string") {
      return;
    }

    const relativePath = await getRelativePath(path);
    onSelect(relativePath);
  }, [accept, getRelativePath, onSelect]);

  return { handleDrop, handleDragOver, handleBrowse };
}
