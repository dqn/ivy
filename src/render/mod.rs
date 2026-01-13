pub mod backlog;
pub mod image;
pub mod settings;
pub mod text;
pub mod title;
pub mod transition;
pub mod ui;

pub use backlog::{draw_backlog, BacklogConfig, BacklogState};
pub use image::{draw_background, draw_character};
pub use settings::{draw_settings_screen, GameSettings, SettingsConfig};
pub use text::{
    draw_continue_indicator, draw_continue_indicator_with_font, draw_text_box,
    draw_text_box_with_font, TextBoxConfig,
};
pub use title::{draw_title_screen, TitleConfig, TitleMenuItem};
pub use transition::TransitionState;
pub use ui::{draw_choices, ChoiceButtonConfig};
