import { useEffect } from "react";
import { useSaveDataValidator } from "../../hooks/useSaveDataValidator";
import { SaveDataList } from "./SaveDataList";
import { ValidationResultPanel } from "./ValidationResultPanel";
import "./styles.css";

interface Props {
  baseDir: string | null;
}

export const SaveDataValidator: React.FC<Props> = ({ baseDir }) => {
  const {
    saveDataList,
    selectedSaveData,
    validationResult,
    isLoading,
    error,
    loadSaveDataList,
    selectSaveData,
    openSaveFile,
    clearSelection,
  } = useSaveDataValidator();

  // Load save data list when baseDir changes
  useEffect(() => {
    if (baseDir) {
      void loadSaveDataList(baseDir);
    }
  }, [baseDir, loadSaveDataList]);

  if (!baseDir) {
    return null;
  }

  return (
    <div className="save-data-validator">
      <div className="validator-header">
        <span className="validator-title">Save Data Validator</span>
        <div className="validator-actions">
          <button onClick={() => void loadSaveDataList(baseDir)} title="Refresh">
            Refresh
          </button>
          <button onClick={() => void openSaveFile(baseDir)} title="Open file">
            Open
          </button>
          {selectedSaveData && (
            <button onClick={clearSelection} title="Clear selection">
              Clear
            </button>
          )}
        </div>
      </div>

      <div className="validator-content">
        {isLoading ? (
          <div className="loading-state">Loading...</div>
        ) : error ? (
          <div className="error-state">{error}</div>
        ) : (
          <>
            <SaveDataList
              saveDataList={saveDataList}
              selectedSaveData={selectedSaveData}
              onSelect={(saveData) => void selectSaveData(saveData)}
            />
            {validationResult && (
              <ValidationResultPanel result={validationResult} />
            )}
          </>
        )}
      </div>
    </div>
  );
};
