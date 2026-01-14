//! Gamepad input handling.

use serde::{Deserialize, Serialize};

/// Gamepad button mapping for visual novel controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GamepadButton {
    /// A button (typically bottom button) - Advance/Confirm.
    A,
    /// B button (typically right button) - Cancel/Back.
    B,
    /// X button (typically left button) - Auto mode.
    X,
    /// Y button (typically top button) - Skip mode.
    Y,
    /// Start button - Menu.
    Start,
    /// Select/Back button - Backlog.
    Select,
    /// Left bumper - Rollback.
    LB,
    /// Right bumper - Next page.
    RB,
    /// D-Pad up.
    DPadUp,
    /// D-Pad down.
    DPadDown,
    /// D-Pad left.
    DPadLeft,
    /// D-Pad right.
    DPadRight,
}

/// Gamepad axis for analog input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    /// Left stick X axis.
    LeftX,
    /// Left stick Y axis.
    LeftY,
    /// Right stick X axis.
    RightX,
    /// Right stick Y axis.
    RightY,
}

/// Default gamepad bindings for visual novel controls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GamepadBindings {
    /// Advance text / confirm selection.
    pub advance: GamepadButton,
    /// Cancel / back.
    pub cancel: GamepadButton,
    /// Toggle auto mode.
    pub auto_mode: GamepadButton,
    /// Toggle skip mode.
    pub skip_mode: GamepadButton,
    /// Open backlog.
    pub backlog: GamepadButton,
    /// Rollback.
    pub rollback: GamepadButton,
    /// Open menu.
    pub menu: GamepadButton,
}

impl Default for GamepadBindings {
    fn default() -> Self {
        Self {
            advance: GamepadButton::A,
            cancel: GamepadButton::B,
            auto_mode: GamepadButton::X,
            skip_mode: GamepadButton::Y,
            backlog: GamepadButton::Select,
            rollback: GamepadButton::LB,
            menu: GamepadButton::Start,
        }
    }
}

/// Threshold for analog stick to register as direction.
pub const STICK_THRESHOLD: f32 = 0.5;

/// Convert macroquad gamepad button index to GamepadButton.
#[cfg(not(target_arch = "wasm32"))]
pub fn from_macroquad_button(button: usize) -> Option<GamepadButton> {
    // Standard gamepad mapping (Xbox-style)
    match button {
        0 => Some(GamepadButton::A),          // A / Cross
        1 => Some(GamepadButton::B),          // B / Circle
        2 => Some(GamepadButton::X),          // X / Square
        3 => Some(GamepadButton::Y),          // Y / Triangle
        4 => Some(GamepadButton::LB),         // Left Bumper
        5 => Some(GamepadButton::RB),         // Right Bumper
        8 => Some(GamepadButton::Select),     // Select / Back
        9 => Some(GamepadButton::Start),      // Start
        12 => Some(GamepadButton::DPadUp),    // D-Pad Up
        13 => Some(GamepadButton::DPadDown),  // D-Pad Down
        14 => Some(GamepadButton::DPadLeft),  // D-Pad Left
        15 => Some(GamepadButton::DPadRight), // D-Pad Right
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn from_macroquad_button(_button: usize) -> Option<GamepadButton> {
    None
}

/// Convert GamepadButton to macroquad button index.
#[cfg(not(target_arch = "wasm32"))]
pub fn to_macroquad_button(button: GamepadButton) -> usize {
    match button {
        GamepadButton::A => 0,
        GamepadButton::B => 1,
        GamepadButton::X => 2,
        GamepadButton::Y => 3,
        GamepadButton::LB => 4,
        GamepadButton::RB => 5,
        GamepadButton::Select => 8,
        GamepadButton::Start => 9,
        GamepadButton::DPadUp => 12,
        GamepadButton::DPadDown => 13,
        GamepadButton::DPadLeft => 14,
        GamepadButton::DPadRight => 15,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn to_macroquad_button(_button: GamepadButton) -> usize {
    0
}

/// Convert GamepadAxis to macroquad axis index.
#[cfg(not(target_arch = "wasm32"))]
pub fn to_macroquad_axis(axis: GamepadAxis) -> usize {
    match axis {
        GamepadAxis::LeftX => 0,
        GamepadAxis::LeftY => 1,
        GamepadAxis::RightX => 2,
        GamepadAxis::RightY => 3,
    }
}

#[cfg(target_arch = "wasm32")]
pub fn to_macroquad_axis(_axis: GamepadAxis) -> usize {
    0
}
