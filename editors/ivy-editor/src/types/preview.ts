export interface ChoiceInfo {
  label: string;
  jump: string;
}

export interface PreviewState {
  title: string;
  command_index: number;
  total_commands: number;
  text: string | null;
  speaker: string | null;
  background: string | null;
  character: string | null;
  char_pos: string | null;
  choices: ChoiceInfo[];
  variables: Record<string, string>;
  labels: string[];
  current_label: string | null;
  nvl_mode: boolean;
}
