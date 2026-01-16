use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::scenario::{CameraFocus, CharAnimation, CharIdleAnimation, CharPosition, Easing};

/// Single character state for multi-character support.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CharacterState {
    pub path: String,
    pub position: CharPosition,
    /// Enter animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter: Option<CharAnimation>,
    /// Exit animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<CharAnimation>,
    /// Idle animation for this character (optional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle: Option<CharIdleAnimation>,
}

/// Camera state for dynamic camera effects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CameraState {
    /// Horizontal pan offset in pixels.
    #[serde(default)]
    pub pan_x: f32,
    /// Vertical pan offset in pixels.
    #[serde(default)]
    pub pan_y: f32,
    /// Zoom level (1.0 = normal).
    #[serde(default = "default_zoom")]
    pub zoom: f32,
    /// Tilt angle in degrees.
    #[serde(default)]
    pub tilt: f32,
    /// Focus point for zoom.
    #[serde(default)]
    pub focus: CameraFocus,
}

fn default_zoom() -> f32 {
    1.0
}

impl CameraState {
    /// Check if camera is at default position.
    pub fn is_default(&self) -> bool {
        self.pan_x == 0.0 && self.pan_y == 0.0 && self.zoom == 1.0 && self.tilt == 0.0
    }
}

/// Camera animation state for smooth transitions.
#[derive(Debug, Clone, Default)]
pub struct CameraAnimationState {
    /// Starting camera state.
    pub from: CameraState,
    /// Target camera state.
    pub to: CameraState,
    /// Animation progress (0.0 to 1.0).
    pub progress: f32,
    /// Animation duration in seconds.
    pub duration: f32,
    /// Easing function.
    pub easing: Easing,
    /// Whether animation is active.
    pub active: bool,
}

impl CameraAnimationState {
    /// Get the interpolated camera state based on current progress.
    pub fn current(&self) -> CameraState {
        if !self.active || self.progress >= 1.0 {
            return self.to.clone();
        }

        let t = self.easing.apply(self.progress);

        CameraState {
            pan_x: self.from.pan_x + (self.to.pan_x - self.from.pan_x) * t,
            pan_y: self.from.pan_y + (self.to.pan_y - self.from.pan_y) * t,
            zoom: self.from.zoom + (self.to.zoom - self.from.zoom) * t,
            tilt: self.from.tilt + (self.to.tilt - self.from.tilt) * t,
            focus: self.to.focus,
        }
    }

    /// Update animation progress.
    pub fn update(&mut self, delta: f32) {
        if self.active && self.duration > 0.0 {
            self.progress += delta / self.duration;
            if self.progress >= 1.0 {
                self.progress = 1.0;
                self.active = false;
            }
        }
    }

    /// Start a new camera animation.
    pub fn start(&mut self, from: CameraState, to: CameraState, duration: f32, easing: Easing) {
        self.from = from;
        self.to = to;
        self.progress = 0.0;
        self.duration = duration;
        self.easing = easing;
        self.active = duration > 0.0;
    }
}

/// Modular character state for layered sprite compositing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModularCharState {
    /// Character definition name.
    pub name: String,
    /// Screen position.
    pub position: CharPosition,
    /// Layer variant selections (layer_name -> variant_index).
    #[serde(default)]
    pub variants: HashMap<String, usize>,
}

/// Visual state (background and character sprite).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualState {
    pub background: Option<String>,
    pub character: Option<String>,
    #[serde(default)]
    pub char_pos: CharPosition,
    /// Multiple characters (used when `characters` field is set in command).
    #[serde(default)]
    pub characters: Vec<CharacterState>,
    /// NVL mode (full-screen text display).
    #[serde(default)]
    pub nvl_mode: bool,
    /// Modular character (layered sprite compositing).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modular_char: Option<ModularCharState>,
}
