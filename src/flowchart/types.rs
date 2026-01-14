use std::collections::HashMap;

/// Unique identifier for a flowchart node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Type of flowchart node.
#[derive(Debug, Clone)]
pub enum NodeType {
    /// Start of scenario.
    Start,
    /// Label definition (story checkpoint).
    Label { name: String },
    /// Choice point with multiple options.
    Choice { options: Vec<String> },
    /// Conditional branch (if statement).
    Conditional {
        var: String,
        #[allow(dead_code)]
        value: String,
    },
    /// End of scenario.
    End,
}

/// A node in the flowchart.
#[derive(Debug, Clone)]
pub struct FlowchartNode {
    pub id: NodeId,
    pub node_type: NodeType,
    /// Script index this node corresponds to.
    pub script_index: usize,
    /// Preview text for this node.
    pub preview: Option<String>,
}

/// Type of edge connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Normal sequential flow.
    Sequential,
    /// Unconditional jump.
    Jump,
    /// Choice selection.
    Choice(usize),
    /// Conditional jump (if true).
    Conditional,
}

/// An edge connecting two nodes.
#[derive(Debug, Clone)]
pub struct FlowchartEdge {
    pub from: NodeId,
    pub to: NodeId,
    pub edge_type: EdgeType,
    /// Label for the edge (e.g., choice text).
    pub label: Option<String>,
}

/// Complete flowchart graph.
#[derive(Debug, Clone)]
pub struct Flowchart {
    pub nodes: Vec<FlowchartNode>,
    pub edges: Vec<FlowchartEdge>,
    /// Mapping from script index to node ID.
    pub index_to_node: HashMap<usize, NodeId>,
}

impl Flowchart {
    /// Create a new empty flowchart.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            index_to_node: HashMap::new(),
        }
    }

    /// Get a node by its ID.
    #[allow(dead_code)]
    pub fn get_node(&self, id: NodeId) -> Option<&FlowchartNode> {
        self.nodes.get(id.0)
    }

    /// Get edges originating from a node.
    #[allow(dead_code)]
    pub fn edges_from(&self, id: NodeId) -> Vec<&FlowchartEdge> {
        self.edges.iter().filter(|e| e.from == id).collect()
    }

    /// Get edges pointing to a node.
    #[allow(dead_code)]
    pub fn edges_to(&self, id: NodeId) -> Vec<&FlowchartEdge> {
        self.edges.iter().filter(|e| e.to == id).collect()
    }
}

impl Default for Flowchart {
    fn default() -> Self {
        Self::new()
    }
}
