interface Props {
  canRollback: boolean;
  isEnded: boolean;
  isBlocked: boolean;
  historyCount: number;
  isAutoMode: boolean;
  isSkipMode: boolean;
  onAdvance: () => void;
  onRollback: () => void;
  onRestart: () => void;
  onToggleAutoMode: () => void;
  onToggleSkipMode: () => void;
}

export const PlaytestControls: React.FC<Props> = ({
  canRollback,
  isEnded,
  isBlocked,
  historyCount,
  isAutoMode,
  isSkipMode,
  onAdvance,
  onRollback,
  onRestart,
  onToggleAutoMode,
  onToggleSkipMode,
}) => {
  return (
    <div className="playtest-controls">
      <div className="playtest-controls-row">
        <button
          className="playtest-button rollback"
          onClick={onRollback}
          disabled={!canRollback}
          title="Rollback (Backspace / ↑)"
        >
          ← Back
        </button>
        <button
          className="playtest-button advance"
          onClick={onAdvance}
          disabled={isEnded || isBlocked}
          title="Advance (Enter / Space / →)"
        >
          Next →
        </button>
      </div>
      <div className="playtest-controls-row">
        <button
          className={`playtest-button mode-toggle ${isAutoMode ? "active" : ""}`}
          onClick={onToggleAutoMode}
          title="Auto Mode (A)"
        >
          Auto
        </button>
        <button
          className={`playtest-button mode-toggle ${isSkipMode ? "active" : ""}`}
          onClick={onToggleSkipMode}
          title="Skip Mode (S)"
        >
          Skip
        </button>
        <button
          className="playtest-button restart"
          onClick={onRestart}
          title="Restart (Ctrl+R)"
        >
          Restart
        </button>
      </div>
      <div className="playtest-controls-row">
        <span className="playtest-history-count">
          History: {historyCount}
        </span>
      </div>
      <div className="playtest-shortcuts-hint">
        Keys: Enter/Space = Next, Backspace = Back, A = Auto, S = Skip, 1-9 = Choice
      </div>
    </div>
  );
};
