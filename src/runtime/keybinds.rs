use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};

/// Actions that can be bound to keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Advance,
    Rollback,
    AutoMode,
    SkipMode,
    Backlog,
    QuickSave,
    QuickLoad,
    Settings,
    Debug,
}

impl Action {
    /// Get all actions.
    pub fn all() -> &'static [Action] {
        &[
            Action::Advance,
            Action::Rollback,
            Action::AutoMode,
            Action::SkipMode,
            Action::Backlog,
            Action::QuickSave,
            Action::QuickLoad,
            Action::Settings,
            Action::Debug,
        ]
    }

    /// Get display name for the action.
    pub fn display_name(&self) -> &'static str {
        match self {
            Action::Advance => "Advance",
            Action::Rollback => "Rollback",
            Action::AutoMode => "Auto Mode",
            Action::SkipMode => "Skip Mode",
            Action::Backlog => "Backlog",
            Action::QuickSave => "Quick Save",
            Action::QuickLoad => "Quick Load",
            Action::Settings => "Settings",
            Action::Debug => "Debug",
        }
    }
}

/// Key code wrapper for serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyBinding {
    /// Primary key.
    pub key: SerializableKeyCode,
    /// Optional modifier (Shift, Ctrl, Alt).
    pub modifier: Option<Modifier>,
}

impl KeyBinding {
    pub fn new(key: KeyCode) -> Self {
        Self {
            key: SerializableKeyCode(key),
            modifier: None,
        }
    }

    pub fn with_modifier(key: KeyCode, modifier: Modifier) -> Self {
        Self {
            key: SerializableKeyCode(key),
            modifier: Some(modifier),
        }
    }

    /// Get display string for the key binding.
    pub fn display(&self) -> String {
        let key_name = key_name(self.key.0);
        match self.modifier {
            Some(Modifier::Shift) => format!("Shift+{}", key_name),
            Some(Modifier::Ctrl) => format!("Ctrl+{}", key_name),
            Some(Modifier::Alt) => format!("Alt+{}", key_name),
            None => key_name.to_string(),
        }
    }
}

/// Modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modifier {
    Shift,
    Ctrl,
    Alt,
}

/// Wrapper for KeyCode to enable serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializableKeyCode(pub KeyCode);

impl Serialize for SerializableKeyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(key_name(self.0))
    }
}

impl<'de> Deserialize<'de> for SerializableKeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        key_from_name(&s)
            .map(SerializableKeyCode)
            .ok_or_else(|| serde::de::Error::custom(format!("Unknown key: {}", s)))
    }
}

/// Key bindings configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    pub advance: KeyBinding,
    pub rollback: KeyBinding,
    pub auto_mode: KeyBinding,
    pub skip_mode: KeyBinding,
    pub backlog: KeyBinding,
    pub quick_save: KeyBinding,
    pub quick_load: KeyBinding,
    pub settings: KeyBinding,
    pub debug: KeyBinding,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            advance: KeyBinding::new(KeyCode::Enter),
            rollback: KeyBinding::new(KeyCode::Up),
            auto_mode: KeyBinding::new(KeyCode::A),
            skip_mode: KeyBinding::new(KeyCode::S),
            backlog: KeyBinding::new(KeyCode::L),
            quick_save: KeyBinding::new(KeyCode::F5),
            quick_load: KeyBinding::new(KeyCode::F9),
            settings: KeyBinding::new(KeyCode::Escape),
            debug: KeyBinding::new(KeyCode::F12),
        }
    }
}

impl KeyBindings {
    /// Get binding for an action.
    pub fn get(&self, action: Action) -> &KeyBinding {
        match action {
            Action::Advance => &self.advance,
            Action::Rollback => &self.rollback,
            Action::AutoMode => &self.auto_mode,
            Action::SkipMode => &self.skip_mode,
            Action::Backlog => &self.backlog,
            Action::QuickSave => &self.quick_save,
            Action::QuickLoad => &self.quick_load,
            Action::Settings => &self.settings,
            Action::Debug => &self.debug,
        }
    }

