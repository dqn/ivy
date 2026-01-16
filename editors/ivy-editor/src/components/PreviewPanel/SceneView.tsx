import type { AssetError } from "../../hooks/usePlaytest";

interface Props {
  backgroundUrl: string | null;
  characterUrl: string | null;
  charPos: string | null;
  assetErrors?: AssetError[];
}

export const SceneView: React.FC<Props> = ({
  backgroundUrl,
  characterUrl,
  charPos,
  assetErrors = [],
}) => {
  const getCharacterStyle = (): React.CSSProperties => {
    const baseStyle: React.CSSProperties = {
      position: "absolute",
      bottom: 0,
      maxHeight: "80%",
      objectFit: "contain",
    };

    switch (charPos) {
      case "left":
        return { ...baseStyle, left: "10%" };
      case "right":
        return { ...baseStyle, right: "10%" };
      case "center":
      default:
        return { ...baseStyle, left: "50%", transform: "translateX(-50%)" };
    }
  };

  const backgroundError = assetErrors.find((e) => e.type === "background");
  const characterError = assetErrors.find((e) => e.type === "character");

  return (
    <div className="scene-view">
      {backgroundUrl && (
        <div
          className="background-layer"
          style={{ backgroundImage: `url(${backgroundUrl})` }}
        />
      )}
      {backgroundError && (
        <div className="asset-error-badge background-error" title={backgroundError.message}>
          ⚠ BG: {backgroundError.path}
        </div>
      )}
      {characterUrl && (
        <img
          className="character-layer"
          src={characterUrl}
          alt="Character"
          style={getCharacterStyle()}
        />
      )}
      {characterError && (
        <div className="asset-error-badge character-error" title={characterError.message}>
          ⚠ Char: {characterError.path}
        </div>
      )}
    </div>
  );
};
