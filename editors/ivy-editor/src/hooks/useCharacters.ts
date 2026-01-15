import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { LayerDef, Scenario } from "../types/scenario";
import type { CharacterDef, CharacterDatabase } from "../types/character";
import {
  createEmptyCharacterDef,
  createEmptyCharacterDatabase,
  createEmptyLayerDef,
} from "../types/character";

interface UseCharactersReturn {
  database: CharacterDatabase;
  selectedCharacter: string | null;
  isDirty: boolean;

  // Database operations
  loadCharacters: (projectPath: string) => Promise<void>;
  saveCharacters: (projectPath: string) => Promise<void>;

  // Character operations
  selectCharacter: (name: string | null) => void;
  addCharacter: (name: string, def?: CharacterDef) => void;
  updateCharacter: (name: string, def: CharacterDef) => void;
  removeCharacter: (name: string) => void;
  renameCharacter: (oldName: string, newName: string) => void;

  // Layer operations
  addLayer: (charName: string, layer?: LayerDef) => void;
  updateLayer: (charName: string, index: number, layer: LayerDef) => void;
  removeLayer: (charName: string, index: number) => void;
  reorderLayers: (charName: string, fromIndex: number, toIndex: number) => void;

  // Alias operations
  addAlias: (charName: string, alias: string) => void;
  removeAlias: (charName: string, aliasIndex: number) => void;
  updateAlias: (charName: string, aliasIndex: number, newAlias: string) => void;

  // Utility
  extractSpeakers: (scenario: Scenario) => Promise<string[]>;
  findCharacterUsages: (scenario: Scenario, charName: string) => Promise<number[]>;
  getMergedCharacters: (scenario: Scenario) => Promise<Record<string, CharacterDef>>;
  findCharacterBySpeaker: (speaker: string) => Promise<string | null>;

  // Character names
  characterNames: string[];
}

