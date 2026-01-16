import type { SaveDataInfo } from "../../types/savedata";

interface Props {
  saveDataList: SaveDataInfo[];
  selectedSaveData: SaveDataInfo | null;
  onSelect: (saveData: SaveDataInfo) => void;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) {
    return `${bytes} B`;
  } else if (bytes < 1024 * 1024) {
    return `${(bytes / 1024).toFixed(1)} KB`;
  } else {
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
}

export const SaveDataList: React.FC<Props> = ({
  saveDataList,
  selectedSaveData,
  onSelect,
}) => {
  if (saveDataList.length === 0) {
    return (
      <div className="empty-message">
        No save data found in saves/ directory
      </div>
    );
  }

  return (
    <div className="save-data-list">
      {saveDataList.map((saveData) => (
        <div
          key={saveData.file_path}
          className={`save-data-item ${
            selectedSaveData?.file_path === saveData.file_path ? "selected" : ""
          }`}
          onClick={() => onSelect(saveData)}
          title={saveData.file_path}
        >
          {saveData.slot !== null && (
            <span className="save-slot-badge">Slot {saveData.slot}</span>
          )}
          <span className="save-file-name">{saveData.file_name}</span>
          <span className="save-timestamp">{saveData.formatted_time}</span>
          <span className="save-size">{formatBytes(saveData.size_bytes)}</span>
        </div>
      ))}
    </div>
  );
};
