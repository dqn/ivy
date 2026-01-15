interface Props {
  speaker: string | null;
  text: string | null;
}

export const TextBox: React.FC<Props> = ({ speaker, text }) => {
  return (
    <div className="text-box">
      {speaker && <div className="speaker-name">{speaker}</div>}
      <div className="dialogue-text">{text ?? ""}</div>
    </div>
  );
};