    /// Set binding for an action.
    pub fn set(&mut self, action: Action, binding: KeyBinding) {
        match action {
            Action::Advance => self.advance = binding,
            Action::Rollback => self.rollback = binding,
            Action::AutoMode => self.auto_mode = binding,
            Action::SkipMode => self.skip_mode = binding,
            Action::Backlog => self.backlog = binding,
            Action::QuickSave => self.quick_save = binding,
            Action::QuickLoad => self.quick_load = binding,
            Action::Settings => self.settings = binding,
            Action::Debug => self.debug = binding,
        }
    }

    /// Check if an action is triggered.
    pub fn is_pressed(&self, action: Action) -> bool {
        use macroquad::prelude::*;

        let binding = self.get(action);

        // Check modifier
        let modifier_ok = match binding.modifier {
            Some(Modifier::Shift) => {
                is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)
            }
            Some(Modifier::Ctrl) => {
                is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
            }
            Some(Modifier::Alt) => is_key_down(KeyCode::LeftAlt) || is_key_down(KeyCode::RightAlt),
            None => true,
        };

        modifier_ok && is_key_pressed(binding.key.0)
    }
}

/// Get the name of a key.
fn key_name(key: KeyCode) -> &'static str {
    match key {
        KeyCode::Space => "Space",
        KeyCode::Apostrophe => "'",
        KeyCode::Comma => ",",
        KeyCode::Minus => "-",
        KeyCode::Period => ".",
        KeyCode::Slash => "/",
        KeyCode::Key0 => "0",
        KeyCode::Key1 => "1",
        KeyCode::Key2 => "2",
        KeyCode::Key3 => "3",
        KeyCode::Key4 => "4",
        KeyCode::Key5 => "5",
        KeyCode::Key6 => "6",
        KeyCode::Key7 => "7",
        KeyCode::Key8 => "8",
        KeyCode::Key9 => "9",
        KeyCode::Semicolon => ";",
        KeyCode::Equal => "=",
        KeyCode::A => "A",
        KeyCode::B => "B",
        KeyCode::C => "C",
        KeyCode::D => "D",
        KeyCode::E => "E",
        KeyCode::F => "F",
        KeyCode::G => "G",
        KeyCode::H => "H",
        KeyCode::I => "I",
        KeyCode::J => "J",
        KeyCode::K => "K",
        KeyCode::L => "L",
        KeyCode::M => "M",
        KeyCode::N => "N",
        KeyCode::O => "O",
        KeyCode::P => "P",
        KeyCode::Q => "Q",
        KeyCode::R => "R",
        KeyCode::S => "S",
        KeyCode::T => "T",
        KeyCode::U => "U",
        KeyCode::V => "V",
        KeyCode::W => "W",
        KeyCode::X => "X",
        KeyCode::Y => "Y",
        KeyCode::Z => "Z",
        KeyCode::LeftBracket => "[",
        KeyCode::Backslash => "\\",
        KeyCode::RightBracket => "]",
        KeyCode::GraveAccent => "`",
        KeyCode::Escape => "Esc",
        KeyCode::Enter => "Enter",
        KeyCode::Tab => "Tab",
        KeyCode::Backspace => "Backspace",
        KeyCode::Insert => "Insert",
        KeyCode::Delete => "Delete",
        KeyCode::Right => "Right",
        KeyCode::Left => "Left",
        KeyCode::Down => "Down",
        KeyCode::Up => "Up",
        KeyCode::PageUp => "PageUp",
        KeyCode::PageDown => "PageDown",
        KeyCode::Home => "Home",
        KeyCode::End => "End",
        KeyCode::CapsLock => "CapsLock",
        KeyCode::ScrollLock => "ScrollLock",
        KeyCode::NumLock => "NumLock",
        KeyCode::PrintScreen => "PrintScreen",
        KeyCode::Pause => "Pause",
        KeyCode::F1 => "F1",
        KeyCode::F2 => "F2",
        KeyCode::F3 => "F3",
        KeyCode::F4 => "F4",
        KeyCode::F5 => "F5",
        KeyCode::F6 => "F6",
        KeyCode::F7 => "F7",
        KeyCode::F8 => "F8",
        KeyCode::F9 => "F9",
        KeyCode::F10 => "F10",
        KeyCode::F11 => "F11",
        KeyCode::F12 => "F12",
        KeyCode::F13 => "F13",
        KeyCode::F14 => "F14",
        KeyCode::F15 => "F15",
        KeyCode::F16 => "F16",
        KeyCode::F17 => "F17",
        KeyCode::F18 => "F18",
        KeyCode::F19 => "F19",
        KeyCode::F20 => "F20",
        KeyCode::F21 => "F21",
        KeyCode::F22 => "F22",
        KeyCode::F23 => "F23",
        KeyCode::F24 => "F24",
        KeyCode::F25 => "F25",
        KeyCode::Kp0 => "Num0",
        KeyCode::Kp1 => "Num1",
        KeyCode::Kp2 => "Num2",
        KeyCode::Kp3 => "Num3",
        KeyCode::Kp4 => "Num4",
        KeyCode::Kp5 => "Num5",
        KeyCode::Kp6 => "Num6",
        KeyCode::Kp7 => "Num7",
        KeyCode::Kp8 => "Num8",
        KeyCode::Kp9 => "Num9",
        KeyCode::KpDecimal => "Num.",
        KeyCode::KpDivide => "Num/",
        KeyCode::KpMultiply => "Num*",
        KeyCode::KpSubtract => "Num-",
        KeyCode::KpAdd => "Num+",
        KeyCode::KpEnter => "NumEnter",
        KeyCode::KpEqual => "Num=",
        KeyCode::LeftShift => "LShift",
        KeyCode::LeftControl => "LCtrl",
        KeyCode::LeftAlt => "LAlt",
        KeyCode::LeftSuper => "LSuper",
        KeyCode::RightShift => "RShift",
        KeyCode::RightControl => "RCtrl",
        KeyCode::RightAlt => "RAlt",
        KeyCode::RightSuper => "RSuper",
        KeyCode::Menu => "Menu",
        KeyCode::Unknown => "Unknown",
        KeyCode::World1 => "World1",
        KeyCode::World2 => "World2",
        KeyCode::Back => "Back",
    }
}

