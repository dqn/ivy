use macroquad::prelude::*;

use crate::runtime::visual::CameraState;
use crate::scenario::CameraFocus;

/// Get focus point coordinates as normalized values (0.0 to 1.0).
fn focus_to_normalized(focus: CameraFocus) -> (f32, f32) {
    match focus {
        CameraFocus::Center => (0.5, 0.5),
        CameraFocus::TopLeft => (0.0, 0.0),
        CameraFocus::TopCenter => (0.5, 0.0),
        CameraFocus::TopRight => (1.0, 0.0),
        CameraFocus::Left => (0.0, 0.5),
        CameraFocus::Right => (1.0, 0.5),
        CameraFocus::BottomLeft => (0.0, 1.0),
        CameraFocus::BottomCenter => (0.5, 1.0),
        CameraFocus::BottomRight => (1.0, 1.0),
    }
}

/// Apply camera transformations and return the transformation parameters.
/// Returns (offset_x, offset_y, scale, rotation_radians, pivot_x, pivot_y).
pub fn calculate_camera_transform(
    camera: &CameraState,
    screen_width: f32,
    screen_height: f32,
) -> CameraTransform {
    let (focus_x, focus_y) = focus_to_normalized(camera.focus);
    let pivot_x = screen_width * focus_x;
    let pivot_y = screen_height * focus_y;

    CameraTransform {
        pan_x: camera.pan_x,
        pan_y: camera.pan_y,
        zoom: camera.zoom,
        tilt_radians: camera.tilt.to_radians(),
        pivot_x,
        pivot_y,
    }
}

/// Camera transformation parameters.
#[derive(Debug, Clone, Copy)]
pub struct CameraTransform {
    pub pan_x: f32,
    pub pan_y: f32,
    pub zoom: f32,
    pub tilt_radians: f32,
    pub pivot_x: f32,
    pub pivot_y: f32,
}

impl CameraTransform {
    /// Check if this is a default (identity) transform.
    pub fn is_identity(&self) -> bool {
        self.pan_x == 0.0 && self.pan_y == 0.0 && self.zoom == 1.0 && self.tilt_radians == 0.0
    }

    /// Apply the transformation to a point.
    pub fn transform_point(&self, x: f32, y: f32) -> (f32, f32) {
        // Translate to pivot
        let x = x - self.pivot_x;
        let y = y - self.pivot_y;

        // Apply rotation
        let cos_r = self.tilt_radians.cos();
        let sin_r = self.tilt_radians.sin();
        let rx = x * cos_r - y * sin_r;
        let ry = x * sin_r + y * cos_r;

        // Apply scale
        let sx = rx * self.zoom;
        let sy = ry * self.zoom;

        // Translate back from pivot and apply pan
        let fx = sx + self.pivot_x - self.pan_x * self.zoom;
        let fy = sy + self.pivot_y - self.pan_y * self.zoom;

        (fx, fy)
    }
}

/// Push camera transformation matrix.
pub fn push_camera_transform(transform: &CameraTransform) {
    if transform.is_identity() {
        return;
    }

    // Build transformation matrix
    // Order: translate to pivot -> rotate -> scale -> translate back + pan
    let pivot_x = transform.pivot_x;
    let pivot_y = transform.pivot_y;
    let zoom = transform.zoom;
    let cos_r = transform.tilt_radians.cos();
    let sin_r = transform.tilt_radians.sin();

    // Combined matrix:
    // 1. Translate to origin (relative to pivot)
    // 2. Rotate
    // 3. Scale
    // 4. Translate back and apply pan

    let m11 = cos_r * zoom;
    let m12 = -sin_r * zoom;
    let m21 = sin_r * zoom;
    let m22 = cos_r * zoom;

    // Translation component
    let tx = pivot_x - pivot_x * m11 - pivot_y * m21 - transform.pan_x * zoom;
    let ty = pivot_y - pivot_x * m12 - pivot_y * m22 - transform.pan_y * zoom;

    unsafe {
        get_internal_gl()
            .quad_gl
            .push_model_matrix(glam::Mat4::from_cols(
                glam::Vec4::new(m11, m12, 0.0, 0.0),
                glam::Vec4::new(m21, m22, 0.0, 0.0),
                glam::Vec4::new(0.0, 0.0, 1.0, 0.0),
                glam::Vec4::new(tx, ty, 0.0, 1.0),
            ));
    }
}

/// Pop camera transformation matrix.
pub fn pop_camera_transform(transform: &CameraTransform) {
    if transform.is_identity() {
        return;
    }

    unsafe {
        get_internal_gl().quad_gl.pop_model_matrix();
    }
}
