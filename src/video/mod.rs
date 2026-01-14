//! Video playback module with platform-specific implementations.

#[cfg(all(not(target_arch = "wasm32"), feature = "video"))]
mod native;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "video")))]
mod stub;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(all(not(target_arch = "wasm32"), feature = "video"))]
pub use native::NativeVideoPlayer as PlatformVideoPlayer;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "video")))]
pub use stub::StubVideoPlayer as PlatformVideoPlayer;
#[cfg(target_arch = "wasm32")]
pub use wasm::WasmVideoPlayer as PlatformVideoPlayer;

/// Video playback trait for platform abstraction.
pub trait VideoPlayer {
    /// Start playing a video from the given path.
    fn play(&mut self, path: &str, loop_video: bool) -> anyhow::Result<()>;

    /// Stop playback and release resources.
    fn stop(&mut self);

    /// Update and get current frame as RGBA bytes.
    /// Returns None if no frame is available.
    fn update(&mut self) -> Option<&[u8]>;

    /// Check if video has finished playing.
    fn is_finished(&self) -> bool;

    /// Check if video is currently playing.
    fn is_playing(&self) -> bool;

    /// Get video dimensions (width, height).
    fn dimensions(&self) -> (u32, u32);
}

/// Create a new platform-specific video player.
pub fn create_video_player() -> PlatformVideoPlayer {
    PlatformVideoPlayer::new()
}