/// Get key code from name.
fn key_from_name(name: &str) -> Option<KeyCode> {
    match name {
        "Space" => Some(KeyCode::Space),
        "'" => Some(KeyCode::Apostrophe),
        "," => Some(KeyCode::Comma),
        "-" => Some(KeyCode::Minus),
        "." => Some(KeyCode::Period),
        "/" => Some(KeyCode::Slash),
        "0" => Some(KeyCode::Key0),
        "1" => Some(KeyCode::Key1),
        "2" => Some(KeyCode::Key2),
        "3" => Some(KeyCode::Key3),
        "4" => Some(KeyCode::Key4),
        "5" => Some(KeyCode::Key5),
        "6" => Some(KeyCode::Key6),
        "7" => Some(KeyCode::Key7),
        "8" => Some(KeyCode::Key8),
        "9" => Some(KeyCode::Key9),
        ";" => Some(KeyCode::Semicolon),
        "=" => Some(KeyCode::Equal),
        "A" => Some(KeyCode::A),
        "B" => Some(KeyCode::B),
        "C" => Some(KeyCode::C),
        "D" => Some(KeyCode::D),
        "E" => Some(KeyCode::E),
        "F" => Some(KeyCode::F),
        "G" => Some(KeyCode::G),
        "H" => Some(KeyCode::H),
        "I" => Some(KeyCode::I),
        "J" => Some(KeyCode::J),
        "K" => Some(KeyCode::K),
        "L" => Some(KeyCode::L),
        "M" => Some(KeyCode::M),
        "N" => Some(KeyCode::N),
        "O" => Some(KeyCode::O),
        "P" => Some(KeyCode::P),
        "Q" => Some(KeyCode::Q),
        "R" => Some(KeyCode::R),
        "S" => Some(KeyCode::S),
        "T" => Some(KeyCode::T),
        "U" => Some(KeyCode::U),
        "V" => Some(KeyCode::V),
        "W" => Some(KeyCode::W),
        "X" => Some(KeyCode::X),
        "Y" => Some(KeyCode::Y),
        "Z" => Some(KeyCode::Z),
        "[" => Some(KeyCode::LeftBracket),
        "\\" => Some(KeyCode::Backslash),
        "]" => Some(KeyCode::RightBracket),
        "`" => Some(KeyCode::GraveAccent),
        "Esc" => Some(KeyCode::Escape),
        "Enter" => Some(KeyCode::Enter),
        "Tab" => Some(KeyCode::Tab),
        "Backspace" => Some(KeyCode::Backspace),
        "Insert" => Some(KeyCode::Insert),
        "Delete" => Some(KeyCode::Delete),
        "Right" => Some(KeyCode::Right),
        "Left" => Some(KeyCode::Left),
        "Down" => Some(KeyCode::Down),
        "Up" => Some(KeyCode::Up),
        "PageUp" => Some(KeyCode::PageUp),
        "PageDown" => Some(KeyCode::PageDown),
        "Home" => Some(KeyCode::Home),
        "End" => Some(KeyCode::End),
        "CapsLock" => Some(KeyCode::CapsLock),
        "ScrollLock" => Some(KeyCode::ScrollLock),
        "NumLock" => Some(KeyCode::NumLock),
        "PrintScreen" => Some(KeyCode::PrintScreen),
        "Pause" => Some(KeyCode::Pause),
        "F1" => Some(KeyCode::F1),
        "F2" => Some(KeyCode::F2),
        "F3" => Some(KeyCode::F3),
        "F4" => Some(KeyCode::F4),
        "F5" => Some(KeyCode::F5),
        "F6" => Some(KeyCode::F6),
        "F7" => Some(KeyCode::F7),
        "F8" => Some(KeyCode::F8),
        "F9" => Some(KeyCode::F9),
        "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11),
        "F12" => Some(KeyCode::F12),
        "F13" => Some(KeyCode::F13),
        "F14" => Some(KeyCode::F14),
        "F15" => Some(KeyCode::F15),
        "F16" => Some(KeyCode::F16),
        "F17" => Some(KeyCode::F17),
        "F18" => Some(KeyCode::F18),
        "F19" => Some(KeyCode::F19),
        "F20" => Some(KeyCode::F20),
        "F21" => Some(KeyCode::F21),
        "F22" => Some(KeyCode::F22),
        "F23" => Some(KeyCode::F23),
        "F24" => Some(KeyCode::F24),
        "F25" => Some(KeyCode::F25),
        "Num0" => Some(KeyCode::Kp0),
        "Num1" => Some(KeyCode::Kp1),
        "Num2" => Some(KeyCode::Kp2),
        "Num3" => Some(KeyCode::Kp3),
        "Num4" => Some(KeyCode::Kp4),
        "Num5" => Some(KeyCode::Kp5),
        "Num6" => Some(KeyCode::Kp6),
        "Num7" => Some(KeyCode::Kp7),
        "Num8" => Some(KeyCode::Kp8),
        "Num9" => Some(KeyCode::Kp9),
        "Num." => Some(KeyCode::KpDecimal),
        "Num/" => Some(KeyCode::KpDivide),
        "Num*" => Some(KeyCode::KpMultiply),
        "Num-" => Some(KeyCode::KpSubtract),
        "Num+" => Some(KeyCode::KpAdd),
        "NumEnter" => Some(KeyCode::KpEnter),
        "Num=" => Some(KeyCode::KpEqual),
        "LShift" => Some(KeyCode::LeftShift),
        "LCtrl" => Some(KeyCode::LeftControl),
        "LAlt" => Some(KeyCode::LeftAlt),
        "LSuper" => Some(KeyCode::LeftSuper),
        "RShift" => Some(KeyCode::RightShift),
        "RCtrl" => Some(KeyCode::RightControl),
        "RAlt" => Some(KeyCode::RightAlt),
        "RSuper" => Some(KeyCode::RightSuper),
        "Menu" => Some(KeyCode::Menu),
        "Back" => Some(KeyCode::Back),
        _ => None,
    }
}
