interface Props {
  backgroundUrl: string | null;
  characterUrl: string | null;
  charPos: string | null;
}

export const SceneView: React.FC<Props> = ({
  backgroundUrl,
  characterUrl,
  charPos,
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

  return (
    <div className="scene-view">
      {backgroundUrl && (
        <div
          className="background-layer"
          style={{ backgroundImage: `url(${backgroundUrl})` }}
        />
      )}
      {characterUrl && (
        <img
          className="character-layer"
          src={characterUrl}
          alt="Character"
          style={getCharacterStyle()}
        />
      )}
    </div>
  );
};
