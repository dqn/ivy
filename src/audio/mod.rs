use std::collections::HashMap;

use macroquad::audio::{Sound, load_sound, play_sound, stop_sound};

/// Audio manager for BGM, SE, and voice playback.
pub struct AudioManager {
    sound_cache: HashMap<String, Sound>,
    current_bgm: Option<String>,
    current_bgm_sound: Option<Sound>,
    bgm_volume: f32,
    se_volume: f32,
    voice_volume: f32,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            sound_cache: HashMap::new(),
            current_bgm: None,
            current_bgm_sound: None,
            bgm_volume: 1.0,
            se_volume: 1.0,
            voice_volume: 1.0,
        }
    }

    /// Set BGM volume (0.0 - 1.0).
    pub fn set_bgm_volume(&mut self, volume: f32) {
        self.bgm_volume = volume.clamp(0.0, 1.0);
        // Note: macroquad doesn't support changing volume of playing sound
        // Volume will be applied on next BGM change
    }

    /// Set SE volume (0.0 - 1.0).
    pub fn set_se_volume(&mut self, volume: f32) {
        self.se_volume = volume.clamp(0.0, 1.0);
    }

    /// Set voice volume (0.0 - 1.0).
    pub fn set_voice_volume(&mut self, volume: f32) {
        self.voice_volume = volume.clamp(0.0, 1.0);
    }

    /// Load a sound file, using cache if available.
    async fn get_sound(&mut self, path: &str) -> Option<Sound> {
        if let Some(sound) = self.sound_cache.get(path) {
            return Some(sound.clone());
        }

        match load_sound(path).await {
            Ok(sound) => {
                self.sound_cache.insert(path.to_string(), sound.clone());
                Some(sound)
            }
            Err(e) => {
                eprintln!("Failed to load sound '{}': {}", path, e);
                None
            }
        }
    }

    /// Play or stop BGM based on the command.
    /// - None: keep current BGM
    /// - Some(""): stop BGM
    /// - Some(path): play new BGM (loop)
    pub async fn update_bgm(&mut self, bgm: Option<&String>) {
        match bgm {
            None => {
                // Keep current BGM
            }
            Some(path) if path.is_empty() => {
                // Stop BGM
                if let Some(sound) = &self.current_bgm_sound {
                    stop_sound(sound);
                }
                self.current_bgm = None;
                self.current_bgm_sound = None;
            }
            Some(path) => {
                // Check if it's already playing
                if self.current_bgm.as_ref() == Some(path) {
                    return;
                }

                // Stop current BGM
                if let Some(sound) = &self.current_bgm_sound {
                    stop_sound(sound);
                }

                // Play new BGM
                if let Some(sound) = self.get_sound(path).await {
                    play_sound(
                        &sound,
                        macroquad::audio::PlaySoundParams {
                            looped: true,
                            volume: self.bgm_volume,
                        },
                    );
                    self.current_bgm = Some(path.clone());
                    self.current_bgm_sound = Some(sound);
                }
            }
        }
    }

    /// Play a sound effect (one-shot).
    pub async fn play_se(&mut self, se: Option<&String>) {
        if let Some(path) = se {
            if path.is_empty() {
                return;
            }
            if let Some(sound) = self.get_sound(path).await {
                play_sound(
                    &sound,
                    macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: self.se_volume,
                    },
                );
            }
        }
    }

    /// Play voice (one-shot).
    pub async fn play_voice(&mut self, voice: Option<&String>) {
        if let Some(path) = voice {
            if path.is_empty() {
                return;
            }
            if let Some(sound) = self.get_sound(path).await {
                play_sound(
                    &sound,
                    macroquad::audio::PlaySoundParams {
                        looped: false,
                        volume: self.voice_volume,
                    },
                );
            }
        }
    }

    /// Get current BGM path for save data.
    pub fn current_bgm(&self) -> Option<&String> {
        self.current_bgm.as_ref()
    }

    /// Restore BGM from save data.
    pub async fn restore_bgm(&mut self, bgm: Option<String>) {
        if let Some(path) = bgm
            && !path.is_empty() {
                self.update_bgm(Some(&path)).await;
            }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
