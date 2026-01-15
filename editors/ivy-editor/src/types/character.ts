import type { LayerDef } from "./scenario";

/**
 * Character definition with aliases for speaker name mapping.
 * Extends ModularCharDef with speaker aliases.
 */
export interface CharacterDef {
  /** Base image path (body silhouette). */
  base: string;
  /** Speaker name aliases for this character. */
  aliases?: string[];
  /** Ordered list of layers (rendered from first to last). */
  layers?: LayerDef[];
}

/**
 * Character database stored in characters.yaml.
 */
export interface CharacterDatabase {
  characters: Record<string, CharacterDef>;
}

/**
 * Create an empty character definition.
 */
export function createEmptyCharacterDef(): CharacterDef {
  return {
    base: "",
    aliases: [],
    layers: [],
  };
}

/**
 * Create an empty character database.
 */
export function createEmptyCharacterDatabase(): CharacterDatabase {
  return {
    characters: {},
  };
}

/**
 * Create an empty layer definition.
 */
export function createEmptyLayerDef(name: string = "layer"): LayerDef {
  return {
    name,
    images: [],
  };
}
