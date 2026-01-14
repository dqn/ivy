//! WASM video player implementation using HTML5 video element.

use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

use super::VideoPlayer;

pub struct WasmVideoPlayer {
    video_element: Option<HtmlVideoElement>,
    canvas: Option<HtmlCanvasElement>,
    ctx: Option<CanvasRenderingContext2d>,
    frame_data: Vec<u8>,
    width: u32,
    height: u32,
    finished: bool,
    playing: bool,
    loop_video: bool,
}

impl WasmVideoPlayer {
    pub fn new() -> Self {
        Self {
            video_element: None,
            canvas: None,
            ctx: None,
            frame_data: Vec::new(),
            width: 0,
            height: 0,
            finished: false,
            playing: false,
            loop_video: false,
        }
    }

    fn create_hidden_video(&mut self, path: &str) -> anyhow::Result<()> {
        let window =
            web_sys::window().ok_or_else(|| anyhow::anyhow!("Failed to get window"))?;
        let document = window
            .document()
            .ok_or_else(|| anyhow::anyhow!("Failed to get document"))?;

        let video: HtmlVideoElement = document
            .create_element("video")
            .map_err(|_| anyhow::anyhow!("Failed to create video element"))?
            .dyn_into()
            .map_err(|_| anyhow::anyhow!("Failed to cast to HtmlVideoElement"))?;

        video.set_src(path);
        video.set_autoplay(true);
        video.set_muted(false);
        video.set_loop(self.loop_video);

        video
            .style()
            .set_property("display", "none")
            .map_err(|_| anyhow::anyhow!("Failed to hide video element"))?;

        document
            .body()
            .ok_or_else(|| anyhow::anyhow!("Failed to get body"))?
            .append_child(&video)
            .map_err(|_| anyhow::anyhow!("Failed to append video to body"))?;

        self.video_element = Some(video);
        Ok(())
    }

    fn create_canvas(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        let window =
            web_sys::window().ok_or_else(|| anyhow::anyhow!("Failed to get window"))?;
        let document = window
            .document()
            .ok_or_else(|| anyhow::anyhow!("Failed to get document"))?;

        let canvas: HtmlCanvasElement = document
            .create_element("canvas")
            .map_err(|_| anyhow::anyhow!("Failed to create canvas element"))?
            .dyn_into()
            .map_err(|_| anyhow::anyhow!("Failed to cast to HtmlCanvasElement"))?;

        canvas.set_width(width);
        canvas.set_height(height);

        canvas
            .style()
            .set_property("display", "none")
            .map_err(|_| anyhow::anyhow!("Failed to hide canvas element"))?;

        let ctx: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .map_err(|_| anyhow::anyhow!("Failed to get 2d context"))?
            .ok_or_else(|| anyhow::anyhow!("2d context is None"))?
            .dyn_into()
            .map_err(|_| anyhow::anyhow!("Failed to cast to CanvasRenderingContext2d"))?;

        self.canvas = Some(canvas);
        self.ctx = Some(ctx);
        self.width = width;
        self.height = height;

        Ok(())
    }

    fn cleanup(&mut self) {
        if let Some(video) = self.video_element.take() {
            video.pause().ok();
            if let Some(parent) = video.parent_node() {
                parent.remove_child(&video).ok();
            }
        }

        if let Some(canvas) = self.canvas.take() {
            if let Some(parent) = canvas.parent_node() {
                parent.remove_child(&canvas).ok();
            }
        }

        self.ctx = None;
    }
}

impl Default for WasmVideoPlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for WasmVideoPlayer {
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl VideoPlayer for WasmVideoPlayer {
    fn play(&mut self, path: &str, loop_video: bool) -> anyhow::Result<()> {
        self.cleanup();

        self.loop_video = loop_video;
        self.finished = false;
        self.playing = true;
        self.frame_data.clear();

        self.create_hidden_video(path)?;

        Ok(())
    }

    fn stop(&mut self) {
        self.cleanup();
        self.playing = false;
        self.finished = true;
    }

    fn update(&mut self) -> Option<&[u8]> {
        if !self.playing || self.finished {
            return None;
        }

        let video = self.video_element.as_ref()?;

        if video.ended() && !self.loop_video {
            self.finished = true;
            self.playing = false;
            return None;
        }

        if video.error().is_some() {
            self.finished = true;
            self.playing = false;
            return None;
        }

        let video_width = video.video_width();
        let video_height = video.video_height();

        if video_width == 0 || video_height == 0 {
            return None;
        }

        if self.canvas.is_none() || self.width != video_width || self.height != video_height {
            if self.create_canvas(video_width, video_height).is_err() {
                return None;
            }
        }

        let canvas = self.canvas.as_ref()?;
        let ctx = self.ctx.as_ref()?;

        ctx.draw_image_with_html_video_element(video, 0.0, 0.0)
            .ok()?;

        let image_data = ctx
            .get_image_data(0.0, 0.0, canvas.width() as f64, canvas.height() as f64)
            .ok()?;

        self.frame_data = image_data.data().to_vec();

        Some(&self.frame_data)
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
