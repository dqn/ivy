use std::collections::HashMap;

use macroquad::prelude::*;

use crate::flowchart::{EdgeType, Flowchart, NodeId, NodeLayout, NodeType};

/// Flowchart viewer configuration.
pub struct FlowchartConfig {
    pub bg_color: Color,
    pub node_bg_color: Color,
    pub node_border_color: Color,
    pub node_text_color: Color,
    pub current_node_color: Color,
    pub start_node_color: Color,
    pub end_node_color: Color,
    pub edge_color: Color,
    pub choice_edge_colors: Vec<Color>,
    pub jump_edge_color: Color,
    pub conditional_edge_color: Color,
    pub title_font_size: f32,
    pub node_font_size: f32,
    pub padding: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,
}

impl Default for FlowchartConfig {
    fn default() -> Self {
        Self {
            bg_color: Color::new(0.1, 0.1, 0.15, 1.0),
            node_bg_color: Color::new(0.2, 0.2, 0.25, 0.9),
            node_border_color: WHITE,
            node_text_color: WHITE,
            current_node_color: YELLOW,
            start_node_color: Color::new(0.3, 0.7, 0.3, 1.0),
            end_node_color: Color::new(0.7, 0.3, 0.3, 1.0),
            edge_color: GRAY,
            choice_edge_colors: vec![
                Color::new(0.4, 0.7, 1.0, 1.0),
                Color::new(1.0, 0.6, 0.4, 1.0),
                Color::new(0.6, 1.0, 0.6, 1.0),
                Color::new(1.0, 0.6, 1.0, 1.0),
            ],
            jump_edge_color: Color::new(0.7, 0.7, 0.7, 0.8),
            conditional_edge_color: Color::new(1.0, 1.0, 0.5, 0.8),
            title_font_size: 32.0,
            node_font_size: 14.0,
            padding: 40.0,
            zoom_speed: 0.1,
            pan_speed: 300.0,
        }
    }
}

/// Flowchart viewer state.
#[derive(Default)]
pub struct FlowchartState {
    /// Current pan offset (for scrolling).
    pub offset: Vec2,
    /// Current zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Whether the flowchart needs to be rebuilt.
    pub dirty: bool,
}

impl FlowchartState {
    pub fn new() -> Self {
        Self {
            offset: Vec2::ZERO,
            zoom: 1.0,
            dirty: true,
        }
    }
}

/// Result of drawing the flowchart.
pub struct FlowchartResult {
    /// True if user wants to go back.
    pub back_pressed: bool,
}

