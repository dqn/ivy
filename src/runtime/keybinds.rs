use macroquad::prelude::KeyCode;
use serde::{Deserialize, Serialize};

use crate::input::{GamepadBindings, GamepadButton};

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

/// Modifier keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modifier {
    Shift,
    Ctrl,
    Alt,
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
        let key_name = self.key.0.name();
        match self.modifier {
            Some(Modifier::Shift) => format!("Shift+{}", key_name),
            Some(Modifier::Ctrl) => format!("Ctrl+{}", key_name),
            Some(Modifier::Alt) => format!("Alt+{}", key_name),
            None => key_name.to_string(),
        }
    }
}

/// Key code to name mapping.
const KEY_MAPPING: &[(KeyCode, &str)] = &[
    (KeyCode::Space, "Space"),
    (KeyCode::Apostrophe, "'"),
    (KeyCode::Comma, ","),
    (KeyCode::Minus, "-"),
    (KeyCode::Period, "."),
    (KeyCode::Slash, "/"),
    (KeyCode::Key0, "0"),
    (KeyCode::Key1, "1"),
    (KeyCode::Key2, "2"),
    (KeyCode::Key3, "3"),
    (KeyCode::Key4, "4"),
    (KeyCode::Key5, "5"),
    (KeyCode::Key6, "6"),
    (KeyCode::Key7, "7"),
    (KeyCode::Key8, "8"),
    (KeyCode::Key9, "9"),
    (KeyCode::Semicolon, ";"),
    (KeyCode::Equal, "="),
    (KeyCode::A, "A"),
    (KeyCode::B, "B"),
    (KeyCode::C, "C"),
    (KeyCode::D, "D"),
    (KeyCode::E, "E"),
    (KeyCode::F, "F"),
    (KeyCode::G, "G"),
    (KeyCode::H, "H"),
    (KeyCode::I, "I"),
    (KeyCode::J, "J"),
    (KeyCode::K, "K"),
    (KeyCode::L, "L"),
    (KeyCode::M, "M"),
    (KeyCode::N, "N"),
    (KeyCode::O, "O"),
    (KeyCode::P, "P"),
    (KeyCode::Q, "Q"),
    (KeyCode::R, "R"),
    (KeyCode::S, "S"),
    (KeyCode::T, "T"),
    (KeyCode::U, "U"),
    (KeyCode::V, "V"),
    (KeyCode::W, "W"),
    (KeyCode::X, "X"),
    (KeyCode::Y, "Y"),
    (KeyCode::Z, "Z"),
    (KeyCode::LeftBracket, "["),
    (KeyCode::Backslash, "\\"),
    (KeyCode::RightBracket, "]"),
    (KeyCode::GraveAccent, "`"),
    (KeyCode::Escape, "Esc"),
    (KeyCode::Enter, "Enter"),
    (KeyCode::Tab, "Tab"),
    (KeyCode::Backspace, "Backspace"),
    (KeyCode::Insert, "Insert"),
    (KeyCode::Delete, "Delete"),
    (KeyCode::Right, "Right"),
    (KeyCode::Left, "Left"),
    (KeyCode::Down, "Down"),
    (KeyCode::Up, "Up"),
    (KeyCode::PageUp, "PageUp"),
    (KeyCode::PageDown, "PageDown"),
    (KeyCode::Home, "Home"),
    (KeyCode::End, "End"),
    (KeyCode::CapsLock, "CapsLock"),
    (KeyCode::ScrollLock, "ScrollLock"),
    (KeyCode::NumLock, "NumLock"),
    (KeyCode::PrintScreen, "PrintScreen"),
    (KeyCode::Pause, "Pause"),
    (KeyCode::F1, "F1"),
    (KeyCode::F2, "F2"),
    (KeyCode::F3, "F3"),
    (KeyCode::F4, "F4"),
    (KeyCode::F5, "F5"),
    (KeyCode::F6, "F6"),
    (KeyCode::F7, "F7"),
    (KeyCode::F8, "F8"),
    (KeyCode::F9, "F9"),
    (KeyCode::F10, "F10"),
    (KeyCode::F11, "F11"),
    (KeyCode::F12, "F12"),
    (KeyCode::F13, "F13"),
    (KeyCode::F14, "F14"),
    (KeyCode::F15, "F15"),
    (KeyCode::F16, "F16"),
    (KeyCode::F17, "F17"),
    (KeyCode::F18, "F18"),
    (KeyCode::F19, "F19"),
    (KeyCode::F20, "F20"),
    (KeyCode::F21, "F21"),
    (KeyCode::F22, "F22"),
    (KeyCode::F23, "F23"),
    (KeyCode::F24, "F24"),
    (KeyCode::F25, "F25"),
    (KeyCode::Kp0, "Num0"),
    (KeyCode::Kp1, "Num1"),
    (KeyCode::Kp2, "Num2"),
    (KeyCode::Kp3, "Num3"),
    (KeyCode::Kp4, "Num4"),
    (KeyCode::Kp5, "Num5"),
    (KeyCode::Kp6, "Num6"),
    (KeyCode::Kp7, "Num7"),
    (KeyCode::Kp8, "Num8"),
    (KeyCode::Kp9, "Num9"),
    (KeyCode::KpDecimal, "Num."),
    (KeyCode::KpDivide, "Num/"),
    (KeyCode::KpMultiply, "Num*"),
    (KeyCode::KpSubtract, "Num-"),
    (KeyCode::KpAdd, "Num+"),
    (KeyCode::KpEnter, "NumEnter"),
    (KeyCode::KpEqual, "Num="),
    (KeyCode::LeftShift, "LShift"),
    (KeyCode::LeftControl, "LCtrl"),
    (KeyCode::LeftAlt, "LAlt"),
    (KeyCode::LeftSuper, "LSuper"),
    (KeyCode::RightShift, "RShift"),
    (KeyCode::RightControl, "RCtrl"),
    (KeyCode::RightAlt, "RAlt"),
    (KeyCode::RightSuper, "RSuper"),
    (KeyCode::Menu, "Menu"),
    (KeyCode::Unknown, "Unknown"),
    (KeyCode::World1, "World1"),
    (KeyCode::World2, "World2"),
    (KeyCode::Back, "Back"),
];

