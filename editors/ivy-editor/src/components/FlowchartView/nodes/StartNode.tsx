import { Handle, Position, type Node, type NodeProps } from "@xyflow/react";

type StartNodeData = {
  label: string;
  [key: string]: unknown;
};

type StartNodeType = Node<StartNodeData, "start">;

export const StartNode: React.FC<NodeProps<StartNodeType>> = ({ data }) => {
  return (
    <div className="flowchart-node start-node">
      <div className="node-content">{data.label}</div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
