//! Video playback rendering state.

use macroquad::prelude::*;

use crate::video::{PlatformVideoPlayer, VideoPlayer, create_video_player};

/// Video playback state for rendering.
pub struct VideoState {
    player: PlatformVideoPlayer,
    texture: Option<Texture2D>,
    skippable: bool,
    playing: bool,
}

/// Video background state for rendering looping videos as backgrounds.
pub struct VideoBackgroundState {
    player: PlatformVideoPlayer,
    texture: Option<Texture2D>,
    playing: bool,
    current_path: Option<String>,
    looped: bool,
}

impl VideoBackgroundState {
    pub fn new() -> Self {
        Self {
            player: create_video_player(),
            texture: None,
            playing: false,
            current_path: None,
            looped: true,
        }
    }

    /// Start playing a video as background.
    pub fn start(&mut self, path: &str, looped: bool) -> anyhow::Result<()> {
        // If same video is already playing, don't restart.
        if self.playing && self.current_path.as_deref() == Some(path) && self.looped == looped {
            return Ok(());
        }

        self.player.play(path, looped)?;
        self.playing = true;
        self.current_path = Some(path.to_string());
        self.looped = looped;
        self.texture = None;
        Ok(())
    }

    /// Stop video background playback.
    pub fn stop(&mut self) {
        self.player.stop();
        self.playing = false;
        self.current_path = None;
        self.texture = None;
    }

    /// Update video background state and texture.
    pub fn update(&mut self) {
        if !self.playing {
            return;
        }

        // Get dimensions first to avoid borrow conflicts.
        let (w, h) = self.player.dimensions();

        if let Some(frame_data) = self.player.update()
            && w > 0
            && h > 0
        {
            // Copy frame data to avoid borrow issues.
            let frame_data = frame_data.to_vec();

            if let Some(ref texture) = self.texture
                && (texture.width() as u32 != w || texture.height() as u32 != h)
            {
                self.texture = None;
            }

            if self.texture.is_none() {
                let texture = Texture2D::from_rgba8(w as u16, h as u16, &frame_data);
                texture.set_filter(FilterMode::Linear);
                self.texture = Some(texture);
            } else if let Some(ref texture) = self.texture {
                texture.update(&Image {
                    bytes: frame_data,
                    width: w as u16,
                    height: h as u16,
                });
            }
        }

        if self.player.is_finished() && !self.looped {
            self.playing = false;
        }
    }

    /// Draw the video as a fullscreen background (covers entire screen).
    pub fn draw(&self) {
        if let Some(ref texture) = self.texture {
            let sw = screen_width();
            let sh = screen_height();
            let tw = texture.width();
            let th = texture.height();

            // Calculate cover scaling (fill entire screen, may crop).
            let scale_w = sw / tw;
            let scale_h = sh / th;
            let scale = scale_w.max(scale_h);

            let draw_w = tw * scale;
            let draw_h = th * scale;
            let x = (sw - draw_w) / 2.0;
            let y = (sh - draw_h) / 2.0;

            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(draw_w, draw_h)),
                    ..Default::default()
                },
            );
        }
    }

    /// Check if video background is currently playing.
    pub fn is_playing(&self) -> bool {
        self.playing && self.player.is_playing()
    }

    /// Check if video background has finished (only relevant for non-looped videos).
    pub fn is_finished(&self) -> bool {
        !self.playing || (!self.looped && self.player.is_finished())
    }

    /// Get the current video path if playing.
    pub fn current_path(&self) -> Option<&str> {
        if self.playing {
            self.current_path.as_deref()
        } else {
            None
        }
    }
}

impl Default for VideoBackgroundState {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoState {
    pub fn new() -> Self {
        Self {
            player: create_video_player(),
            texture: None,
            skippable: true,
            playing: false,
        }
    }

    /// Start playing a video.
    pub fn start(&mut self, path: &str, skippable: bool, loop_video: bool) -> anyhow::Result<()> {
        self.player.play(path, loop_video)?;
        self.skippable = skippable;
        self.playing = true;
        self.texture = None;
        Ok(())
    }

    /// Stop video playback.
    pub fn stop(&mut self) {
        self.player.stop();
        self.playing = false;
        self.texture = None;
    }

    /// Update video state and texture.
    pub fn update(&mut self) {
        if !self.playing {
            return;
        }

        // Get dimensions first to avoid borrow conflicts.
        let (w, h) = self.player.dimensions();

        if let Some(frame_data) = self.player.update()
            && w > 0
            && h > 0
        {
            // Copy frame data to avoid borrow issues.
            let frame_data = frame_data.to_vec();

            if let Some(ref texture) = self.texture
                && (texture.width() as u32 != w || texture.height() as u32 != h)
            {
                self.texture = None;
            }

            if self.texture.is_none() {
                let texture = Texture2D::from_rgba8(w as u16, h as u16, &frame_data);
                texture.set_filter(FilterMode::Linear);
                self.texture = Some(texture);
            } else if let Some(ref texture) = self.texture {
                texture.update(&Image {
                    bytes: frame_data,
                    width: w as u16,
                    height: h as u16,
                });
            }
        }

        if self.player.is_finished() {
            self.playing = false;
        }
    }

    /// Draw the video fullscreen.
    pub fn draw(&self) {
        clear_background(BLACK);

        if let Some(ref texture) = self.texture {
            let sw = screen_width();
            let sh = screen_height();
            let tw = texture.width();
            let th = texture.height();

            // Calculate aspect-correct scaling.
            let scale_w = sw / tw;
            let scale_h = sh / th;
            let scale = scale_w.min(scale_h);

            let draw_w = tw * scale;
            let draw_h = th * scale;
            let x = (sw - draw_w) / 2.0;
            let y = (sh - draw_h) / 2.0;

            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(draw_w, draw_h)),
                    ..Default::default()
                },
            );
        }
    }

    /// Check if the video can be skipped.
    pub fn can_skip(&self) -> bool {
        self.skippable
    }

    /// Check if video has finished playing.
    pub fn is_finished(&self) -> bool {
        !self.playing || self.player.is_finished()
    }

    /// Check if video is currently playing.
    pub fn is_playing(&self) -> bool {
        self.playing && self.player.is_playing()
    }
}

impl Default for VideoState {
    fn default() -> Self {
        Self::new()
    }
}
