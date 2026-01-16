//! Game mode handlers.
//!
//! Each handler processes one frame for a specific game mode
//! and returns a `HandlerResult` indicating what should happen next.

mod chapters;
mod flowchart;
mod gallery;
pub mod ingame;
mod settings;
mod title;

pub use chapters::handle_chapters;
pub use flowchart::handle_flowchart;
pub use gallery::handle_gallery;
pub use ingame::handle_ingame;
pub use settings::handle_settings;
pub use title::handle_title;

use super::GameMode;

/// Result of a handler execution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandlerResult {
    /// Continue in the current mode.
    Continue,
    /// Transition to a different mode.
    Transition(GameMode),
    /// Exit the application.
    Exit,
}
