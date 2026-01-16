import { useTranslation } from "react-i18next";
import type { PreviewState } from "../../types/preview";
import type { PlaytestState, InputDisplay, WaitDisplay, VideoDisplay } from "../../types/playtest";
import type { AssetError } from "../../hooks/usePlaytest";
import { SceneView } from "./SceneView";
import { TextBox } from "./TextBox";
import { ChoiceButtons } from "./ChoiceButtons";
import { Controls } from "./Controls";
import { PlaytestControls } from "./PlaytestControls";
import { LanguageSelector } from "./LanguageSelector";
import { InputForm } from "./InputForm";
import { WaitOverlay } from "./WaitOverlay";
import { VideoPlayer } from "./VideoPlayer";
import { TransitionOverlay } from "./TransitionOverlay";
import "./styles.css";

// Preview mode props (read-only navigation)
interface PreviewModeProps {
  mode: "preview";
  state: PreviewState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  baseDir: string | null;
  language: string;
  onLanguageChange: (lang: string) => void;
  onPrev: () => void;
  onNext: () => void;
  onGoto: (index: number) => void;
}

// Playtest mode props (interactive execution)
interface PlaytestModeProps {
  mode: "playtest";
  state: PlaytestState | null;
  backgroundUrl: string | null;
  characterUrl: string | null;
  assetErrors?: AssetError[];
  baseDir: string | null;
  language: string;
  isAutoMode: boolean;
  isSkipMode: boolean;
  onLanguageChange: (lang: string) => void;
  onAdvance: () => void;
  onSelectChoice: (index: number) => void;
  onRollback: () => void;
  onRestart: () => void;
  onSubmitInput: (value: string) => void;
  onToggleAutoMode: () => void;
  onToggleSkipMode: () => void;
}

type Props = PreviewModeProps | PlaytestModeProps;

// Helper to extract display info from playtest state
function getPlaytestDisplayInfo(state: PlaytestState) {
  const display = state.display;

  switch (display.type) {
    case "text":
      return {
        speaker: display.speaker,
        text: display.text,
        nvlMode: display.nvl_mode,
        choices: [],
        timeout: null,
        defaultChoice: null,
      };
    case "choices":
      return {
        speaker: display.speaker,
        text: display.text,
        nvlMode: false,
        choices: display.choices.map((c) => ({ label: c.label, jump: c.jump })),
        timeout: display.timeout,
        defaultChoice: display.default_choice,
      };
    case "input":
      return {
        speaker: null,
        text: display.prompt,
        nvlMode: false,
        choices: [],
        timeout: null,
        defaultChoice: null,
      };
    case "wait":
      return {
        speaker: null,
        text: `[Wait: ${display.duration}s]`,
        nvlMode: false,
        choices: [],
        timeout: null,
        defaultChoice: null,
      };
    case "video":
      return {
        speaker: null,
        text: `[Video: ${display.path}]`,
        nvlMode: false,
        choices: [],
        timeout: null,
        defaultChoice: null,
      };
    case "end":
      return {
        speaker: null,
        text: "[End of scenario]",
        nvlMode: false,
        choices: [],
        timeout: null,
        defaultChoice: null,
      };
  }
}

export const PreviewPanel: React.FC<Props> = (props) => {
  const { t } = useTranslation();
  const { mode, state, backgroundUrl, characterUrl, baseDir, language, onLanguageChange } = props;

  if (!state) {
    return (
      <div className="preview-panel empty">
        <p>{t("previewPanel.loadScenario")}</p>
      </div>
    );
  }

  // Get display info based on mode
  const displayInfo =
    mode === "playtest"
      ? getPlaytestDisplayInfo(state)
      : {
          speaker: state.speaker,
          text: state.text,
          nvlMode: state.nvl_mode,
          choices: state.choices,
          timeout: null as number | null,
          defaultChoice: null as number | null,
        };

  const totalCommands = state.total_commands;
  const currentIndex = state.command_index;

  return (
    <div
      className={`preview-panel ${displayInfo.nvlMode ? "nvl-mode" : "adv-mode"} ${mode}`}
    >
      <SceneView
        backgroundUrl={backgroundUrl}
        characterUrl={characterUrl}
        charPos={
          mode === "preview"
            ? state.char_pos
            : state.display.type !== "video" && state.display.type !== "end"
              ? state.display.char_pos
              : null
        }
        assetErrors={mode === "playtest" ? props.assetErrors : undefined}
      />

      {displayInfo.nvlMode ? (
        <div className="nvl-text-area">
          {displayInfo.speaker && (
            <span className="nvl-speaker">{displayInfo.speaker}: </span>
          )}
          {displayInfo.text}
        </div>
      ) : (
        <TextBox
          speaker={displayInfo.speaker}
          text={displayInfo.text}
          typewriterEnabled={mode === "playtest" && displayInfo.choices.length === 0}
          typewriterSpeed={40}
        />
      )}

      {displayInfo.choices.length > 0 && (
        <ChoiceButtons
          choices={displayInfo.choices}
          disabled={mode === "preview"}
          timeout={displayInfo.timeout}
          defaultChoice={displayInfo.defaultChoice}
          onSelect={mode === "playtest" ? props.onSelectChoice : undefined}
        />
      )}

      {mode === "playtest" && state.display.type === "input" && (
        <InputForm
          prompt={(state.display as InputDisplay).prompt}
          defaultValue={(state.display as InputDisplay).default_value}
          onSubmit={props.onSubmitInput}
        />
      )}

      {mode === "playtest" && state.display.type === "wait" && (
        <WaitOverlay
          duration={(state.display as WaitDisplay).duration}
          onComplete={props.onAdvance}
          onSkip={props.onAdvance}
        />
      )}

      {mode === "playtest" && state.display.type === "video" && (
        <VideoPlayer
          path={(state.display as VideoDisplay).path}
          baseDir={baseDir}
          skippable={(state.display as VideoDisplay).skippable}
          loopVideo={(state.display as VideoDisplay).loop_video}
          onComplete={props.onAdvance}
        />
      )}

      {mode === "playtest" && (
        <TransitionOverlay transition={state.transition} />
      )}

      {mode === "preview" ? (
        <Controls
          currentIndex={currentIndex}
          totalCommands={totalCommands}
          onPrev={props.onPrev}
          onNext={props.onNext}
        />
      ) : (
        <PlaytestControls
          canRollback={state.can_rollback}
          isEnded={state.is_ended}
          isBlocked={
            state.display.type === "choices" ||
            state.display.type === "input" ||
            state.display.type === "wait" ||
            state.display.type === "video"
          }
          historyCount={state.history_count}
          isAutoMode={props.isAutoMode}
          isSkipMode={props.isSkipMode}
          onAdvance={props.onAdvance}
          onRollback={props.onRollback}
          onRestart={props.onRestart}
          onToggleAutoMode={props.onToggleAutoMode}
          onToggleSkipMode={props.onToggleSkipMode}
        />
      )}

      <LanguageSelector language={language} onChange={onLanguageChange} />

      {mode === "playtest" && state.is_ended && (
        <div className="playtest-ended-overlay">
          <p>{t("previewPanel.scenarioEnded")}</p>
          <button onClick={props.onRestart}>{t("previewPanel.restart")}</button>
        </div>
      )}
    </div>
  );
};
