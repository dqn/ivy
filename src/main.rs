// Many public APIs are not used in main but are provided for external use or future features.
#![allow(dead_code)]

mod accessibility;
mod audio;
mod cache;
mod flowchart;
mod game;
mod hotreload;
mod i18n;
mod input;
mod platform;
mod render;
mod runtime;
mod scenario;
mod types;
mod video;

use macroquad::prelude::*;

use game::{
    GameContext, GameMode, HandlerResult, handle_chapters, handle_flowchart, handle_gallery,
    handle_ingame, handle_settings, handle_title, window_conf,
};

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize game context and font (font is separate to avoid borrow conflicts)
    let (mut ctx, custom_font) = match GameContext::new().await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    // Start with title screen
    let mut game_mode = GameMode::Title;

    // Font reference is separate from ctx to avoid borrow conflicts
    let font_ref = custom_font.as_ref();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));

        // Poll gamepad events at the start of each frame
        ctx.gamepad_state.poll();

        let result = match game_mode {
            GameMode::Title => handle_title(&mut ctx, font_ref),
            GameMode::Settings => handle_settings(&mut ctx, font_ref),
            GameMode::Gallery => handle_gallery(&mut ctx, font_ref).await,
            GameMode::Chapters => handle_chapters(&mut ctx, font_ref),
            GameMode::Flowchart => handle_flowchart(&mut ctx, font_ref),
            GameMode::InGame => handle_ingame(&mut ctx, font_ref).await,
        };

        match result {
            HandlerResult::Continue => {}
            HandlerResult::Transition(new_mode) => {
                game_mode = new_mode;
            }
            HandlerResult::Exit => break,
        }

        next_frame().await;
    }
}
