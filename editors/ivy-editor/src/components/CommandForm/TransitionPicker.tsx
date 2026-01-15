import { useState } from "react";
import type {
  Transition,
  TransitionType,
  TransitionDirection,
  Easing,
} from "../../types/scenario";

interface TransitionPickerProps {
  value: Transition | undefined;
  onChange: (value: Transition | undefined) => void;
}

const TRANSITION_TYPES: { value: TransitionType; label: string; description: string }[] = [
  { value: "none", label: "None", description: "No transition effect" },
  { value: "fade", label: "Fade", description: "Fade to black, then fade in" },
  { value: "fade_white", label: "Fade White", description: "Fade to white, then fade in" },
  { value: "dissolve", label: "Dissolve", description: "Cross-fade between scenes" },
  { value: "wipe", label: "Wipe", description: "Wipe from one side to another" },
  { value: "slide", label: "Slide", description: "Slide the new scene in" },
  { value: "pixelate", label: "Pixelate", description: "Pixelation effect" },
  { value: "iris", label: "Iris", description: "Circular iris transition" },
  { value: "blinds", label: "Blinds", description: "Venetian blinds effect" },
];

const DIRECTIONS: { value: TransitionDirection; label: string; forTypes: TransitionType[] }[] = [
  { value: "left_to_right", label: "Left to Right", forTypes: ["wipe", "blinds"] },
  { value: "right_to_left", label: "Right to Left", forTypes: ["wipe", "blinds"] },
  { value: "top_to_bottom", label: "Top to Bottom", forTypes: ["wipe", "blinds"] },
  { value: "bottom_to_top", label: "Bottom to Top", forTypes: ["wipe", "blinds"] },
  { value: "left", label: "Left", forTypes: ["slide"] },
  { value: "right", label: "Right", forTypes: ["slide"] },
  { value: "up", label: "Up", forTypes: ["slide"] },
  { value: "down", label: "Down", forTypes: ["slide"] },
  { value: "open", label: "Open", forTypes: ["iris"] },
  { value: "close", label: "Close", forTypes: ["iris"] },
  { value: "horizontal", label: "Horizontal", forTypes: ["blinds"] },
  { value: "vertical", label: "Vertical", forTypes: ["blinds"] },
];

const EASINGS: { value: Easing; label: string }[] = [
  { value: "linear", label: "Linear" },
  { value: "ease_in", label: "Ease In" },
  { value: "ease_out", label: "Ease Out" },
  { value: "ease_in_out", label: "Ease In/Out" },
  { value: "ease_in_quad", label: "Ease In (Quad)" },
  { value: "ease_out_quad", label: "Ease Out (Quad)" },
  { value: "ease_in_out_quad", label: "Ease In/Out (Quad)" },
  { value: "ease_in_cubic", label: "Ease In (Cubic)" },
  { value: "ease_out_cubic", label: "Ease Out (Cubic)" },
  { value: "ease_in_out_cubic", label: "Ease In/Out (Cubic)" },
  { value: "ease_in_back", label: "Ease In (Back)" },
  { value: "ease_out_back", label: "Ease Out (Back)" },
  { value: "ease_in_out_back", label: "Ease In/Out (Back)" },
  { value: "ease_out_bounce", label: "Ease Out (Bounce)" },
];

function getTransitionPreviewStyle(type: TransitionType): React.CSSProperties {
  const base: React.CSSProperties = {
    width: "100%",
    height: "100%",
    position: "absolute",
    top: 0,
    left: 0,
  };

  switch (type) {
    case "fade":
      return { ...base, background: "linear-gradient(to right, #333, #000, #333)" };
    case "fade_white":
      return { ...base, background: "linear-gradient(to right, #ccc, #fff, #ccc)" };
    case "dissolve":
      return { ...base, background: "linear-gradient(135deg, #666 0%, #333 50%, #666 100%)" };
    case "wipe":
      return { ...base, background: "linear-gradient(to right, var(--accent-color) 50%, transparent 50%)" };
    case "slide":
      return { ...base, background: "linear-gradient(to right, var(--accent-color), var(--bg-tertiary))" };
    case "pixelate":
      return {
        ...base,
        backgroundImage: `
          linear-gradient(45deg, #666 25%, transparent 25%),
          linear-gradient(-45deg, #666 25%, transparent 25%),
          linear-gradient(45deg, transparent 75%, #666 75%),
          linear-gradient(-45deg, transparent 75%, #666 75%)
        `,
        backgroundSize: "8px 8px",
        backgroundPosition: "0 0, 0 4px, 4px -4px, -4px 0px",
      };
    case "iris":
      return {
        ...base,
        background: "radial-gradient(circle, var(--accent-color) 30%, transparent 30%)",
      };
    case "blinds":
      return {
        ...base,
        backgroundImage: "repeating-linear-gradient(90deg, var(--accent-color) 0px, var(--accent-color) 4px, transparent 4px, transparent 8px)",
      };
    default:
      return { ...base, background: "var(--bg-tertiary)" };
  }
}

