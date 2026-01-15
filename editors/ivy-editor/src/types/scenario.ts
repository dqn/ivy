// LocalizedString can be a plain string, a localized map, or a key reference
export type LocalizedString =
  | string
  | Record<string, string>;

export type CharPosition = "left" | "center" | "right";

export type TransitionType =
  | "none"
  | "fade"
  | "fade_white"
  | "dissolve"
  | "wipe"
  | "slide"
  | "pixelate"
  | "iris"
  | "blinds";

export type TransitionDirection =
  | "left_to_right"
  | "right_to_left"
  | "top_to_bottom"
  | "bottom_to_top"
  | "left"
  | "right"
  | "up"
  | "down"
  | "open"
  | "close"
  | "horizontal"
  | "vertical";

export type Easing =
  | "linear"
  | "ease_in"
  | "ease_out"
  | "ease_in_out"
  | "ease_in_quad"
  | "ease_out_quad"
  | "ease_in_out_quad"
  | "ease_in_cubic"
  | "ease_out_cubic"
  | "ease_in_out_cubic"
  | "ease_in_back"
  | "ease_out_back"
  | "ease_in_out_back"
  | "ease_out_bounce";

export interface Transition {
  type?: TransitionType;
  duration?: number;
  easing?: Easing;
  direction?: TransitionDirection;
  blinds_count?: number;
  max_pixel_size?: number;
}

export interface Choice {
  label: LocalizedString;
  jump: string;
  default?: boolean;
}

export type Value = boolean | number | string;

export interface SetVar {
  name: string;
  value: Value;
}

export interface IfCondition {
  var: string;
  is: Value;
  jump: string;
}

export interface Input {
  var: string;
  prompt?: string;
  default?: string;
}

export interface Shake {
  type?: "horizontal" | "vertical" | "both";
  intensity?: number;
  duration?: number;
  easing?: Easing;
}

export interface CameraPan {
  x?: number;
  y?: number;
}

export interface CameraCommand {
  pan?: CameraPan;
  zoom?: number;
  tilt?: number;
  focus?: string;
  duration?: number;
  easing?: Easing;
}

export interface Achievement {
  id: string;
  name: string;
  description?: string;
}

export interface AmbientTrack {
  id: string;
  path: string;
  volume?: number;
  looped?: boolean;
  fade_in?: number;
}

export interface AmbientStop {
  id: string;
  fade_out?: number;
}

export interface VideoBackground {
  path: string;
  looped?: boolean;
  on_end?: string;
}

export interface VideoCommand {
  path: string;
  skippable?: boolean;
  loop_video?: boolean;
  bgm_fade_out?: number;
  bgm_fade_in?: number;
}

export interface CharAnimation {
  type?: "none" | "fade" | "slide_left" | "slide_right";
  duration?: number;
  easing?: Easing;
}

export interface CharIdleAnimation {
  type?: "none" | "breath" | "bob" | "sway" | "pulse";
  duration?: number;
  intensity?: number;
  easing?: Easing;
}

export interface ModularCharRef {
  name: string;
  [layer: string]: string | number;
}

export interface Command {
  label?: string;
  speaker?: LocalizedString;
  text?: LocalizedString;
  choices?: Choice[];
  jump?: string;
  background?: string;
  video_bg?: VideoBackground;
  character?: string;
  char_pos?: CharPosition;
  char_enter?: CharAnimation;
  char_exit?: CharAnimation;
  char_idle?: CharIdleAnimation;
  bgm?: string;
  se?: string;
  voice?: string;
  ambient?: AmbientTrack[];
  ambient_stop?: AmbientStop[];
  transition?: Transition;
  shake?: Shake;
  set?: SetVar;
  if?: IfCondition;
  wait?: number;
  timeout?: number;
  input?: Input;
  particles?: string;
  particle_intensity?: number;
  cinematic?: boolean;
  cinematic_duration?: number;
  achievement?: Achievement;
  video?: VideoCommand;
  camera?: CameraCommand;
  nvl?: boolean;
  nvl_clear?: boolean;
  modular_char?: ModularCharRef;
}

export interface ChapterDef {
  id: string;
  title: string;
  start_label: string;
  description?: string;
}

export interface LayerDef {
  name: string;
  images: string[];
}

export interface ModularCharDef {
  base: string;
  layers?: LayerDef[];
}

export interface Scenario {
  title: string;
  chapters?: ChapterDef[];
  modular_characters?: Record<string, ModularCharDef>;
  script: Command[];
}

export type Severity = "error" | "warning";

export interface ValidationIssue {
  severity: Severity;
  message: string;
  command_index?: number;
  label?: string;
}

export interface ValidationResult {
  issues: ValidationIssue[];
}
