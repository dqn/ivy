import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { CharacterDef } from "../../types/character";

interface Props {
  character: CharacterDef;
  baseDir: string | null;
  variantSelections?: Record<string, number>;
}

export const CharacterPreview: React.FC<Props> = ({
  character,
  baseDir,
  variantSelections = {},
}) => {
  const [imageUrls, setImageUrls] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!baseDir || !character.base) {
      setImageUrls([]);
      return;
    }

    const loadImages = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const urls: string[] = [];

        // Load base image
        if (character.base) {
          const baseData = await invoke<string>("read_asset_base64", {
            baseDir,
            assetPath: character.base,
          });
          urls.push(`data:image/png;base64,${baseData}`);
        }

        // Load layer images based on variant selections
        for (const layer of character.layers ?? []) {
          const variantIndex = variantSelections[layer.name] ?? 0;
          const imagePath = layer.images[variantIndex];

          if (imagePath) {
            try {
              const layerData = await invoke<string>("read_asset_base64", {
                baseDir,
                assetPath: imagePath,
              });
              urls.push(`data:image/png;base64,${layerData}`);
            } catch {
              // Skip if layer image not found
            }
          }
        }

        setImageUrls(urls);
      } catch (e) {
        setError(`Failed to load images: ${e}`);
        setImageUrls([]);
      } finally {
        setIsLoading(false);
      }
    };

    void loadImages();
  }, [baseDir, character, variantSelections]);

  if (!character.base) {
    return (
      <div className="character-preview empty">
        <p>Set a base image to preview</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="character-preview loading">
        <p>Loading preview...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="character-preview error">
        <p>{error}</p>
      </div>
    );
  }

  return (
    <div className="character-preview">
      <div className="preview-container">
        {imageUrls.map((url, index) => (
          <img
            key={index}
            src={url}
            alt={index === 0 ? "Base" : `Layer ${index}`}
            className="preview-layer"
            style={{ zIndex: index }}
          />
        ))}
      </div>
    </div>
  );
};
