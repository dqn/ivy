import { Handle, Position, type Node, type NodeProps } from "@xyflow/react";

type ConditionalNodeData = {
  label: string;
  preview?: string;
  scriptIndex: number;
  [key: string]: unknown;
};

type ConditionalNodeType = Node<ConditionalNodeData, "conditional">;

export const ConditionalNode: React.FC<NodeProps<ConditionalNodeType>> = ({ data }) => {
  return (
    <div className="flowchart-node conditional-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-label">{data.label}</div>
      {data.preview && <div className="node-preview">{data.preview}</div>}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
