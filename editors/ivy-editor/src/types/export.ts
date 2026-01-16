export type ExportTarget =
  | "current_platform"
  | "windows"
  | "macos"
  | "linux"
  | "web";

export type PackageFormat = "none" | "zip" | "tar_gz" | "app_bundle";

export interface ExportOptions {
  target: ExportTarget;
  output_dir: string;
  release_build: boolean;
  optimize_assets: boolean;
  image_compression: ImageCompressionOptions | null;
  audio_conversion: AudioConversionOptions | null;
  exclude_unused_assets: boolean;
  package_format: PackageFormat;
}

export interface ImageCompressionOptions {
  format: "webp" | "jpeg" | "png";
  quality: number; // 0-100
}

export interface AudioConversionOptions {
  format: "ogg" | "mp3";
  bitrate: number; // kbps
}

export interface BuildEnvironment {
  has_rust: boolean;
  rust_version: string | null;
  has_cargo: boolean;
  has_wasm_pack: boolean;
  current_platform: ExportTarget;
}

export interface ExportProgress {
  stage: ExportStage;
  message: string;
  progress: number; // 0-100
}

export type ExportStage =
  | "checking_environment"
  | "optimizing_assets"
  | "building"
  | "packaging"
  | "completed"
  | "failed";

export interface ExportResult {
  success: boolean;
  output_path: string | null;
  error: string | null;
  warnings: string[];
}

export const EXPORT_TARGET_LABELS: Record<ExportTarget, string> = {
  current_platform: "Current Platform",
  windows: "Windows (x64)",
  macos: "macOS (Universal)",
  linux: "Linux (x64)",
  web: "Web (WASM)",
};

export const PACKAGE_FORMAT_LABELS: Record<PackageFormat, string> = {
  none: "No packaging (folder)",
  zip: "ZIP Archive",
  tar_gz: "TAR.GZ Archive",
  app_bundle: "macOS App Bundle",
};

export function getDefaultPackageFormat(target: ExportTarget): PackageFormat {
  switch (target) {
    case "windows":
      return "zip";
    case "macos":
      return "app_bundle";
    case "linux":
      return "tar_gz";
    case "web":
      return "zip";
    default:
      return "none";
  }
}

export function getDefaultExportOptions(): ExportOptions {
  return {
    target: "current_platform",
    output_dir: "",
    release_build: true,
    optimize_assets: true,
    image_compression: {
      format: "webp",
      quality: 85,
    },
    audio_conversion: {
      format: "ogg",
      bitrate: 128,
    },
    exclude_unused_assets: true,
    package_format: "none",
  };
}
