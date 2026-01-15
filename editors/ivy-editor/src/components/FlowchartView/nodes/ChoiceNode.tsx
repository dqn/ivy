import { Handle, Position, type Node, type NodeProps } from "@xyflow/react";

type ChoiceNodeData = {
  label: string;
  preview?: string;
  scriptIndex: number;
  [key: string]: unknown;
};

type ChoiceNodeType = Node<ChoiceNodeData, "choice">;

export const ChoiceNode: React.FC<NodeProps<ChoiceNodeType>> = ({ data }) => {
  return (
    <div className="flowchart-node choice-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-label">{data.label}</div>
      {data.preview && <div className="node-preview">{data.preview}</div>}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
