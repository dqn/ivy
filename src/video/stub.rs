//! Stub video player implementation when video feature is disabled.

use super::VideoPlayer;

/// Stub video player that does nothing (used when video feature is disabled).
pub struct StubVideoPlayer;

impl StubVideoPlayer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StubVideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoPlayer for StubVideoPlayer {
    fn play(&mut self, path: &str, _loop_video: bool) -> anyhow::Result<()> {
        eprintln!(
            "Video playback is not available. \
             Build with --features video to enable video support. \
             Skipping: {}",
            path
        );
        Ok(())
    }

    fn stop(&mut self) {}

    fn update(&mut self) -> Option<&[u8]> {
        None
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn is_playing(&self) -> bool {
        false
    }

    fn dimensions(&self) -> (u32, u32) {
        (0, 0)
    }
}
