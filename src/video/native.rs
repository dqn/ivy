//! Native video player implementation using video-rs (FFmpeg).

use std::path::PathBuf;
use std::time::{Duration, Instant};

use video_rs::decode::Decoder;
use video_rs::location::Location;

use super::VideoPlayer;

pub struct NativeVideoPlayer {
    decoder: Option<Decoder>,
    current_frame: Vec<u8>,
    width: u32,
    height: u32,
    finished: bool,
    playing: bool,
    loop_video: bool,
    video_path: Option<PathBuf>,
    frame_duration: Duration,
    last_frame_time: Option<Instant>,
}

impl NativeVideoPlayer {
    pub fn new() -> Self {
        Self {
            decoder: None,
            current_frame: Vec::new(),
            width: 0,
            height: 0,
            finished: false,
            playing: false,
            loop_video: false,
            video_path: None,
            frame_duration: Duration::from_secs_f64(1.0 / 30.0),
            last_frame_time: None,
        }
    }

    fn decode_next_frame(&mut self) -> bool {
        let Some(decoder) = self.decoder.as_mut() else {
            return false;
        };

        match decoder.decode() {
            Ok((_time, frame)) => {
                let rgb_frame = frame
                    .slice(ndarray::s![.., .., 0..3])
                    .to_owned();

                let (height, width, _) = rgb_frame.dim();
                self.width = width as u32;
                self.height = height as u32;

                // Convert RGB to RGBA.
                let rgb_data = rgb_frame.into_raw_vec();
                self.current_frame.clear();
                self.current_frame.reserve(rgb_data.len() / 3 * 4);

                for chunk in rgb_data.chunks(3) {
                    self.current_frame.push(chunk[0]);
                    self.current_frame.push(chunk[1]);
                    self.current_frame.push(chunk[2]);
                    self.current_frame.push(255);
                }

                true
            }
            Err(_) => {
                if self.loop_video {
                    if let Some(path) = &self.video_path {
                        if let Ok(decoder) = Decoder::new(Location::File(path.clone())) {
                            self.decoder = Some(decoder);
                            return self.decode_next_frame();
                        }
                    }
                }
                self.finished = true;
                self.playing = false;
                false
            }
        }
    }
}

impl Default for NativeVideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoPlayer for NativeVideoPlayer {
    fn play(&mut self, path: &str, loop_video: bool) -> anyhow::Result<()> {
        let path_buf = PathBuf::from(path);
        let location = Location::File(path_buf.clone());
        let decoder = Decoder::new(location)?;

        let (width, height) = decoder.size();
        self.width = width as u32;
        self.height = height as u32;

        let frame_rate = decoder.frame_rate();
        self.frame_duration = Duration::from_secs_f64(1.0 / frame_rate);

        self.decoder = Some(decoder);
        self.video_path = Some(path_buf);
        self.loop_video = loop_video;
        self.finished = false;
        self.playing = true;
        self.last_frame_time = None;
        self.current_frame.clear();

        Ok(())
    }

    fn stop(&mut self) {
        self.decoder = None;
        self.playing = false;
        self.finished = true;
        self.video_path = None;
    }

    fn update(&mut self) -> Option<&[u8]> {
        if !self.playing || self.finished {
            return None;
        }

        let now = Instant::now();

        let should_decode = match self.last_frame_time {
            Some(last) => now.duration_since(last) >= self.frame_duration,
            None => true,
        };

        if should_decode {
            self.last_frame_time = Some(now);
            if !self.decode_next_frame() {
                return None;
            }
        }

        if self.current_frame.is_empty() {
            None
        } else {
            Some(&self.current_frame)
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn is_playing(&self) -> bool {
        self.playing
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
