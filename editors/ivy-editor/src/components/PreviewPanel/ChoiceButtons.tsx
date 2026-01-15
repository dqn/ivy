import type { ChoiceInfo } from "../../types/preview";

interface Props {
  choices: ChoiceInfo[];
}

export const ChoiceButtons: React.FC<Props> = ({ choices }) => {
  return (
    <div className="choice-buttons">
      {choices.map((choice, index) => (
        <button key={index} className="choice-button" disabled>
          {choice.label}
        </button>
      ))}
    </div>
  );
};
