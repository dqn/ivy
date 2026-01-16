import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type {
  SaveDataInfo,
  SaveDataValidationResult,
} from "../types/savedata";

interface UseSaveDataValidatorReturn {
  // State
  saveDataList: SaveDataInfo[];
  selectedSaveData: SaveDataInfo | null;
  validationResult: SaveDataValidationResult | null;
  isLoading: boolean;
  error: string | null;

  // Actions
  loadSaveDataList: (baseDir: string) => Promise<void>;
  selectSaveData: (saveData: SaveDataInfo) => Promise<void>;
  validateSaveData: (savePath: string, baseDir: string | null) => Promise<void>;
  openSaveFile: (baseDir: string | null) => Promise<void>;
  clearSelection: () => void;
}

export function useSaveDataValidator(): UseSaveDataValidatorReturn {
  const [saveDataList, setSaveDataList] = useState<SaveDataInfo[]>([]);
  const [selectedSaveData, setSelectedSaveData] = useState<SaveDataInfo | null>(
    null
  );
  const [validationResult, setValidationResult] =
    useState<SaveDataValidationResult | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadSaveDataList = useCallback(async (baseDir: string) => {
    setIsLoading(true);
    setError(null);

    try {
      const list = await invoke<SaveDataInfo[]>("list_save_data", { baseDir });
      setSaveDataList(list);
    } catch (e) {
      setError(e instanceof Error ? e.message : String(e));
      setSaveDataList([]);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const validateSaveData = useCallback(
    async (savePath: string, baseDir: string | null) => {
      setIsLoading(true);
      setError(null);

      try {
        const result = await invoke<SaveDataValidationResult>(
          "validate_save_data",
          { savePath, baseDir }
        );
        setValidationResult(result);
      } catch (e) {
        setError(e instanceof Error ? e.message : String(e));
        setValidationResult(null);
      } finally {
        setIsLoading(false);
      }
    },
    []
  );

  const selectSaveData = useCallback(
    async (saveData: SaveDataInfo) => {
      setSelectedSaveData(saveData);

      // Extract base directory from file path
      const baseDir = saveData.file_path.substring(
        0,
        saveData.file_path.lastIndexOf("/saves/")
      );

      await validateSaveData(saveData.file_path, baseDir || null);
    },
    [validateSaveData]
  );

  const openSaveFile = useCallback(
    async (baseDir: string | null) => {
      try {
        const result = await open({
          title: "Select Save Data File",
          filters: [{ name: "JSON Files", extensions: ["json"] }],
          defaultPath: baseDir ? `${baseDir}/saves` : undefined,
        });

        if (result) {
          const filePath = result as string;
          const fileName = filePath.split("/").pop() || filePath;

          // Create a SaveDataInfo for the selected file
          const saveData: SaveDataInfo = {
            file_name: fileName,
            file_path: filePath,
            slot: null,
            timestamp: 0,
            formatted_time: "Unknown",
            scenario_path: null,
            size_bytes: 0,
          };

          setSelectedSaveData(saveData);
          await validateSaveData(filePath, baseDir);
        }
      } catch (e) {
        setError(e instanceof Error ? e.message : String(e));
      }
    },
    [validateSaveData]
  );

  const clearSelection = useCallback(() => {
    setSelectedSaveData(null);
    setValidationResult(null);
    setError(null);
  }, []);

  return {
    saveDataList,
    selectedSaveData,
    validationResult,
    isLoading,
    error,
    loadSaveDataList,
    selectSaveData,
    validateSaveData,
    openSaveFile,
    clearSelection,
  };
}