/// Draw the flowchart viewer.
pub fn draw_flowchart(
    config: &FlowchartConfig,
    state: &mut FlowchartState,
    flowchart: &Flowchart,
    layouts: &HashMap<NodeId, NodeLayout>,
    current_index: Option<usize>,
    font: Option<&Font>,
) -> FlowchartResult {
    let mut back_pressed = false;

    let screen_w = screen_width();
    let screen_h = screen_height();
    let dt = get_frame_time();

    // Draw background
    draw_rectangle(0.0, 0.0, screen_w, screen_h, config.bg_color);

    // Draw title
    let title = "Flowchart";
    if let Some(f) = font {
        draw_text_ex(
            title,
            config.padding,
            config.padding + config.title_font_size,
            TextParams {
                font: Some(f),
                font_size: config.title_font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            title,
            config.padding,
            config.padding + config.title_font_size,
            config.title_font_size,
            WHITE,
        );
    }

    // Draw node count info
    let info = format!(
        "{} nodes, {} edges",
        flowchart.nodes.len(),
        flowchart.edges.len()
    );
    if let Some(f) = font {
        draw_text_ex(
            &info,
            config.padding,
            config.padding + config.title_font_size + 25.0,
            TextParams {
                font: Some(f),
                font_size: 16,
                color: GRAY,
                ..Default::default()
            },
        );
    } else {
        draw_text(
            &info,
            config.padding,
            config.padding + config.title_font_size + 25.0,
            16.0,
            GRAY,
        );
    }

    // Draw zoom level
    let zoom_text = format!("Zoom: {:.0}%", state.zoom * 100.0);
    let zoom_x = screen_w - config.padding - 100.0;
    if let Some(f) = font {
        draw_text_ex(
            &zoom_text,
            zoom_x,
            config.padding + 20.0,
            TextParams {
                font: Some(f),
                font_size: 14,
                color: GRAY,
                ..Default::default()
            },
        );
    } else {
        draw_text(&zoom_text, zoom_x, config.padding + 20.0, 14.0, GRAY);
    }

    // Handle input
    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Backspace) {
        back_pressed = true;
    }

    // Pan with arrow keys
    if is_key_down(KeyCode::Left) {
        state.offset.x += config.pan_speed * dt;
    }
    if is_key_down(KeyCode::Right) {
        state.offset.x -= config.pan_speed * dt;
    }
    if is_key_down(KeyCode::Up) {
        state.offset.y += config.pan_speed * dt;
    }
    if is_key_down(KeyCode::Down) {
        state.offset.y -= config.pan_speed * dt;
    }

    // Zoom with mouse wheel
    let wheel = mouse_wheel();
    if wheel.1 != 0.0 {
        let zoom_delta = wheel.1 * config.zoom_speed;
        state.zoom = (state.zoom + zoom_delta).clamp(0.25, 2.0);
    }

    // Reset zoom with R key
    if is_key_pressed(KeyCode::R) {
        state.zoom = 1.0;
        state.offset = Vec2::ZERO;
    }

    // Graph area starts below title
    let graph_y = config.padding + config.title_font_size + 50.0;

    // Transform helper
    let transform = |x: f32, y: f32| -> (f32, f32) {
        let tx = (x * state.zoom) + state.offset.x + config.padding;
        let ty = (y * state.zoom) + state.offset.y + graph_y;
        (tx, ty)
    };

    // Draw edges first (so they appear behind nodes)
    for edge in &flowchart.edges {
        let from_layout = match layouts.get(&edge.from) {
            Some(l) => l,
            None => continue,
        };
        let to_layout = match layouts.get(&edge.to) {
            Some(l) => l,
            None => continue,
        };

        // Calculate edge start and end points
        let (from_x, from_y) = transform(
            from_layout.x + from_layout.width / 2.0,
            from_layout.y + from_layout.height,
        );
        let (to_x, to_y) = transform(to_layout.x + to_layout.width / 2.0, to_layout.y);

        // Choose edge color based on type
        let edge_color = match edge.edge_type {
            EdgeType::Sequential => config.edge_color,
            EdgeType::Jump => config.jump_edge_color,
            EdgeType::Choice(idx) => config
                .choice_edge_colors
                .get(idx % config.choice_edge_colors.len())
                .copied()
                .unwrap_or(config.edge_color),
            EdgeType::Conditional => config.conditional_edge_color,
        };

        // Draw edge line
        let thickness = match edge.edge_type {
            EdgeType::Sequential => 2.0,
            _ => 2.5,
        };
        draw_line(from_x, from_y, to_x, to_y, thickness, edge_color);

        // Draw arrow head
        draw_arrow_head(to_x, to_y, from_x, from_y, 10.0 * state.zoom, edge_color);

        // Draw edge label if present
        if let Some(label) = &edge.label {
            let mid_x = (from_x + to_x) / 2.0;
            let mid_y = (from_y + to_y) / 2.0;
            let truncated = if label.len() > 20 {
                format!("{}...", &label[..20])
            } else {
                label.clone()
            };

            // Draw label background
            let text_dim = measure_text(&truncated, None, 12, 1.0);
            draw_rectangle(
                mid_x - text_dim.width / 2.0 - 2.0,
                mid_y - 8.0,
                text_dim.width + 4.0,
                14.0,
                Color::new(0.1, 0.1, 0.15, 0.9),
            );

            if let Some(f) = font {
                draw_text_ex(
                    &truncated,
                    mid_x - text_dim.width / 2.0,
                    mid_y + 4.0,
                    TextParams {
                        font: Some(f),
                        font_size: 12,
                        color: edge_color,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(
                    &truncated,
                    mid_x - text_dim.width / 2.0,
                    mid_y + 4.0,
                    12.0,
                    edge_color,
                );
            }
        }
    }

    // Draw nodes
    for node in &flowchart.nodes {
        let layout = match layouts.get(&node.id) {
            Some(l) => l,
            None => continue,
        };

        let (x, y) = transform(layout.x, layout.y);
        let w = layout.width * state.zoom;
        let h = layout.height * state.zoom;

        // Check if this is the current node
        let is_current = current_index
            .map(|idx| node.script_index == idx)
            .unwrap_or(false);

        // Choose node colors based on type
        let (bg_color, border_color) = match &node.node_type {
            NodeType::Start => (config.start_node_color, config.start_node_color),
            NodeType::End => (config.end_node_color, config.end_node_color),
            _ if is_current => (Color::new(0.3, 0.3, 0.1, 0.9), config.current_node_color),
            _ => (config.node_bg_color, config.node_border_color),
        };

        // Draw node background
        draw_rectangle(x, y, w, h, bg_color);

        // Draw node border
        let border_width = if is_current { 3.0 } else { 1.5 };
        draw_rectangle_lines(x, y, w, h, border_width, border_color);

        // Draw node label
        let label = match &node.node_type {
            NodeType::Start => "Start".to_string(),
            NodeType::End => "End".to_string(),
            NodeType::Label { name } => name.clone(),
            NodeType::Choice { options } => {
                format!("Choice ({})", options.len())
            }
            NodeType::Conditional { var, .. } => {
                format!("if {}", var)
            }
        };

        let font_size = (config.node_font_size * state.zoom).max(10.0);
        let text_x = x + 8.0 * state.zoom;
        let text_y = y + 20.0 * state.zoom;

        if let Some(f) = font {
            draw_text_ex(
                &label,
                text_x,
                text_y,
                TextParams {
                    font: Some(f),
                    font_size: font_size as u16,
                    color: config.node_text_color,
                    ..Default::default()
                },
            );
        } else {
            draw_text(&label, text_x, text_y, font_size, config.node_text_color);
        }

        // Draw preview text if available
        if let Some(preview) = &node.preview {
            let preview_y = text_y + font_size + 4.0;
            let preview_size = (font_size - 2.0).max(8.0);
            let truncated = if preview.len() > 25 {
                format!("{}...", &preview[..25])
            } else {
                preview.clone()
            };

            if let Some(f) = font {
                draw_text_ex(
                    &truncated,
                    text_x,
                    preview_y,
                    TextParams {
                        font: Some(f),
                        font_size: preview_size as u16,
                        color: GRAY,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(&truncated, text_x, preview_y, preview_size, GRAY);
            }
        }

        // Draw current indicator
        if is_current {
            let indicator = "[CURRENT]";
            let ind_size = (12.0 * state.zoom).max(8.0);
            let ind_x = x + w - 70.0 * state.zoom;
            let ind_y = y + h - 8.0 * state.zoom;

            if let Some(f) = font {
                draw_text_ex(
                    indicator,
                    ind_x,
                    ind_y,
                    TextParams {
                        font: Some(f),
                        font_size: ind_size as u16,
                        color: config.current_node_color,
                        ..Default::default()
                    },
                );
            } else {
                draw_text(indicator, ind_x, ind_y, ind_size, config.current_node_color);
            }
        }
    }

    // Draw help text at bottom
    let help = "Arrow keys: Pan | Mouse wheel: Zoom | R: Reset | Escape: Back";
    if let Some(f) = font {
        draw_text_ex(
            help,
            config.padding,
            screen_h - config.padding,
            TextParams {
                font: Some(f),
                font_size: 14,
                color: GRAY,
                ..Default::default()
            },
        );
    } else {
        draw_text(help, config.padding, screen_h - config.padding, 14.0, GRAY);
    }

    FlowchartResult { back_pressed }
}

/// Draw an arrow head pointing from (from_x, from_y) to (to_x, to_y).
fn draw_arrow_head(to_x: f32, to_y: f32, from_x: f32, from_y: f32, size: f32, color: Color) {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let len = (dx * dx + dy * dy).sqrt();

    if len < 0.001 {
        return;
    }

    let nx = dx / len;
    let ny = dy / len;

    // Perpendicular vector
    let px = -ny;
    let py = nx;

    // Arrow points
    let p1_x = to_x - nx * size + px * size * 0.4;
    let p1_y = to_y - ny * size + py * size * 0.4;
    let p2_x = to_x - nx * size - px * size * 0.4;
    let p2_y = to_y - ny * size - py * size * 0.4;

    draw_triangle(
        Vec2::new(to_x, to_y),
        Vec2::new(p1_x, p1_y),
        Vec2::new(p2_x, p2_y),
        color,
    );
}
