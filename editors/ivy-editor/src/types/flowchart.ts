export type NodeType =
  | "start"
  | "label"
  | "choice"
  | "conditional"
  | "end";

export type EdgeType = "sequential" | "jump" | "choice" | "conditional";

export interface NodeData {
  id: string;
  node_type: NodeType;
  label: string;
  script_index: number;
  preview?: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface EdgeData {
  id: string;
  source: string;
  target: string;
  edge_type: EdgeType;
  label?: string;
}

export interface FlowchartData {
  nodes: NodeData[];
  edges: EdgeData[];
}
