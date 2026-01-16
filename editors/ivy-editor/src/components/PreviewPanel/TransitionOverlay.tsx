import { useState, useEffect } from "react";
import type { PlaytestTransition } from "../../types/playtest";

interface Props {
  transition: PlaytestTransition | null;
}

export const TransitionOverlay: React.FC<Props> = ({ transition }) => {
  const [isAnimating, setIsAnimating] = useState(false);
  const [currentTransition, setCurrentTransition] = useState<PlaytestTransition | null>(null);

  useEffect(() => {
    if (transition && transition.type !== "none") {
      setCurrentTransition(transition);
      setIsAnimating(true);

      const timer = setTimeout(() => {
        setIsAnimating(false);
      }, transition.duration * 1000);

      return () => clearTimeout(timer);
    }
  }, [transition]);

  if (!isAnimating || !currentTransition) {
    return null;
  }

  const getAnimationStyle = (): React.CSSProperties => {
    const duration = `${currentTransition.duration}s`;

    switch (currentTransition.type) {
      case "fade":
        return {
          animation: `transition-fade ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "fadewhite":
        return {
          animation: `transition-fade ${duration} ease-out forwards`,
          backgroundColor: "white",
        };
      case "dissolve":
        return {
          animation: `transition-dissolve ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "wipe":
        return {
          animation: `transition-wipe-${currentTransition.direction} ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "slide":
        return {
          animation: `transition-slide-${currentTransition.direction} ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "pixelate":
        return {
          animation: `transition-pixelate ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "iris":
        return {
          animation: `transition-iris ${duration} ease-out forwards`,
          backgroundColor: "black",
        };
      case "blinds":
        return {
          animation: `transition-blinds ${duration} ease-out forwards`,
          backgroundColor: "transparent",
        };
      default:
        return {};
    }
  };

  return (
    <div className="transition-overlay" style={getAnimationStyle()}>
      {currentTransition.type === "blinds" && (
        <div className="blinds-container">
          {Array.from({ length: 10 }).map((_, i) => (
            <div
              key={i}
              className="blind-strip"
              style={{
                animationDelay: `${(i * currentTransition.duration) / 20}s`,
                animationDuration: `${currentTransition.duration}s`,
              }}
            />
          ))}
        </div>
      )}
      <span className="transition-label">{currentTransition.type}</span>
    </div>
  );
};
