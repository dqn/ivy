import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  ExportOptions,
  ExportProgress,
  ExportResult,
  BuildEnvironment,
} from "../types/export";

interface UseExportReturn {
  isExporting: boolean;
  progress: ExportProgress | null;
  result: ExportResult | null;
  environment: BuildEnvironment | null;

  checkEnvironment: () => Promise<BuildEnvironment>;
  startExport: (projectPath: string, options: ExportOptions) => Promise<void>;
  cancelExport: () => Promise<void>;
  clearResult: () => void;
}

export function useExport(): UseExportReturn {
  const [isExporting, setIsExporting] = useState(false);
  const [progress, setProgress] = useState<ExportProgress | null>(null);
  const [result, setResult] = useState<ExportResult | null>(null);
  const [environment, setEnvironment] = useState<BuildEnvironment | null>(null);

  const checkEnvironment = useCallback(async () => {
    const env = await invoke<BuildEnvironment>("check_build_environment");
    setEnvironment(env);
    return env;
  }, []);

  const startExport = useCallback(
    async (projectPath: string, options: ExportOptions) => {
      setIsExporting(true);
      setProgress({
        stage: "checking_environment",
        message: "Checking build environment...",
        progress: 0,
      });
      setResult(null);

      const unlisten = await listen<ExportProgress>(
        "export-progress",
        (event) => {
          setProgress(event.payload);
        }
      );

      try {
        const exportResult = await invoke<ExportResult>("start_export", {
          projectPath,
          options,
        });
        setResult(exportResult);
      } catch (e) {
        setResult({
          success: false,
          output_path: null,
          error: String(e),
          warnings: [],
        });
      } finally {
        unlisten();
        setIsExporting(false);
        setProgress(null);
      }
    },
    []
  );

  const cancelExport = useCallback(async () => {
    try {
      await invoke("cancel_export");
    } catch (e) {
      console.error("Failed to cancel export:", e);
    }
  }, []);

  const clearResult = useCallback(() => {
    setResult(null);
  }, []);

  return {
    isExporting,
    progress,
    result,
    environment,
    checkEnvironment,
    startExport,
    cancelExport,
    clearResult,
  };
}
