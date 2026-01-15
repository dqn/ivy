import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AssetTree } from "./AssetTree";
import { AssetPreview } from "./AssetPreview";
import { AssetInfoPanel } from "./AssetInfoPanel";
import type { AssetTree as AssetTreeData, AssetInfo } from "../../types/assets";
import type { Scenario } from "../../types/scenario";
import "./styles.css";

interface Props {
  baseDir: string | null;
  scenario: Scenario | null;
  onSelectAsset: (path: string) => void;
  onShowUsages: (indices: number[]) => void;
}

export const AssetBrowser: React.FC<Props> = ({
  baseDir,
  scenario,
  onSelectAsset,
  onShowUsages,
}) => {
  const [tree, setTree] = useState<AssetTreeData | null>(null);
  const [selectedPath, setSelectedPath] = useState<string | null>(null);
  const [assetInfo, setAssetInfo] = useState<AssetInfo | null>(null);
  const [unusedAssets, setUnusedAssets] = useState<string[]>([]);
  const [showUnusedOnly, setShowUnusedOnly] = useState(false);

  useEffect(() => {
    if (!baseDir) {
      setTree(null);
      return;
    }

    const loadTree = async () => {
      try {
        const data = await invoke<AssetTreeData>("list_assets", { baseDir });
        setTree(data);
      } catch (err) {
        console.error("Failed to load assets:", err);
      }
    };

    void loadTree();
  }, [baseDir]);

  const handleSelect = useCallback(
    async (path: string) => {
      setSelectedPath(path);

      if (baseDir) {
        try {
          const info = await invoke<AssetInfo>("get_asset_info", {
            baseDir,
            relativePath: path,
          });
          setAssetInfo(info);
        } catch {
          setAssetInfo(null);
        }
      }
    },
    [baseDir],
  );

  const handleUse = useCallback(() => {
    if (selectedPath) {
      onSelectAsset(selectedPath);
    }
  }, [selectedPath, onSelectAsset]);

  const handleFindUsages = useCallback(async () => {
    if (!selectedPath || !scenario) {
      return;
    }

    try {
      const usages = await invoke<number[]>("find_asset_usages", {
        scenario,
        assetPath: selectedPath,
      });
      onShowUsages(usages);
    } catch (err) {
      console.error("Failed to find usages:", err);
    }
  }, [selectedPath, scenario, onShowUsages]);

  const handleFindUnused = useCallback(async () => {
    if (!baseDir || !scenario) {
      return;
    }

    try {
      const unused = await invoke<string[]>("find_unused_assets", {
        baseDir,
        scenario,
      });
      setUnusedAssets(unused);
      setShowUnusedOnly(true);
    } catch (err) {
      console.error("Failed to find unused assets:", err);
    }
  }, [baseDir, scenario]);

  const handleRefresh = useCallback(async () => {
    if (!baseDir) {
      return;
    }

    try {
      const data = await invoke<AssetTreeData>("list_assets", { baseDir });
      setTree(data);
    } catch (err) {
      console.error("Failed to refresh:", err);
    }
  }, [baseDir]);

  if (!baseDir) {
    return (
      <div className="asset-browser empty">
        <p>Open a scenario to browse assets</p>
      </div>
    );
  }

  return (
    <div className="asset-browser">
      <div className="asset-browser-toolbar">
        <button onClick={() => void handleRefresh()}>Refresh</button>
        <button onClick={() => void handleFindUnused()}>Find Unused</button>
        {showUnusedOnly && (
          <button onClick={() => setShowUnusedOnly(false)}>Show All</button>
        )}
      </div>

      <div className="asset-browser-content">
        <div className="asset-tree-panel">
          {tree && (
            <AssetTree
              node={tree}
              selectedPath={selectedPath}
              unusedAssets={showUnusedOnly ? unusedAssets : []}
              onSelect={(path) => void handleSelect(path)}
            />
          )}
        </div>

        <div className="asset-detail-panel">
          {selectedPath && assetInfo && (
            <>
              <AssetPreview
                baseDir={baseDir}
                path={selectedPath}
                type={assetInfo.asset_type}
              />
              <AssetInfoPanel
                info={assetInfo}
                onUse={handleUse}
                onFindUsages={() => void handleFindUsages()}
              />
            </>
          )}
        </div>
      </div>
    </div>
  );
};
