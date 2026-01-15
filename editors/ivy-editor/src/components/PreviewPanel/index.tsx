import type { PreviewState } from "../../types/preview";
import { SceneView } from "./SceneView";
import { TextBox } from "./TextBox";
import { ChoiceButtons } from "./ChoiceButtons";
import { Controls } from "./Controls";
import "./styles.css";

interface Props {
  state: PreviewState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  onPrev: () => void;
  onNext: () => void;
  onGoto: (index: number) => void;
}

export const PreviewPanel: React.FC<Props> = ({
  state,
  backgroundUrl,
  characterUrl,
  onPrev,
  onNext,
}) => {
  if (!state) {
    return (
      <div className="preview-panel empty">
        <p>Load a scenario to see preview</p>
      </div>
    );
  }

  return (
    <div
      className={`preview-panel ${state.nvl_mode ? "nvl-mode" : "adv-mode"}`}
    >
      <SceneView
        backgroundUrl={backgroundUrl}
        characterUrl={characterUrl}
        charPos={state.char_pos}
      />

      {state.nvl_mode ? (
        <div className="nvl-text-area">
          {state.speaker && (
            <span className="nvl-speaker">{state.speaker}: </span>
          )}
          {state.text}
        </div>
      ) : (
        <TextBox speaker={state.speaker} text={state.text} />
      )}

      {state.choices.length > 0 && <ChoiceButtons choices={state.choices} />}

      <Controls
        currentIndex={state.command_index}
        totalCommands={state.total_commands}
        onPrev={onPrev}
        onNext={onNext}
      />
    </div>
  );
};
