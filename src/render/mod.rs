//! Rendering modules for the visual novel engine.
//!
//! The render module is organized into logical groups:
//!
//! ## Text and UI
//! - `text`: Text box rendering with rich text support
//! - `nvl`: NVL (full-screen text) mode
//! - `ui`: Choice buttons and navigation
//! - `widgets`: Common UI widgets
//! - `input`: Player text input dialog
//!
//! ## Visual Effects
//! - `image`: Background rendering
//! - `character`: Character sprite rendering with animations
//! - `modular_char`: Layered character sprite compositing
//! - `transition`: Scene transition effects
//! - `particles`: Particle effects (rain, snow, etc.)
//! - `shake`: Screen shake effects
//! - `cinematic`: Letterbox/cinematic bars
//! - `camera`: Camera transforms (pan, zoom, tilt)
//! - `lipsync`: Lip sync animation
//!
//! ## Screens and Menus
//! - `title`: Title screen
//! - `settings`: Settings menu
//! - `gallery`: CG gallery
//! - `chapter_select`: Chapter selection menu
//! - `backlog`: Text history/backlog viewer
//! - `achievement`: Achievement notifications
//!
//! ## Development/Debug
//! - `debug`: Debug console overlay
//! - `flowchart`: Scenario flowchart visualization
//!
//! ## Other
//! - `typewriter`: Typewriter text animation state
//! - `video`: Video playback

// --- Text and UI ---
pub mod input;
pub mod nvl;
pub mod text;
pub mod ui;
pub mod widgets;

// --- Visual Effects ---
pub mod camera;
pub mod character;
pub mod cinematic;
pub mod image;
pub mod lipsync;
pub mod modular_char;
pub mod particles;
pub mod shake;
pub mod transition;

// --- Screens and Menus ---
pub mod achievement;
pub mod backlog;
pub mod chapter_select;
pub mod gallery;
pub mod settings;
pub mod title;

// --- Development/Debug ---
pub mod debug;
pub mod flowchart;

// --- Other ---
pub mod typewriter;
pub mod video;

// Re-exports for convenient access

// Text and UI
pub use input::{InputConfig, InputState, draw_input};
pub use nvl::{NvlConfig, NvlState, count_nvl_chars, draw_nvl_text_box};
pub use text::{
    TextBoxConfig, count_visible_chars, draw_continue_indicator_with_font, draw_speaker_name,
    draw_text_box_typewriter, draw_text_box_with_font, interpolate_variables,
};
pub use ui::{ChoiceButtonConfig, ChoiceNavState, InputSource, draw_choices_with_timer};

// Visual Effects
pub use camera::{calculate_camera_transform, pop_camera_transform, push_camera_transform};
pub use character::{CharAnimationState, CharIdleState, draw_character_animated};
pub use cinematic::CinematicState;
pub use image::draw_background_with_offset;
pub use lipsync::{LipSyncConfig, LipSyncManager, LipSyncState};
pub use modular_char::draw_modular_char;
pub use particles::{ParticleState, ParticleType};
pub use shake::ShakeState;
pub use transition::TransitionState;

// Screens and Menus
pub use achievement::{AchievementConfig, draw_achievement};
pub use backlog::{BacklogConfig, BacklogState, draw_backlog};
pub use chapter_select::{ChapterSelectConfig, ChapterSelectState, draw_chapter_select};
pub use gallery::{GalleryConfig, GalleryState, draw_gallery};
pub use settings::{GameSettings, SettingsConfig, draw_settings_screen};
pub use title::{TitleConfig, TitleMenuItem, draw_title_screen};

// Development/Debug
pub use debug::{DebugConfig, DebugState, draw_debug};
pub use flowchart::{FlowchartConfig, FlowchartState, draw_flowchart};

// Other
pub use typewriter::TypewriterState;
pub use video::{VideoBackgroundState, VideoState};
