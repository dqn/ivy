import { invoke } from "@tauri-apps/api/core";

type ShowToast = (
  message: string,
  type?: "error" | "success" | "info" | "warning"
) => void;

interface InvokeOptions {
  showToast?: ShowToast;
  silent?: boolean;
}

export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
  options?: InvokeOptions
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (e) {
    if (!options?.silent) {
      console.error(`Failed to invoke ${command}:`, e);
    }
    options?.showToast?.(`Failed: ${e}`, "error");
    throw e;
  }
}

export async function invokeCommandSafe<T>(
  command: string,
  args?: Record<string, unknown>,
  options?: InvokeOptions
): Promise<T | null> {
  try {
    return await invoke<T>(command, args);
  } catch (e) {
    if (!options?.silent) {
      console.error(`Failed to invoke ${command}:`, e);
    }
    options?.showToast?.(`Failed: ${e}`, "error");
    return null;
  }
}
