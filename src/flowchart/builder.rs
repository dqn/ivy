use std::collections::HashMap;

use crate::i18n::LocalizedString;
use crate::scenario::Scenario;

use super::types::{EdgeType, Flowchart, FlowchartEdge, FlowchartNode, NodeId, NodeType};

/// Get the default text from a LocalizedString.
fn get_default_text(s: &LocalizedString) -> String {
    match s {
        LocalizedString::Plain(text) => text.clone(),
        LocalizedString::Localized(map) => {
            // Try English first, then any available language
            map.get("en")
                .or_else(|| map.values().next())
                .cloned()
                .unwrap_or_default()
        }
        LocalizedString::Key(key) => format!("@{}", key),
    }
}

/// Build flowchart from scenario.
pub fn build_flowchart(scenario: &Scenario) -> Flowchart {
    let mut flowchart = Flowchart::new();

    if scenario.script.is_empty() {
        return flowchart;
    }

    // First pass: collect all label positions
    let mut label_to_index: HashMap<String, usize> = HashMap::new();
    for (i, cmd) in scenario.script.iter().enumerate() {
        if let Some(label) = &cmd.label {
            label_to_index.insert(label.clone(), i);
        }
    }

    // Second pass: identify significant nodes (labels, choices, conditionals)
    let mut significant_indices: Vec<usize> = Vec::new();
    significant_indices.push(0); // Start node

    for (i, cmd) in scenario.script.iter().enumerate() {
        // Label nodes
        if cmd.label.is_some() && i != 0 {
            significant_indices.push(i);
        }
        // Choice nodes
        if cmd.choices.is_some() && !significant_indices.contains(&i) {
            significant_indices.push(i);
        }
        // Conditional nodes
        if cmd.if_cond.is_some() && !significant_indices.contains(&i) {
            significant_indices.push(i);
        }
    }

    // Add end node if the script doesn't jump away at the end
    let last_idx = scenario.script.len() - 1;
    let last_cmd = &scenario.script[last_idx];
    if last_cmd.jump.is_none()
        && last_cmd.choices.is_none()
        && !significant_indices.contains(&last_idx)
    {
        significant_indices.push(last_idx);
    }

    significant_indices.sort();
    significant_indices.dedup();

    // Create nodes for significant indices
    for &idx in &significant_indices {
        let cmd = &scenario.script[idx];
        let node_id = NodeId(flowchart.nodes.len());

        let node_type = if idx == 0 {
            NodeType::Start
        } else if let Some(label) = &cmd.label {
            NodeType::Label {
                name: label.clone(),
            }
        } else if let Some(choices) = &cmd.choices {
            let options: Vec<String> = choices.iter().map(|c| get_default_text(&c.label)).collect();
            NodeType::Choice { options }
        } else if let Some(if_cond) = &cmd.if_cond {
            NodeType::Conditional {
                var: if_cond.var.clone(),
                value: format!("{:?}", if_cond.is),
            }
        } else if idx == last_idx && cmd.jump.is_none() {
            NodeType::End
        } else {
            NodeType::Label {
                name: format!("#{}", idx),
            }
        };

        // Preview text
        let preview = cmd.text.as_ref().map(|t| {
            let text = get_default_text(t);
            if text.len() > 30 {
                format!("{}...", &text[..30])
            } else {
                text
            }
        });

        let node = FlowchartNode {
            id: node_id,
            node_type,
            script_index: idx,
            preview,
        };

        flowchart.index_to_node.insert(idx, node_id);
        flowchart.nodes.push(node);
    }

    // Third pass: create edges
    for (i, &idx) in significant_indices.iter().enumerate() {
        let cmd = &scenario.script[idx];
        let from_id = flowchart.index_to_node[&idx];

        // Choice edges
        if let Some(choices) = &cmd.choices {
            for (choice_idx, choice) in choices.iter().enumerate() {
                if let Some(&target_idx) = label_to_index.get(&choice.jump) {
                    // Find the node at or after target_idx
                    if let Some(&to_id) = flowchart.index_to_node.get(&target_idx) {
                        flowchart.edges.push(FlowchartEdge {
                            from: from_id,
                            to: to_id,
                            edge_type: EdgeType::Choice(choice_idx),
                            label: Some(get_default_text(&choice.label)),
                        });
                    } else if let Some(&to_id) =
                        find_nearest_node(&flowchart, &significant_indices, target_idx)
                    {
                        flowchart.edges.push(FlowchartEdge {
                            from: from_id,
                            to: to_id,
                            edge_type: EdgeType::Choice(choice_idx),
                            label: Some(get_default_text(&choice.label)),
                        });
                    }
                }
            }
            continue; // Choices don't have sequential flow
        }

        // Jump edge
        if let Some(jump) = &cmd.jump {
            if let Some(&target_idx) = label_to_index.get(jump) {
                if let Some(&to_id) = flowchart.index_to_node.get(&target_idx) {
                    flowchart.edges.push(FlowchartEdge {
                        from: from_id,
                        to: to_id,
                        edge_type: EdgeType::Jump,
                        label: None,
                    });
                } else if let Some(&to_id) =
                    find_nearest_node(&flowchart, &significant_indices, target_idx)
                {
                    flowchart.edges.push(FlowchartEdge {
                        from: from_id,
                        to: to_id,
                        edge_type: EdgeType::Jump,
                        label: None,
                    });
                }
            }
            continue; // Jump doesn't have sequential flow
        }

        // Conditional edge
        if let Some(if_cond) = &cmd.if_cond {
            if let Some(&target_idx) = label_to_index.get(&if_cond.jump) {
                if let Some(&to_id) = flowchart.index_to_node.get(&target_idx) {
                    flowchart.edges.push(FlowchartEdge {
                        from: from_id,
                        to: to_id,
                        edge_type: EdgeType::Conditional,
                        label: Some(format!("{} == {:?}", if_cond.var, if_cond.is)),
                    });
                } else if let Some(&to_id) =
                    find_nearest_node(&flowchart, &significant_indices, target_idx)
                {
                    flowchart.edges.push(FlowchartEdge {
                        from: from_id,
                        to: to_id,
                        edge_type: EdgeType::Conditional,
                        label: Some(format!("{} == {:?}", if_cond.var, if_cond.is)),
                    });
                }
            }
            // Conditional also has sequential flow (else branch)
        }

        // Sequential edge to next significant node
        if i + 1 < significant_indices.len() {
            let next_idx = significant_indices[i + 1];
            let to_id = flowchart.index_to_node[&next_idx];
            flowchart.edges.push(FlowchartEdge {
                from: from_id,
                to: to_id,
                edge_type: EdgeType::Sequential,
                label: None,
            });
        }
    }

    flowchart
}

/// Find the nearest significant node at or after the given index.
fn find_nearest_node<'a>(
    flowchart: &'a Flowchart,
    significant_indices: &[usize],
    target_idx: usize,
) -> Option<&'a NodeId> {
    // First try exact match
    if let Some(id) = flowchart.index_to_node.get(&target_idx) {
        return Some(id);
    }

    // Find nearest significant index at or after target
    for &idx in significant_indices {
        if idx >= target_idx {
            if let Some(id) = flowchart.index_to_node.get(&idx) {
                return Some(id);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_scenario() -> Scenario {
        // Minimal scenario for testing
        Scenario {
            title: "Test".to_string(),
            chapters: vec![],
            script: vec![],
        }
    }

    #[test]
    fn test_empty_scenario() {
        let scenario = create_test_scenario();
        let flowchart = build_flowchart(&scenario);
        assert!(flowchart.nodes.is_empty());
        assert!(flowchart.edges.is_empty());
    }
}