export function useCharacters(): UseCharactersReturn {
  const [database, setDatabase] = useState<CharacterDatabase>(
    createEmptyCharacterDatabase()
  );
  const [selectedCharacter, setSelectedCharacter] = useState<string | null>(
    null
  );
  const [isDirty, setIsDirty] = useState(false);

  const loadCharacters = useCallback(async (projectPath: string) => {
    try {
      const loaded = await invoke<CharacterDatabase>("load_characters", {
        projectPath,
      });
      setDatabase(loaded);
      setIsDirty(false);
      setSelectedCharacter(null);
    } catch (e) {
      console.error("Failed to load characters:", e);
    }
  }, []);

  const saveCharacters = useCallback(
    async (projectPath: string) => {
      try {
        await invoke("save_characters", { projectPath, database });
        setIsDirty(false);
      } catch (e) {
        console.error("Failed to save characters:", e);
        throw e;
      }
    },
    [database]
  );

  const selectCharacter = useCallback((name: string | null) => {
    setSelectedCharacter(name);
  }, []);

  const addCharacter = useCallback(
    (name: string, def?: CharacterDef) => {
      if (database.characters[name]) {
        throw new Error(`Character '${name}' already exists`);
      }

      const newDatabase: CharacterDatabase = {
        ...database,
        characters: {
          ...database.characters,
          [name]: def ?? createEmptyCharacterDef(),
        },
      };
      setDatabase(newDatabase);
      setIsDirty(true);
      setSelectedCharacter(name);
    },
    [database]
  );

  const updateCharacter = useCallback(
    (name: string, def: CharacterDef) => {
      const newDatabase: CharacterDatabase = {
        ...database,
        characters: {
          ...database.characters,
          [name]: def,
        },
      };
      setDatabase(newDatabase);
      setIsDirty(true);
    },
    [database]
  );

  const removeCharacter = useCallback(
    (name: string) => {
      const { [name]: _, ...rest } = database.characters;
      const newDatabase: CharacterDatabase = {
        ...database,
        characters: rest,
      };
      setDatabase(newDatabase);
      setIsDirty(true);

      if (selectedCharacter === name) {
        setSelectedCharacter(null);
      }
    },
    [database, selectedCharacter]
  );

  const renameCharacter = useCallback(
    (oldName: string, newName: string) => {
      if (oldName === newName) return;
      if (database.characters[newName]) {
        throw new Error(`Character '${newName}' already exists`);
      }

      const charDef = database.characters[oldName];
      if (!charDef) return;

      const { [oldName]: _, ...rest } = database.characters;
      const newDatabase: CharacterDatabase = {
        ...database,
        characters: {
          ...rest,
          [newName]: charDef,
        },
      };
      setDatabase(newDatabase);
      setIsDirty(true);

      if (selectedCharacter === oldName) {
        setSelectedCharacter(newName);
      }
    },
    [database, selectedCharacter]
  );

  const addLayer = useCallback(
    (charName: string, layer?: LayerDef) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newLayers = [...(charDef.layers ?? []), layer ?? createEmptyLayerDef()];
      updateCharacter(charName, { ...charDef, layers: newLayers });
    },
    [database, updateCharacter]
  );

  const updateLayer = useCallback(
    (charName: string, index: number, layer: LayerDef) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newLayers = [...(charDef.layers ?? [])];
      newLayers[index] = layer;
      updateCharacter(charName, { ...charDef, layers: newLayers });
    },
    [database, updateCharacter]
  );

  const removeLayer = useCallback(
    (charName: string, index: number) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newLayers = (charDef.layers ?? []).filter((_, i) => i !== index);
      updateCharacter(charName, { ...charDef, layers: newLayers });
    },
    [database, updateCharacter]
  );

  const reorderLayers = useCallback(
    (charName: string, fromIndex: number, toIndex: number) => {
      const charDef = database.characters[charName];
      if (!charDef) return;
      if (fromIndex === toIndex) return;

      const newLayers = [...(charDef.layers ?? [])];
      const [removed] = newLayers.splice(fromIndex, 1);
      newLayers.splice(toIndex, 0, removed);
      updateCharacter(charName, { ...charDef, layers: newLayers });
    },
    [database, updateCharacter]
  );

  const addAlias = useCallback(
    (charName: string, alias: string) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newAliases = [...(charDef.aliases ?? []), alias];
      updateCharacter(charName, { ...charDef, aliases: newAliases });
    },
    [database, updateCharacter]
  );

  const removeAlias = useCallback(
    (charName: string, aliasIndex: number) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newAliases = (charDef.aliases ?? []).filter((_, i) => i !== aliasIndex);
      updateCharacter(charName, { ...charDef, aliases: newAliases });
    },
    [database, updateCharacter]
  );

  const updateAlias = useCallback(
    (charName: string, aliasIndex: number, newAlias: string) => {
      const charDef = database.characters[charName];
      if (!charDef) return;

      const newAliases = [...(charDef.aliases ?? [])];
      newAliases[aliasIndex] = newAlias;
      updateCharacter(charName, { ...charDef, aliases: newAliases });
    },
    [database, updateCharacter]
  );

  const extractSpeakers = useCallback(async (scenario: Scenario) => {
    try {
      return await invoke<string[]>("extract_speakers", { scenario });
    } catch (e) {
      console.error("Failed to extract speakers:", e);
      return [];
    }
  }, []);

  const findCharacterUsages = useCallback(
    async (scenario: Scenario, characterName: string) => {
      try {
        return await invoke<number[]>("find_character_usages", {
          scenario,
          characterName,
        });
      } catch (e) {
        console.error("Failed to find character usages:", e);
        return [];
      }
    },
    []
  );

  const getMergedCharacters = useCallback(
    async (scenario: Scenario) => {
      try {
        return await invoke<Record<string, CharacterDef>>("get_merged_characters", {
          database,
          scenario,
        });
      } catch (e) {
        console.error("Failed to get merged characters:", e);
        return {};
      }
    },
    [database]
  );

  const findCharacterBySpeaker = useCallback(
    async (speaker: string) => {
      try {
        return await invoke<string | null>("find_character_by_speaker", {
          database,
          speaker,
        });
      } catch (e) {
        console.error("Failed to find character by speaker:", e);
        return null;
      }
    },
    [database]
  );

  const characterNames = Object.keys(database.characters).sort();

  return {
    database,
    selectedCharacter,
    isDirty,
    loadCharacters,
    saveCharacters,
    selectCharacter,
    addCharacter,
    updateCharacter,
    removeCharacter,
    renameCharacter,
    addLayer,
    updateLayer,
    removeLayer,
    reorderLayers,
    addAlias,
    removeAlias,
    updateAlias,
    extractSpeakers,
    findCharacterUsages,
    getMergedCharacters,
    findCharacterBySpeaker,
    characterNames,
  };
}
