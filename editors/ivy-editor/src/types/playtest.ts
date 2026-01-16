// Variable value types
export type Value = boolean | number | string;

// Choice in playtest display
export interface PlaytestChoice {
  label: string;
  jump: string;
}

// Base visual state fields shared across display types
interface BaseVisualState {
  background: string | null;
  character: string | null;
  char_pos: string | null;
}

// Text display state
export interface TextDisplay extends BaseVisualState {
  type: "text";
  speaker: string | null;
  text: string;
  nvl_mode: boolean;
}

// Choices display state
export interface ChoicesDisplay extends BaseVisualState {
  type: "choices";
  speaker: string | null;
  text: string;
  choices: PlaytestChoice[];
  timeout: number | null;
  default_choice: number | null;
}

// Input display state
export interface InputDisplay extends BaseVisualState {
  type: "input";
  prompt: string;
  var_name: string;
  default_value: string | null;
}

// Wait display state
export interface WaitDisplay extends BaseVisualState {
  type: "wait";
  duration: number;
}

// Video display state
export interface VideoDisplay {
  type: "video";
  path: string;
  skippable: boolean;
  loop_video: boolean;
}

// End display state
export interface EndDisplay {
  type: "end";
}

// Union of all display states
export type PlaytestDisplay =
  | TextDisplay
  | ChoicesDisplay
  | InputDisplay
  | WaitDisplay
  | VideoDisplay
  | EndDisplay;

// History entry for backlog display
export interface PlaytestHistoryEntry {
  index: number;
  speaker: string | null;
  text: string;
}

// Transition effect info
export interface PlaytestTransition {
  type: string;
  duration: number;
  direction: string;
}

// Full playtest state returned from backend
export interface PlaytestState {
  active: boolean;
  command_index: number;
  total_commands: number;
  display: PlaytestDisplay;
  variables: Record<string, Value>;
  history_count: number;
  can_rollback: boolean;
  is_ended: boolean;
  labels: string[];
  current_label: string | null;
  history: PlaytestHistoryEntry[];
  transition: PlaytestTransition | null;
}
