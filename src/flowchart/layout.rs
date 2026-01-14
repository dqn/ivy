use std::collections::{HashMap, HashSet, VecDeque};

use super::types::{Flowchart, NodeId};

/// Node position in the flowchart layout.
#[derive(Debug, Clone, Copy)]
pub struct NodeLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    #[allow(dead_code)]
    pub layer: usize,
}

/// Layout configuration.
pub struct LayoutConfig {
    pub node_width: f32,
    pub node_height: f32,
    pub horizontal_gap: f32,
    pub vertical_gap: f32,
    pub padding: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            node_width: 180.0,
            node_height: 60.0,
            horizontal_gap: 40.0,
            vertical_gap: 80.0,
            padding: 50.0,
        }
    }
}

/// Calculate layout for all nodes using a layered approach.
pub fn calculate_layout(flowchart: &Flowchart, config: &LayoutConfig) -> HashMap<NodeId, NodeLayout> {
    let mut layouts = HashMap::new();

    if flowchart.nodes.is_empty() {
        return layouts;
    }

    // Phase 1: Assign layers using BFS from start node
    let layers = assign_layers(flowchart);

    // Phase 2: Assign horizontal positions within layers
    let positions = assign_positions(&layers, config);

    // Phase 3: Create NodeLayout for each node
    for (node_id, (layer, position)) in positions {
        let x = config.padding + position as f32 * (config.node_width + config.horizontal_gap);
        let y = config.padding + layer as f32 * (config.node_height + config.vertical_gap);

        layouts.insert(
            node_id,
            NodeLayout {
                x,
                y,
                width: config.node_width,
                height: config.node_height,
                layer,
            },
        );
    }

    layouts
}

/// Assign layers to nodes using BFS from start node.
fn assign_layers(flowchart: &Flowchart) -> HashMap<NodeId, usize> {
    let mut layers = HashMap::new();

    if flowchart.nodes.is_empty() {
        return layers;
    }

    // Build adjacency list
    let mut adjacency: HashMap<NodeId, Vec<NodeId>> = HashMap::new();
    for edge in &flowchart.edges {
        adjacency.entry(edge.from).or_default().push(edge.to);
    }

    // BFS from first node (assumed to be start)
    let start_id = NodeId(0);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start_id, 0usize));
    visited.insert(start_id);

    while let Some((node_id, layer)) = queue.pop_front() {
        // Only update if not already assigned or if this path is shorter
        let current_layer = layers.entry(node_id).or_insert(layer);
        if layer > *current_layer {
            continue; // Skip if we found a longer path
        }
        *current_layer = layer;

        if let Some(neighbors) = adjacency.get(&node_id) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, layer + 1));
                }
            }
        }
    }

    // Handle any unvisited nodes (disconnected components)
    for node in &flowchart.nodes {
        if !layers.contains_key(&node.id) {
            // Place at the end
            let max_layer = layers.values().copied().max().unwrap_or(0);
            layers.insert(node.id, max_layer + 1);
        }
    }

    layers
}

/// Assign horizontal positions within each layer.
fn assign_positions(
    layers: &HashMap<NodeId, usize>,
    _config: &LayoutConfig,
) -> HashMap<NodeId, (usize, usize)> {
    let mut positions = HashMap::new();

    // Group nodes by layer
    let mut layer_nodes: HashMap<usize, Vec<NodeId>> = HashMap::new();
    for (&node_id, &layer) in layers {
        layer_nodes.entry(layer).or_default().push(node_id);
    }

    // Sort nodes within each layer by their ID (maintains order)
    for nodes in layer_nodes.values_mut() {
        nodes.sort_by_key(|n| n.0);
    }

    // Assign positions within each layer
    for (&layer, nodes) in &layer_nodes {
        for (pos, &node_id) in nodes.iter().enumerate() {
            positions.insert(node_id, (layer, pos));
        }
    }

    // Simple centering: shift positions to center each layer
    let max_width = layer_nodes.values().map(|n| n.len()).max().unwrap_or(1);

    for nodes in layer_nodes.values() {
        let layer_width = nodes.len();
        let offset = (max_width - layer_width) / 2;

        for &node_id in nodes {
            if let Some((_, p)) = positions.get_mut(&node_id) {
                *p += offset;
            }
        }
    }

    positions
}

/// Calculate total bounds of the layout.
#[allow(dead_code)]
pub fn calculate_bounds(
    layouts: &HashMap<NodeId, NodeLayout>,
    config: &LayoutConfig,
) -> (f32, f32) {
    if layouts.is_empty() {
        return (config.padding * 2.0, config.padding * 2.0);
    }

    let max_x = layouts
        .values()
        .map(|l| l.x + l.width)
        .fold(0.0f32, f32::max);
    let max_y = layouts
        .values()
        .map(|l| l.y + l.height)
        .fold(0.0f32, f32::max);

    (max_x + config.padding, max_y + config.padding)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_flowchart() {
        let flowchart = Flowchart::new();
        let config = LayoutConfig::default();
        let layouts = calculate_layout(&flowchart, &config);
        assert!(layouts.is_empty());
    }
}
