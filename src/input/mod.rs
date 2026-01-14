pub mod gamepad;

use macroquad::prelude::{KeyCode, MouseButton};

pub use gamepad::{GamepadAxis, GamepadBindings, GamepadButton, STICK_THRESHOLD};

/// Input provider trait for abstracting input handling.
///
/// This trait allows swapping between real input (macroquad) and
/// test input (mock) for testing purposes.
pub trait InputProvider {
    /// Check if a key was just pressed this frame.
    fn is_key_pressed(&self, key: KeyCode) -> bool;

    /// Check if a key is currently held down.
    fn is_key_down(&self, key: KeyCode) -> bool;

    /// Get the current mouse position.
    fn mouse_position(&self) -> (f32, f32);

    /// Check if a mouse button was just pressed this frame.
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;

    /// Get the mouse wheel delta (x, y).
    fn mouse_wheel(&self) -> (f32, f32);

    /// Get the next character pressed (for text input).
    fn get_char_pressed(&self) -> Option<char>;

    /// Check if a gamepad button was just pressed this frame.
    fn is_gamepad_button_pressed(&self, button: GamepadButton) -> bool;

    /// Check if a gamepad button is currently held down.
    fn is_gamepad_button_down(&self, button: GamepadButton) -> bool;

    /// Get the value of a gamepad axis (-1.0 to 1.0).
    fn gamepad_axis(&self, axis: GamepadAxis) -> f32;

    /// Check if any gamepad is connected.
    fn is_any_gamepad_connected(&self) -> bool;
}

/// Real input provider using macroquad.
pub struct MacroquadInput;

impl InputProvider for MacroquadInput {
    fn is_key_pressed(&self, key: KeyCode) -> bool {
        macroquad::prelude::is_key_pressed(key)
    }

    fn is_key_down(&self, key: KeyCode) -> bool {
        macroquad::prelude::is_key_down(key)
    }

    fn mouse_position(&self) -> (f32, f32) {
        macroquad::prelude::mouse_position()
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        macroquad::prelude::is_mouse_button_pressed(button)
    }

    fn mouse_wheel(&self) -> (f32, f32) {
        macroquad::prelude::mouse_wheel()
    }

    fn get_char_pressed(&self) -> Option<char> {
        macroquad::prelude::get_char_pressed()
    }

    // Gamepad support is not yet available in macroquad 0.4.
    // These are stub implementations that return false/0.0.
    // TODO: Add `gamepads` crate for gamepad support.

    fn is_gamepad_button_pressed(&self, _button: GamepadButton) -> bool {
        false
    }

    fn is_gamepad_button_down(&self, _button: GamepadButton) -> bool {
        false
    }

    fn gamepad_axis(&self, _axis: GamepadAxis) -> f32 {
        0.0
    }

    fn is_any_gamepad_connected(&self) -> bool {
        false
    }
}

/// Test input provider with queued events.
pub mod test {
    use super::*;
    use std::collections::HashSet;

    /// Mock input provider for testing.
    #[derive(Default)]
    pub struct TestInput {
        pressed_keys: HashSet<KeyCode>,
        down_keys: HashSet<KeyCode>,
        mouse_pos: (f32, f32),
        mouse_buttons: HashSet<MouseButton>,
        mouse_wheel_delta: (f32, f32),
        char_queue: Vec<char>,
        pressed_gamepad_buttons: HashSet<GamepadButton>,
        down_gamepad_buttons: HashSet<GamepadButton>,
        gamepad_axes: std::collections::HashMap<GamepadAxis, f32>,
        gamepad_connected: bool,
    }

    impl TestInput {
        /// Create a new test input provider.
        pub fn new() -> Self {
            Self::default()
        }

        /// Simulate a key press (pressed this frame).
        pub fn press_key(&mut self, key: KeyCode) {
            self.pressed_keys.insert(key);
            self.down_keys.insert(key);
        }

        /// Simulate holding a key down.
        pub fn hold_key(&mut self, key: KeyCode) {
            self.down_keys.insert(key);
        }

        /// Simulate releasing a key.
        pub fn release_key(&mut self, key: KeyCode) {
            self.down_keys.remove(&key);
        }

        /// Set mouse position.
        pub fn set_mouse_position(&mut self, x: f32, y: f32) {
            self.mouse_pos = (x, y);
        }

        /// Simulate a mouse button click.
        pub fn click(&mut self, button: MouseButton) {
            self.mouse_buttons.insert(button);
        }

        /// Simulate clicking at a specific position.
        pub fn click_at(&mut self, x: f32, y: f32) {
            self.mouse_pos = (x, y);
            self.mouse_buttons.insert(MouseButton::Left);
        }

        /// Simulate mouse wheel scroll.
        pub fn scroll(&mut self, x: f32, y: f32) {
            self.mouse_wheel_delta = (x, y);
        }

        /// Queue a character for text input.
        pub fn type_char(&mut self, ch: char) {
            self.char_queue.push(ch);
        }

        /// Simulate a gamepad button press.
        pub fn press_gamepad_button(&mut self, button: GamepadButton) {
            self.pressed_gamepad_buttons.insert(button);
            self.down_gamepad_buttons.insert(button);
        }

        /// Set gamepad axis value.
        pub fn set_gamepad_axis(&mut self, axis: GamepadAxis, value: f32) {
            self.gamepad_axes.insert(axis, value);
        }

        /// Set gamepad connected state.
        pub fn set_gamepad_connected(&mut self, connected: bool) {
            self.gamepad_connected = connected;
        }

        /// Clear frame-based input state (call between frames).
        pub fn clear_frame(&mut self) {
            self.pressed_keys.clear();
            self.mouse_buttons.clear();
            self.mouse_wheel_delta = (0.0, 0.0);
            self.pressed_gamepad_buttons.clear();
        }
    }

    impl InputProvider for TestInput {
        fn is_key_pressed(&self, key: KeyCode) -> bool {
            self.pressed_keys.contains(&key)
        }

        fn is_key_down(&self, key: KeyCode) -> bool {
            self.down_keys.contains(&key)
        }

        fn mouse_position(&self) -> (f32, f32) {
            self.mouse_pos
        }

        fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
            self.mouse_buttons.contains(&button)
        }

        fn mouse_wheel(&self) -> (f32, f32) {
            self.mouse_wheel_delta
        }

        fn get_char_pressed(&self) -> Option<char> {
            // Pop from front of queue
            None // Simplified for now
        }

        fn is_gamepad_button_pressed(&self, button: GamepadButton) -> bool {
            self.pressed_gamepad_buttons.contains(&button)
        }

        fn is_gamepad_button_down(&self, button: GamepadButton) -> bool {
            self.down_gamepad_buttons.contains(&button)
        }

        fn gamepad_axis(&self, axis: GamepadAxis) -> f32 {
            self.gamepad_axes.get(&axis).copied().unwrap_or(0.0)
        }

        fn is_any_gamepad_connected(&self) -> bool {
            self.gamepad_connected
        }
    }
}
