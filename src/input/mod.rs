use macroquad::prelude::{KeyCode, MouseButton};

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

        /// Clear frame-based input state (call between frames).
        pub fn clear_frame(&mut self) {
            self.pressed_keys.clear();
            self.mouse_buttons.clear();
            self.mouse_wheel_delta = (0.0, 0.0);
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
    }
}
