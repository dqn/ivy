import { useState } from "react";
import type { CameraCommand, Easing } from "../../types/scenario";

interface CameraPickerProps {
  value: CameraCommand | undefined;
  onChange: (value: CameraCommand | undefined) => void;
}

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

const FOCUS_POINTS = [
  { value: "", label: "Default" },
  { value: "center", label: "Center" },
  { value: "left", label: "Left" },
  { value: "right", label: "Right" },
  { value: "top", label: "Top" },
  { value: "bottom", label: "Bottom" },
];

export const CameraPicker: React.FC<CameraPickerProps> = ({
  value,
  onChange,
}) => {
  const [isEnabled, setIsEnabled] = useState(!!value);

  const updateCamera = (updates: Partial<CameraCommand>) => {
    const newValue = { ...value, ...updates };
    // Clean up empty pan object
    if (newValue.pan && newValue.pan.x === 0 && newValue.pan.y === 0) {
      delete newValue.pan;
    }
    // Clean up default values
    if (newValue.zoom === 1) {
      delete newValue.zoom;
    }
    if (newValue.tilt === 0) {
      delete newValue.tilt;
    }
    // If all values are default, clear the camera
    const hasValues =
      newValue.pan ||
      newValue.zoom !== undefined ||
      newValue.tilt !== undefined ||
      newValue.focus;
    if (!hasValues) {
      onChange(undefined);
      return;
    }
    onChange(newValue);
  };

  const handleToggle = (enabled: boolean) => {
    setIsEnabled(enabled);
    if (!enabled) {
      onChange(undefined);
    } else {
      onChange({ duration: 0.5 });
    }
  };

  const panX = value?.pan?.x ?? 0;
  const panY = value?.pan?.y ?? 0;
  const zoom = value?.zoom ?? 1;
  const tilt = value?.tilt ?? 0;

  return (
    <div className="camera-picker">
      <div className="camera-header">
        <label>
          <input
            type="checkbox"
            checked={isEnabled}
            onChange={(e) => handleToggle(e.target.checked)}
          />
          Camera Effect
        </label>
      </div>

      {isEnabled && (
        <div className="camera-options">
          {/* Visual Preview */}
          <div className="camera-preview-container">
            <div
              className="camera-preview"
              style={{
                transform: `translate(${-panX * 0.1}px, ${-panY * 0.1}px) scale(${zoom}) rotate(${tilt}deg)`,
              }}
            >
              <div className="camera-preview-grid">
                <div className="grid-line horizontal" />
                <div className="grid-line vertical" />
              </div>
              <div className="camera-preview-center" />
            </div>
            <div className="camera-preview-frame" />
          </div>

          {/* Pan Controls */}
          <div className="camera-control-group">
            <label className="control-group-label">Pan</label>
            <div className="pan-controls">
              <div className="pan-row">
                <label>X</label>
                <input
                  type="range"
                  min="-200"
                  max="200"
                  step="10"
                  value={panX}
                  onChange={(e) => {
                    const x = parseInt(e.target.value, 10);
                    updateCamera({
                      pan: { x, y: panY },
                    });
                  }}
                />
                <span className="value-display">{panX}px</span>
              </div>
              <div className="pan-row">
                <label>Y</label>
                <input
                  type="range"
                  min="-200"
                  max="200"
                  step="10"
                  value={panY}
                  onChange={(e) => {
                    const y = parseInt(e.target.value, 10);
                    updateCamera({
                      pan: { x: panX, y },
                    });
                  }}
                />
                <span className="value-display">{panY}px</span>
              </div>
            </div>
          </div>

          {/* Zoom Control */}
          <div className="camera-control-group">
            <label className="control-group-label">Zoom</label>
            <div className="zoom-control">
              <input
                type="range"
                min="0.5"
                max="2"
                step="0.1"
                value={zoom}
                onChange={(e) => {
                  updateCamera({ zoom: parseFloat(e.target.value) });
                }}
              />
              <span className="value-display">{zoom.toFixed(1)}x</span>
            </div>
          </div>

          {/* Tilt Control */}
          <div className="camera-control-group">
            <label className="control-group-label">Tilt</label>
            <div className="tilt-control">
              <input
                type="range"
                min="-15"
                max="15"
                step="1"
                value={tilt}
                onChange={(e) => {
                  updateCamera({ tilt: parseInt(e.target.value, 10) });
                }}
              />
              <span className="value-display">{tilt}°</span>
            </div>
          </div>

          {/* Focus Point */}
          <div className="camera-control-group">
            <label className="control-group-label">Focus</label>
            <select
              value={value?.focus || ""}
              onChange={(e) => {
                updateCamera({ focus: e.target.value || undefined });
              }}
            >
              {FOCUS_POINTS.map((fp) => (
                <option key={fp.value} value={fp.value}>
                  {fp.label}
                </option>
              ))}
            </select>
          </div>

          {/* Duration */}
          <div className="camera-control-group">
            <label className="control-group-label">Duration</label>
            <div className="duration-control">
              <input
                type="range"
                min="0.1"
                max="3"
                step="0.1"
                value={value?.duration ?? 0.5}
                onChange={(e) => {
                  updateCamera({ duration: parseFloat(e.target.value) });
                }}
              />
              <span className="value-display">
                {(value?.duration ?? 0.5).toFixed(1)}s
              </span>
            </div>
          </div>

          {/* Easing */}
          <div className="camera-control-group">
            <label className="control-group-label">Easing</label>
            <select
              value={value?.easing || "ease_out"}
              onChange={(e) => {
                updateCamera({ easing: e.target.value as Easing });
              }}
            >
              {EASINGS.map((e) => (
                <option key={e.value} value={e.value}>
                  {e.label}
                </option>
              ))}
            </select>
          </div>

          {/* Reset Button */}
          <button
            type="button"
            className="camera-reset-button"
            onClick={() => {
              onChange({ duration: value?.duration ?? 0.5 });
            }}
          >
            Reset to Default
          </button>
        </div>
      )}
    </div>
  );
};
