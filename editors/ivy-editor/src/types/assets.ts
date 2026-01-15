export interface AssetInfo {
  path: string;
  relative_path: string;
  name: string;
  asset_type: string;
  size: number;
  modified: number;
}

export interface AssetTree {
  name: string;
  path: string;
  is_dir: boolean;
  children: AssetTree[];
  asset_type: string | null;
}
