import { useState } from "react";
import type { AssetTree as AssetTreeData } from "../../types/assets";

interface Props {
  node: AssetTreeData;
  selectedPath: string | null;
  unusedAssets: string[];
  onSelect: (path: string) => void;
  depth?: number;
}

export const AssetTree: React.FC<Props> = ({
  node,
  selectedPath,
  unusedAssets,
  onSelect,
  depth = 0,
}) => {
  const [expanded, setExpanded] = useState(depth < 2);

  const isUnused = unusedAssets.includes(node.path);
  const showNode = unusedAssets.length === 0 || isUnused || node.is_dir;

  if (!showNode) {
    return null;
  }

  const getIcon = () => {
    if (node.is_dir) {
      return expanded ? "📂" : "📁";
    }
    switch (node.asset_type) {
      case "image":
        return "🖼️";
      case "audio":
        return "🔊";
      case "video":
        return "🎬";
      case "scenario":
        return "📄";
      default:
        return "📎";
    }
  };

  if (node.is_dir) {
    const visibleChildren = node.children.filter((child) => {
      if (unusedAssets.length === 0) {
        return true;
      }
      if (child.is_dir) {
        return true;
      }
      return unusedAssets.includes(child.path);
    });

    return (
      <div className="tree-node">
        <div
          className="tree-item folder"
          style={{ paddingLeft: depth * 16 }}
          onClick={() => setExpanded(!expanded)}
        >
          <span className="icon">{getIcon()}</span>
          <span className="name">{node.name}</span>
        </div>
        {expanded && (
          <div className="tree-children">
            {visibleChildren.map((child) => (
              <AssetTree
                key={child.path}
                node={child}
                selectedPath={selectedPath}
                unusedAssets={unusedAssets}
                onSelect={onSelect}
                depth={depth + 1}
              />
            ))}
          </div>
        )}
      </div>
    );
  }

  return (
    <div
      className={`tree-item file ${selectedPath === node.path ? "selected" : ""} ${isUnused ? "unused" : ""}`}
      style={{ paddingLeft: depth * 16 }}
      onClick={() => onSelect(node.path)}
    >
      <span className="icon">{getIcon()}</span>
      <span className="name">{node.name}</span>
      {isUnused && <span className="unused-badge">unused</span>}
    </div>
  );
};