/// Extension trait for KeyCode to get its name.
trait KeyCodeExt {
    fn name(&self) -> &'static str;
    fn from_name(name: &str) -> Option<KeyCode>;
}

impl KeyCodeExt for KeyCode {
    fn name(&self) -> &'static str {
        KEY_MAPPING
            .iter()
            .find(|(k, _)| k == self)
            .map(|(_, n)| *n)
            .unwrap_or("Unknown")
    }

    fn from_name(name: &str) -> Option<KeyCode> {
        KEY_MAPPING
            .iter()
            .find(|(_, n)| *n == name)
            .map(|(k, _)| *k)
    }
}

/// Wrapper for KeyCode to enable serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SerializableKeyCode(pub KeyCode);

impl Serialize for SerializableKeyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.name())
    }
}

impl<'de> Deserialize<'de> for SerializableKeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        KeyCode::from_name(&s)
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
    /// Gamepad bindings.
    #[serde(default)]
    pub gamepad: GamepadBindings,
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
            gamepad: GamepadBindings::default(),
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

    /// Get gamepad button for an action.
    pub fn get_gamepad(&self, action: Action) -> Option<GamepadButton> {
        match action {
            Action::Advance => Some(self.gamepad.advance),
            Action::Rollback => Some(self.gamepad.rollback),
            Action::AutoMode => Some(self.gamepad.auto_mode),
            Action::SkipMode => Some(self.gamepad.skip_mode),
            Action::Backlog => Some(self.gamepad.backlog),
            Action::Settings => Some(self.gamepad.menu),
            // No gamepad bindings for QuickSave, QuickLoad, Debug
            Action::QuickSave | Action::QuickLoad | Action::Debug => None,
        }
    }

    /// Check if an action is triggered (using macroquad directly).
    /// Checks both keyboard and gamepad inputs.
    pub fn is_pressed(&self, action: Action) -> bool {
        self.is_keyboard_pressed(action) || self.is_gamepad_pressed(action)
    }

    /// Check if an action is triggered via keyboard.
    fn is_keyboard_pressed(&self, action: Action) -> bool {
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

    /// Check if an action is triggered via gamepad.
    /// Note: Gamepad support is not yet available in macroquad 0.4.
    /// TODO: Add `gamepads` crate for gamepad support.
    fn is_gamepad_pressed(&self, _action: Action) -> bool {
        // Stub implementation - gamepad support pending
        false
    }

    /// Check if an action is triggered (using InputProvider).
    pub fn is_pressed_with<I: crate::input::InputProvider>(
        &self,
        action: Action,
        input: &I,
    ) -> bool {
        let binding = self.get(action);

        // Check modifier
        let modifier_ok = match binding.modifier {
            Some(Modifier::Shift) => {
                input.is_key_down(KeyCode::LeftShift) || input.is_key_down(KeyCode::RightShift)
            }
            Some(Modifier::Ctrl) => {
                input.is_key_down(KeyCode::LeftControl) || input.is_key_down(KeyCode::RightControl)
            }
            Some(Modifier::Alt) => {
                input.is_key_down(KeyCode::LeftAlt) || input.is_key_down(KeyCode::RightAlt)
            }
            None => true,
        };

        modifier_ok && input.is_key_pressed(binding.key.0)
    }
}
