import { useState, useEffect, useRef } from "react";

interface Props {
  speaker: string | null;
  text: string | null;
  typewriterEnabled?: boolean;
  typewriterSpeed?: number; // characters per second
  onComplete?: () => void;
}

export const TextBox: React.FC<Props> = ({
  speaker,
  text,
  typewriterEnabled = false,
  typewriterSpeed = 30,
  onComplete,
}) => {
  const [displayedText, setDisplayedText] = useState("");
  const [isComplete, setIsComplete] = useState(true);
  const timerRef = useRef<number | null>(null);
  const fullText = text ?? "";

  // Reset and start typewriter when text changes
  useEffect(() => {
    if (!typewriterEnabled || !fullText) {
      setDisplayedText(fullText);
      setIsComplete(true);
      return;
    }

    // Reset state
    setDisplayedText("");
    setIsComplete(false);

    let currentIndex = 0;
    const interval = 1000 / typewriterSpeed;

    const tick = () => {
      currentIndex++;
      setDisplayedText(fullText.slice(0, currentIndex));

      if (currentIndex >= fullText.length) {
        setIsComplete(true);
        onComplete?.();
      } else {
        timerRef.current = window.setTimeout(tick, interval);
      }
    };

    timerRef.current = window.setTimeout(tick, interval);

    return () => {
      if (timerRef.current !== null) {
        clearTimeout(timerRef.current);
      }
    };
  }, [fullText, typewriterEnabled, typewriterSpeed, onComplete]);

  const handleClick = () => {
    if (!isComplete && typewriterEnabled) {
      // Skip to full text
      if (timerRef.current !== null) {
        clearTimeout(timerRef.current);
      }
      setDisplayedText(fullText);
      setIsComplete(true);
      onComplete?.();
    }
  };

  return (
    <div
      className={`text-box ${typewriterEnabled && !isComplete ? "typing" : ""}`}
      onClick={handleClick}
    >
      {speaker && <div className="speaker-name">{speaker}</div>}
      <div className="dialogue-text">
        {typewriterEnabled ? displayedText : fullText}
        {typewriterEnabled && !isComplete && (
          <span className="typing-cursor">▌</span>
        )}
      </div>
    </div>
  );
};
