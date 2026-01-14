use std::collections::HashMap;

use macroquad::prelude::*;

use crate::cache::TextureCache;
use crate::runtime::visual::ModularCharState;
use crate::scenario::{CharPosition, ModularCharDef};

/// Get layer images for a modular character based on variant selections.
pub fn get_modular_char_layers(
    def: &ModularCharDef,
    variants: &HashMap<String, usize>,
) -> Vec<String> {
    let mut layers = vec![def.base.clone()];

    for layer_def in &def.layers {
        let index = variants.get(&layer_def.name).copied().unwrap_or(0);
        if let Some(image) = layer_def.images.get(index) {
            layers.push(image.clone());
        } else if let Some(image) = layer_def.images.first() {
            // Fall back to first variant if index is out of bounds
            layers.push(image.clone());
        }
    }

    layers
}

/// Draw a modular character by compositing layers.
pub async fn draw_modular_char(
    state: &ModularCharState,
    def: &ModularCharDef,
    texture_cache: &mut TextureCache,
    offset: (f32, f32),
) {
    let layers = get_modular_char_layers(def, &state.variants);

    if layers.is_empty() {
        return;
    }

    // Load all textures first
    let mut textures = Vec::new();
    for path in &layers {
        if let Some(tex) = texture_cache.get(path).await {
            textures.push(tex);
        }
    }

    if textures.is_empty() {
        return;
    }

    // Use first texture (base) for dimensions
    let base_texture = &textures[0];
    let tex_width = base_texture.width();
    let tex_height = base_texture.height();

    // Calculate position based on CharPosition
    let screen_width = screen_width();
    let screen_height = screen_height();

    let x = match state.position {
        CharPosition::Left => screen_width * 0.25 - tex_width / 2.0 + offset.0,
        CharPosition::Center => screen_width / 2.0 - tex_width / 2.0 + offset.0,
        CharPosition::Right => screen_width * 0.75 - tex_width / 2.0 + offset.0,
    };

    // Align to bottom of screen
    let y = screen_height - tex_height + offset.1;

    // Draw all layers in order (back to front)
    for texture in &textures {
        draw_texture(texture, x, y, WHITE);
    }
}

/// Draw a modular character with scale and alpha.
pub async fn draw_modular_char_ex(
    state: &ModularCharState,
    def: &ModularCharDef,
    texture_cache: &mut TextureCache,
    offset: (f32, f32),
    scale: f32,
    alpha: f32,
) {
    let layers = get_modular_char_layers(def, &state.variants);

    if layers.is_empty() {
        return;
    }

    // Load all textures first
    let mut textures = Vec::new();
    for path in &layers {
        if let Some(tex) = texture_cache.get(path).await {
            textures.push(tex);
        }
    }

    if textures.is_empty() {
        return;
    }

    // Use first texture (base) for dimensions
    let base_texture = &textures[0];
    let tex_width = base_texture.width() * scale;
    let tex_height = base_texture.height() * scale;

    // Calculate position based on CharPosition
    let screen_width = screen_width();
    let screen_height = screen_height();

    let x = match state.position {
        CharPosition::Left => screen_width * 0.25 - tex_width / 2.0 + offset.0,
        CharPosition::Center => screen_width / 2.0 - tex_width / 2.0 + offset.0,
        CharPosition::Right => screen_width * 0.75 - tex_width / 2.0 + offset.0,
    };

    // Align to bottom of screen
    let y = screen_height - tex_height + offset.1;

    let color = Color::new(1.0, 1.0, 1.0, alpha);

    // Draw all layers in order (back to front)
    for texture in &textures {
        draw_texture_ex(
            texture,
            x,
            y,
            color,
            DrawTextureParams {
                dest_size: Some(Vec2::new(tex_width, tex_height)),
                ..Default::default()
            },
        );
    }
}
