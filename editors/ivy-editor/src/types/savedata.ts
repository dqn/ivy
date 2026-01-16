// Save data validation types

// Severity level for validation issues
export type IssueSeverity = "error" | "warning" | "info";

// Single validation issue
export interface ValidationIssue {
  severity: IssueSeverity;
  code: string;
  message: string;
  details: string | null;
}

// Visual state in save data
export interface SaveDataVisualState {
  background: string | null;
  character: string | null;
  char_pos: string | null;
}

// Summary information about save data
export interface SaveDataSummary {
  scenario_path: string;
  current_index: number;
  total_commands: number | null;
  timestamp: number;
  formatted_time: string;
  variable_count: number;
  visual: SaveDataVisualState;
}

// Full validation result
export interface SaveDataValidationResult {
  valid: boolean;
  file_path: string;
  summary: SaveDataSummary | null;
  issues: ValidationIssue[];
  error_count: number;
  warning_count: number;
  info_count: number;
}

// Save data list item
export interface SaveDataInfo {
  file_name: string;
  file_path: string;
  slot: number | null;
  timestamp: number;
  formatted_time: string;
  scenario_path: string | null;
  size_bytes: number;
}
