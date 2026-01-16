import { useRef, useCallback, useEffect, useState } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";

interface Props {
  path: string;
  baseDir: string | null;
  skippable: boolean;
  loopVideo: boolean;
  onComplete: () => void;
}

export const VideoPlayer: React.FC<Props> = ({
  path,
  baseDir,
  skippable,
  loopVideo,
  onComplete,
}) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [videoSrc, setVideoSrc] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  // Resolve video path
  useEffect(() => {
    const resolvePath = async () => {
      if (!baseDir || !path) {
        setError("Invalid video path");
        return;
      }

      // Check for absolute paths (Unix and Windows)
      const isAbsolute = path.startsWith("/") || /^[A-Za-z]:[\\/]/.test(path);
      const fullPath = isAbsolute ? path : await join(baseDir, path);
      const src = convertFileSrc(fullPath);
      setVideoSrc(src);
      setError(null);
    };
    void resolvePath();
  }, [path, baseDir]);

  const handleEnded = useCallback(() => {
    if (!loopVideo) {
      onComplete();
    }
  }, [loopVideo, onComplete]);

  const handleSkip = useCallback(() => {
    if (skippable) {
      onComplete();
    }
  }, [skippable, onComplete]);

  const handleError = useCallback(() => {
    setError("Failed to load video");
  }, []);

  if (error) {
    return (
      <div className="video-player-overlay">
        <div className="video-error">
          <span>{error}</span>
          <button onClick={onComplete}>Continue</button>
        </div>
      </div>
    );
  }

  if (!videoSrc) {
    return (
      <div className="video-player-overlay">
        <div className="video-loading">Loading video...</div>
      </div>
    );
  }

  return (
    <div className="video-player-overlay">
      <video
        ref={videoRef}
        className="video-player"
        src={videoSrc}
        autoPlay
        loop={loopVideo}
        onEnded={handleEnded}
        onError={handleError}
      />
      {skippable && (
        <button className="video-skip-button" onClick={handleSkip}>
          Skip
        </button>
      )}
      {loopVideo && (
        <button className="video-continue-button" onClick={onComplete}>
          Continue
        </button>
      )}
    </div>
  );
};
