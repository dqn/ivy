import type { AssetInfo } from "../../types/assets";

interface Props {
  info: AssetInfo;
  onUse: () => void;
  onFindUsages: () => void;
}

function formatSize(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`;
  }
  if (bytes < 1024 * 1024) {
    return `${(bytes / 1024).toFixed(1)} KB`;
  }
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}

function formatDate(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleDateString();
}

export const AssetInfoPanel: React.FC<Props> = ({
  info,
  onUse,
  onFindUsages,
}) => {
  return (
    <div className="asset-info">
      <h4>{info.name}</h4>
      <dl>
        <dt>Type</dt>
        <dd>{info.asset_type}</dd>
        <dt>Size</dt>
        <dd>{formatSize(info.size)}</dd>
        <dt>Modified</dt>
        <dd>{formatDate(info.modified)}</dd>
        <dt>Path</dt>
        <dd className="path">{info.relative_path}</dd>
      </dl>
      <div className="asset-actions">
        <button onClick={onUse}>Use in Command</button>
        <button onClick={onFindUsages}>Find Usages</button>
      </div>
    </div>
  );
};
