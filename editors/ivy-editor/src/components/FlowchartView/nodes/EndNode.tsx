import { Handle, Position, type Node, type NodeProps } from "@xyflow/react";

type EndNodeData = {
  label: string;
  [key: string]: unknown;
};

type EndNodeType = Node<EndNodeData, "end">;

export const EndNode: React.FC<NodeProps<EndNodeType>> = ({ data }) => {
  return (
    <div className="flowchart-node end-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-content">{data.label}</div>
    </div>
  );
};
