import { useState, useEffect, useCallback, useRef } from "react";

interface Props {
  duration: number;
  onComplete: () => void;
  onSkip: () => void;
}

export const WaitOverlay: React.FC<Props> = ({ duration, onComplete, onSkip }) => {
  const [remaining, setRemaining] = useState(duration);
  const startTimeRef = useRef<number>(Date.now());
  const completedRef = useRef(false);

  useEffect(() => {
    startTimeRef.current = Date.now();
    completedRef.current = false;
    setRemaining(duration);

    const interval = setInterval(() => {
      const elapsed = (Date.now() - startTimeRef.current) / 1000;
      const newRemaining = Math.max(0, duration - elapsed);
      setRemaining(newRemaining);

      if (newRemaining <= 0 && !completedRef.current) {
        completedRef.current = true;
        clearInterval(interval);
        onComplete();
      }
    }, 50);

    return () => clearInterval(interval);
  }, [duration, onComplete]);

  const handleSkip = useCallback(() => {
    if (!completedRef.current) {
      completedRef.current = true;
      onSkip();
    }
  }, [onSkip]);

  const progress = ((duration - remaining) / duration) * 100;

  return (
    <div className="wait-overlay">
      <div className="wait-content">
        <div className="wait-timer">
          <span className="wait-label">Wait</span>
          <span className="wait-time">{remaining.toFixed(1)}s</span>
        </div>
        <div className="wait-progress-bar">
          <div
            className="wait-progress-fill"
            style={{ width: `${progress}%` }}
          />
        </div>
        <button className="wait-skip-button" onClick={handleSkip}>
          Skip
        </button>
      </div>
    </div>
  );
};
