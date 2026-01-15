import { Handle, Position, type Node, type NodeProps } from "@xyflow/react";

type LabelNodeData = {
  label: string;
  preview?: string;
  scriptIndex: number;
  [key: string]: unknown;
};

type LabelNodeType = Node<LabelNodeData, "label">;

export const LabelNode: React.FC<NodeProps<LabelNodeType>> = ({ data }) => {
  return (
    <div className="flowchart-node label-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-label">{data.label}</div>
      {data.preview && <div className="node-preview">{data.preview}</div>}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
