export type PlaytestAction =
  | "advance"
  | "advance_or_select_first"
  | "rollback"
  | "restart"
  | "toggle_auto"
  | "toggle_skip"
  | "save"
  | "load"
  | "select_choice";

interface KeyBinding {
  key: string;
  modifiers?: ("ctrl" | "meta" | "shift" | "alt")[];
  requireNoModifiers?: boolean;
  action: PlaytestAction;
}

export const PLAYTEST_KEYBINDINGS: KeyBinding[] = [
  { key: "Enter", action: "advance_or_select_first" },
  { key: " ", action: "advance" },
  { key: "Backspace", action: "rollback" },
  { key: "ArrowUp", action: "rollback" },
  { key: "ArrowDown", action: "advance" },
  { key: "ArrowRight", action: "advance" },
  { key: "r", modifiers: ["ctrl"], action: "restart" },
  { key: "r", modifiers: ["meta"], action: "restart" },
  { key: "a", action: "toggle_auto" },
  { key: "s", requireNoModifiers: true, action: "toggle_skip" },
  { key: "F5", action: "save" },
  { key: "F9", action: "load" },
];

export function matchesBinding(
  event: KeyboardEvent,
  binding: KeyBinding
): boolean {
  if (event.key.toLowerCase() !== binding.key.toLowerCase()) {return false;}

  const modifiers = binding.modifiers ?? [];
  const hasCtrl = modifiers.includes("ctrl");
  const hasMeta = modifiers.includes("meta");
  const hasShift = modifiers.includes("shift");
  const hasAlt = modifiers.includes("alt");

  if (binding.requireNoModifiers) {
    if (event.ctrlKey || event.metaKey) {return false;}
  } else {
    if (hasCtrl !== event.ctrlKey) {return false;}
    if (hasMeta !== event.metaKey) {return false;}
    if (hasShift !== event.shiftKey) {return false;}
    if (hasAlt !== event.altKey) {return false;}
  }

  return true;
}

export function getActionForKey(event: KeyboardEvent): PlaytestAction | null {
  for (const binding of PLAYTEST_KEYBINDINGS) {
    if (matchesBinding(event, binding)) {
      return binding.action;
    }
  }

  // Handle number keys for choice selection
  if (/^[1-9]$/.test(event.key)) {
    return "select_choice";
  }

  return null;
}