export const TransitionPicker: React.FC<TransitionPickerProps> = ({
  value,
  onChange,
}) => {
  const [isExpanded, setIsExpanded] = useState(!!value?.type && value.type !== "none");

  const currentType = value?.type || "none";
  const availableDirections = DIRECTIONS.filter((d) =>
    d.forTypes.includes(currentType)
  );
  const showDirection = availableDirections.length > 0;
  const showBlindsCount = currentType === "blinds";
  const showPixelSize = currentType === "pixelate";

  const updateTransition = (updates: Partial<Transition>) => {
    const newValue = { ...value, ...updates };
    if (newValue.type === "none" || !newValue.type) {
      onChange(undefined);
      return;
    }
    onChange(newValue);
  };

  const handleTypeChange = (type: TransitionType) => {
    if (type === "none") {
      onChange(undefined);
      setIsExpanded(false);
    } else {
      const defaultDuration = 0.5;
      onChange({ type, duration: defaultDuration });
      setIsExpanded(true);
    }
  };

  return (
    <div className="transition-picker">
      <label>Transition</label>

      {/* Type Selector Grid */}
      <div className="transition-type-grid">
        {TRANSITION_TYPES.map((t) => (
          <button
            key={t.value}
            type="button"
            className={`transition-type-button ${currentType === t.value ? "selected" : ""}`}
            onClick={() => handleTypeChange(t.value)}
            title={t.description}
          >
            <div className="transition-preview">
              <div style={getTransitionPreviewStyle(t.value)} />
            </div>
            <span className="transition-label">{t.label}</span>
          </button>
        ))}
      </div>

      {/* Options Panel */}
      {isExpanded && currentType !== "none" && (
        <div className="transition-options">
          {/* Duration */}
          <div className="transition-option-row">
            <label>Duration</label>
            <div className="duration-input">
              <input
                type="range"
                min="0.1"
                max="3"
                step="0.1"
                value={value?.duration ?? 0.5}
                onChange={(e) => {
                  updateTransition({ duration: parseFloat(e.target.value) });
                }}
              />
              <span className="duration-value">{(value?.duration ?? 0.5).toFixed(1)}s</span>
            </div>
          </div>

          {/* Direction */}
          {showDirection && (
            <div className="transition-option-row">
              <label>Direction</label>
              <select
                value={value?.direction || availableDirections[0]?.value || ""}
                onChange={(e) => {
                  updateTransition({ direction: e.target.value as TransitionDirection });
                }}
              >
                {availableDirections.map((d) => (
                  <option key={d.value} value={d.value}>
                    {d.label}
                  </option>
                ))}
              </select>
            </div>
          )}

          {/* Easing */}
          <div className="transition-option-row">
            <label>Easing</label>
            <select
              value={value?.easing || "ease_out"}
              onChange={(e) => {
                updateTransition({ easing: e.target.value as Easing });
              }}
            >
              {EASINGS.map((e) => (
                <option key={e.value} value={e.value}>
                  {e.label}
                </option>
              ))}
            </select>
          </div>

          {/* Blinds Count */}
          {showBlindsCount && (
            <div className="transition-option-row">
              <label>Blinds Count</label>
              <div className="duration-input">
                <input
                  type="range"
                  min="4"
                  max="20"
                  step="1"
                  value={value?.blinds_count ?? 10}
                  onChange={(e) => {
                    updateTransition({ blinds_count: parseInt(e.target.value, 10) });
                  }}
                />
                <span className="duration-value">{value?.blinds_count ?? 10}</span>
              </div>
            </div>
          )}

          {/* Pixel Size */}
          {showPixelSize && (
            <div className="transition-option-row">
              <label>Max Pixel Size</label>
              <div className="duration-input">
                <input
                  type="range"
                  min="4"
                  max="64"
                  step="4"
                  value={value?.max_pixel_size ?? 32}
                  onChange={(e) => {
                    updateTransition({ max_pixel_size: parseInt(e.target.value, 10) });
                  }}
                />
                <span className="duration-value">{value?.max_pixel_size ?? 32}px</span>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
