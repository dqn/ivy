import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  baseDir: string;
  path: string;
  type: string;
}

export const AssetPreview: React.FC<Props> = ({ baseDir, path, type }) => {
  const [dataUrl, setDataUrl] = useState<string | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    const loadAsset = async () => {
      if (type === "image" || type === "audio") {
        try {
          const url = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: path,
          });
          setDataUrl(url);
        } catch {
          setDataUrl(null);
        }
      }
    };

    void loadAsset();
    setIsPlaying(false);
  }, [baseDir, path, type]);

  const toggleAudio = () => {
    if (!audioRef.current) {
      return;
    }

    if (isPlaying) {
      audioRef.current.pause();
      audioRef.current.currentTime = 0;
    } else {
      void audioRef.current.play();
    }
    setIsPlaying(!isPlaying);
  };

  if (type === "image" && dataUrl) {
    return (
      <div className="asset-preview image-preview">
        <img src={dataUrl} alt={path} />
      </div>
    );
  }

  if (type === "audio" && dataUrl) {
    return (
      <div className="asset-preview audio-preview">
        <audio
          ref={audioRef}
          src={dataUrl}
          onEnded={() => setIsPlaying(false)}
        />
        <button className="play-button" onClick={toggleAudio}>
          {isPlaying ? "⏹️ Stop" : "▶️ Play"}
        </button>
        <div className="waveform-placeholder">🎵 {path}</div>
      </div>
    );
  }

  return (
    <div className="asset-preview no-preview">
      <p>Preview not available</p>
    </div>
  );
};
