import { useTranslation } from "react-i18next";

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
  const { t } = useTranslation();

  return (
    <div className="playtest-controls">
      <div className="playtest-controls-row">
        <button
          className="playtest-button rollback"
          onClick={onRollback}
          disabled={!canRollback}
          title={t("playtestControls.rollbackTitle")}
        >
          ⏪ {t("playtestControls.back")}
        </button>
        <button
          className="playtest-button advance"
          onClick={onAdvance}
          disabled={isEnded || isBlocked}
          title={t("playtestControls.advanceTitle")}
        >
          ⏩ {t("playtestControls.nextArrow")}
        </button>
      </div>
      <div className="playtest-controls-row">
        <button
          className={`playtest-button mode-toggle ${isAutoMode ? "active" : ""}`}
          onClick={onToggleAutoMode}
          title={t("playtestControls.autoModeTitle")}
        >
          🔄 {t("playtestControls.auto")}
        </button>
        <button
          className={`playtest-button mode-toggle ${isSkipMode ? "active" : ""}`}
          onClick={onToggleSkipMode}
          title={t("playtestControls.skipModeTitle")}
        >
          ⏭️ {t("playtestControls.skip")}
        </button>
        <button
          className="playtest-button restart"
          onClick={onRestart}
          title={t("playtestControls.restartTitle")}
        >
          🔁 {t("playtestControls.restart")}
        </button>
      </div>
      <div className="playtest-controls-row">
        <span className="playtest-history-count">
          📜 {t("playtestControls.history", { count: historyCount })}
        </span>
      </div>
      <div className="playtest-shortcuts-hint">
        {t("playtestControls.shortcutsHint")}
      </div>
    </div>
  );
};
