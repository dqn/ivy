use macroquad::prelude::*;

/// Manages typewriter text animation state.
#[derive(Default)]
pub struct TypewriterState {
    /// Total character count in current text.
    total_chars: usize,
    /// Characters displayed so far.
    displayed_chars: usize,
    /// Elapsed time since text started.
    elapsed: f32,
    /// Whether text is fully displayed.
    complete: bool,
}

impl TypewriterState {
    /// Reset for new text.
    pub fn reset(&mut self, total_chars: usize) {
        self.total_chars = total_chars;
        self.displayed_chars = 0;
        self.elapsed = 0.0;
        self.complete = total_chars == 0;
    }

    /// Update the typewriter state.
    /// Returns the number of characters to display.
    pub fn update(&mut self, cps: f32) -> usize {
        if self.complete {
            return self.total_chars;
        }

        // CPS of 0 means instant display
        if cps <= 0.0 {
            self.complete = true;
            self.displayed_chars = self.total_chars;
            return self.total_chars;
        }

        self.elapsed += get_frame_time();

        // Calculate how many characters should be displayed
        let chars_to_display = (self.elapsed * cps) as usize;
        self.displayed_chars = chars_to_display.min(self.total_chars);

        if self.displayed_chars >= self.total_chars {
            self.complete = true;
            self.displayed_chars = self.total_chars;
        }

        self.displayed_chars
    }

    /// Complete the text instantly (on click).
    pub fn complete(&mut self) {
        self.complete = true;
        self.displayed_chars = self.total_chars;
    }

    /// Check if text is fully displayed.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Get the number of characters to display.
    pub fn displayed_chars(&self) -> usize {
        self.displayed_chars
    }
}
