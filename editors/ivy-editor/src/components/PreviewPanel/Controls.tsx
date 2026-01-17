import { useTranslation } from "react-i18next";

interface Props {
  currentIndex: number;
  totalCommands: number;
  onPrev: () => void;
  onNext: () => void;
}

export const Controls: React.FC<Props> = ({
  currentIndex,
  totalCommands,
  onPrev,
  onNext,
}) => {
  const { t } = useTranslation();

  return (
    <div className="preview-controls">
      <button onClick={onPrev} disabled={currentIndex <= 0}>
        ◀ {t("previewControls.prev")}
      </button>
      <span className="position-indicator">
        {currentIndex + 1} / {totalCommands}
      </span>
      <button onClick={onNext} disabled={currentIndex >= totalCommands - 1}>
        {t("previewControls.next")} ▶
      </button>
    </div>
  );
};
