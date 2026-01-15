use ivy::flowchart::{build_flowchart, calculate_layout, EdgeType, LayoutConfig, NodeType};
use ivy::scenario::Scenario;
use serde::Serialize;

#[derive(Serialize)]
pub struct FlowchartData {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}

#[derive(Serialize)]
pub struct NodeData {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub script_index: usize,
    pub preview: Option<String>,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Serialize)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub label: Option<String>,
}

#[tauri::command]
pub fn get_flowchart(scenario: Scenario) -> FlowchartData {
    let flowchart = build_flowchart(&scenario);
    let config = LayoutConfig::default();
    let layouts = calculate_layout(&flowchart, &config);

    let nodes = flowchart
        .nodes
        .iter()
        .map(|node| {
            let (node_type, label) = match &node.node_type {
                NodeType::Start => ("start".to_string(), "Start".to_string()),
                NodeType::Label { name } => ("label".to_string(), name.clone()),
                NodeType::Choice { options } => {
                    ("choice".to_string(), format!("{} choices", options.len()))
                }
                NodeType::Conditional { var, .. } => {
                    ("conditional".to_string(), format!("if {}", var))
                }
                NodeType::End => ("end".to_string(), "End".to_string()),
            };

            let layout = layouts.get(&node.id).copied().unwrap_or_default();

            NodeData {
                id: format!("node-{}", node.id.0),
                node_type,
                label,
                script_index: node.script_index,
                preview: node.preview.clone(),
                x: layout.x,
                y: layout.y,
                width: layout.width,
                height: layout.height,
            }
        })
        .collect();

    let edges = flowchart
        .edges
        .iter()
        .enumerate()
        .map(|(i, edge)| {
            let edge_type = match edge.edge_type {
                EdgeType::Sequential => "sequential",
                EdgeType::Jump => "jump",
                EdgeType::Choice(_) => "choice",
                EdgeType::Conditional => "conditional",
            };

            EdgeData {
                id: format!("edge-{}", i),
                source: format!("node-{}", edge.from.0),
                target: format!("node-{}", edge.to.0),
                edge_type: edge_type.to_string(),
                label: edge.label.clone(),
            }
        })
        .collect();

    FlowchartData { nodes, edges }
}
