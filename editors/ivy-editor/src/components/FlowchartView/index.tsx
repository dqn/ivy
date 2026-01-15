import { useEffect, useState } from "react";
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  type Node,
  type Edge,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { invoke } from "@tauri-apps/api/core";
import type { Scenario } from "../../types/scenario";
import type { FlowchartData, EdgeType } from "../../types/flowchart";
import { StartNode, LabelNode, ChoiceNode, ConditionalNode, EndNode } from "./nodes";
import "./styles.css";

interface Props {
  scenario: Scenario;
  onNodeClick: (scriptIndex: number) => void;
}

const nodeTypes = {
  start: StartNode,
  label: LabelNode,
  choice: ChoiceNode,
  conditional: ConditionalNode,
  end: EndNode,
};

const edgeStyles: Record<EdgeType, React.CSSProperties> = {
  sequential: { stroke: "#666", strokeWidth: 2 },
  jump: { stroke: "#f59e0b", strokeWidth: 2, strokeDasharray: "5,5" },
  choice: { stroke: "#9b59b6", strokeWidth: 2 },
  conditional: { stroke: "#e74c3c", strokeWidth: 2, strokeDasharray: "3,3" },
};

export const FlowchartView: React.FC<Props> = ({ scenario, onNodeClick }) => {
  const [nodes, setNodes] = useState<Node[]>([]);
  const [edges, setEdges] = useState<Edge[]>([]);

  useEffect(() => {
    const loadFlowchart = async () => {
      const data = await invoke<FlowchartData>("get_flowchart", { scenario });

      const flowNodes: Node[] = data.nodes.map((node) => ({
        id: node.id,
        type: node.node_type,
        position: { x: node.x, y: node.y },
        data: {
          label: node.label,
          preview: node.preview,
          scriptIndex: node.script_index,
        },
        style: { width: node.width, height: node.height },
      }));

      const flowEdges: Edge[] = data.edges.map((edge) => ({
        id: edge.id,
        source: edge.source,
        target: edge.target,
        label: edge.label,
        style: edgeStyles[edge.edge_type],
        animated: edge.edge_type === "jump",
      }));

      setNodes(flowNodes);
      setEdges(flowEdges);
    };

    void loadFlowchart();
  }, [scenario]);

  const handleNodeClick = (_: React.MouseEvent, node: Node) => {
    const scriptIndex = node.data.scriptIndex;
    if (typeof scriptIndex === "number") {
      onNodeClick(scriptIndex);
    }
  };

  return (
    <div className="flowchart-view">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        onNodeClick={handleNodeClick}
        fitView
        panOnScroll
        nodesDraggable={false}
        nodesConnectable={false}
      >
        <Background />
        <Controls />
        <MiniMap />
      </ReactFlow>
    </div>
  );
};
