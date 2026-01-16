//! Player action detection and input handling.

use macroquad::prelude::*;

use crate::input::{GamepadAxis, GamepadButton, GamepadState, STICK_THRESHOLD};
use crate::runtime::{Action, KeyBindings};

/// Actions that can be triggered by player input.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerAction {
    /// Advance to next text/command
    Advance,
    /// Complete typewriter animation instantly
    CompleteText,
    /// Toggle auto mode
    ToggleAuto,
    /// Toggle skip mode
    ToggleSkip,
    /// Toggle backlog
    ToggleBacklog,
    /// Toggle debug console
    ToggleDebug,
    /// Open flowchart
    OpenFlowchart,
    /// Quick save
    QuickSave,
    /// Quick load
    QuickLoad,
    /// Rollback to previous state
    Rollback,
    /// Take screenshot
    Screenshot,
    /// Save to slot (1-10)
    SaveToSlot(u8),
    /// Load from slot (1-10)
    LoadFromSlot(u8),
    /// Return to title
    ReturnToTitle,
    /// Exit application
    Exit,
}

/// Detected navigation input for choices.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChoiceNavAction {
    /// Move selection up
    Up,
    /// Move selection down
    Down,
    /// Confirm selection (A button)
    Confirm,
    /// Mouse moved
    MouseMoved,
}

/// Captured input state for a single frame.
/// This struct holds the results of input detection, allowing the InputDetector
/// to be dropped before the results are used.
#[derive(Debug, Clone, Default)]
pub struct DetectedInput {
    /// Actions triggered this frame
    pub actions: Vec<PlayerAction>,
    /// Whether advance was pressed
    pub advance_pressed: bool,
    /// Whether skip should be active (Ctrl held)
    pub ctrl_held: bool,
    /// Choice navigation action (if any)
    pub choice_nav: Option<ChoiceNavAction>,
}

impl DetectedInput {
    /// Check if skip mode should be active.
    pub fn is_skip_active(&self, skip_mode: bool) -> bool {
        skip_mode || self.ctrl_held
    }
}

/// Input detection context.
pub struct InputDetector<'a> {
    pub keybinds: &'a KeyBindings,
    pub gamepad: &'a GamepadState,
}

impl<'a> InputDetector<'a> {
    pub fn new(keybinds: &'a KeyBindings, gamepad: &'a GamepadState) -> Self {
        Self { keybinds, gamepad }
    }

    /// Capture all input for this frame into a DetectedInput struct.
    pub fn capture(&self, last_mouse_pos: (f32, f32), stick_debounce: f32) -> DetectedInput {
        DetectedInput {
            actions: self.detect_actions(),
            advance_pressed: self.is_advance_pressed(),
            ctrl_held: is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl),
            choice_nav: self.detect_choice_nav(last_mouse_pos, stick_debounce),
        }
    }

    /// Check if advance action is triggered (click, enter, gamepad A).
    pub fn is_advance_pressed(&self) -> bool {
        is_mouse_button_pressed(MouseButton::Left)
            || self
                .keybinds
                .is_pressed_with_gamepad(Action::Advance, self.gamepad)
    }

    /// Check if skip mode should be active (Ctrl held or skip mode on).
    pub fn is_skip_active(&self, skip_mode: bool) -> bool {
        skip_mode || is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl)
    }

    /// Detect all triggered actions this frame.
    pub fn detect_actions(&self) -> Vec<PlayerAction> {
        let mut actions = Vec::new();

        // Quick save/load
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::QuickSave, self.gamepad)
        {
            actions.push(PlayerAction::QuickSave);
        }
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::QuickLoad, self.gamepad)
        {
            actions.push(PlayerAction::QuickLoad);
        }

        // Toggle actions
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::Backlog, self.gamepad)
        {
            actions.push(PlayerAction::ToggleBacklog);
        }
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::AutoMode, self.gamepad)
        {
            actions.push(PlayerAction::ToggleAuto);
        }
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::SkipMode, self.gamepad)
        {
            actions.push(PlayerAction::ToggleSkip);
        }
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::Debug, self.gamepad)
        {
            actions.push(PlayerAction::ToggleDebug);
        }

        // Flowchart (F key)
        if is_key_pressed(KeyCode::F) {
            actions.push(PlayerAction::OpenFlowchart);
        }

        // Rollback (keybind or mouse wheel up)
        let wheel = mouse_wheel();
        if self
            .keybinds
            .is_pressed_with_gamepad(Action::Rollback, self.gamepad)
            || wheel.1 > 0.0
        {
            actions.push(PlayerAction::Rollback);
        }

        // Screenshot
        if self.keybinds.is_pressed(Action::Screenshot) {
            actions.push(PlayerAction::Screenshot);
        }

        // Escape
        if is_key_pressed(KeyCode::Escape) {
            actions.push(PlayerAction::ReturnToTitle);
        }

        // Slot save/load (Shift+1-0 for save, 1-0 for load)
        let shift_held = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
        let slot_keys = [
            (KeyCode::Key1, 1),
            (KeyCode::Key2, 2),
            (KeyCode::Key3, 3),
            (KeyCode::Key4, 4),
            (KeyCode::Key5, 5),
            (KeyCode::Key6, 6),
            (KeyCode::Key7, 7),
            (KeyCode::Key8, 8),
            (KeyCode::Key9, 9),
            (KeyCode::Key0, 10),
        ];

        for (key, slot) in slot_keys {
            if is_key_pressed(key) {
                if shift_held {
                    actions.push(PlayerAction::SaveToSlot(slot));
                } else {
                    actions.push(PlayerAction::LoadFromSlot(slot));
                }
            }
        }

        actions
    }

    /// Detect choice navigation actions.
    pub fn detect_choice_nav(
        &self,
        last_mouse_pos: (f32, f32),
        stick_debounce: f32,
    ) -> Option<ChoiceNavAction> {
        let mouse_pos = mouse_position();
        if mouse_pos != last_mouse_pos {
            return Some(ChoiceNavAction::MouseMoved);
        }

        // D-pad
        if self.gamepad.is_button_pressed(GamepadButton::DPadUp) {
            return Some(ChoiceNavAction::Up);
        }
        if self.gamepad.is_button_pressed(GamepadButton::DPadDown) {
            return Some(ChoiceNavAction::Down);
        }

        // Stick (with debounce)
        if stick_debounce <= 0.0 {
            let stick_y = self.gamepad.axis(GamepadAxis::LeftY);
            if stick_y < -STICK_THRESHOLD {
                return Some(ChoiceNavAction::Up);
            }
            if stick_y > STICK_THRESHOLD {
                return Some(ChoiceNavAction::Down);
            }
        }

        // A button for confirm
        if self.gamepad.is_button_pressed(GamepadButton::A) {
            return Some(ChoiceNavAction::Confirm);
        }

        None
    }
}
