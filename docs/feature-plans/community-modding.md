# Community Modding

Enable players to create and share custom content.

## Overview

Community modding allows players to extend games with new scenarios, characters, translations, and other content without modifying the core game files.

## Features

### Mod Types

| Type | Description |
|------|-------------|
| Scenarios | New stories or routes |
| Characters | New character sprites and definitions |
| Translations | Localization to other languages |
| Assets | Replacement backgrounds, music, UI |
| Patches | Bug fixes or balance changes |

### Mod Structure

```
mods/
├── my-new-route/
│   ├── mod.yaml           # Mod metadata
│   ├── scenario/
│   │   └── new_route.yaml
│   └── assets/
│       ├── characters/
│       └── backgrounds/
└── japanese-translation/
    ├── mod.yaml
    └── locales/
        └── ja.yaml
```

### Mod Metadata

```yaml
# mod.yaml
name: "New Route - Sakura's Secret"
version: "1.0.0"
author: "ModCreator"
description: "A new route exploring Sakura's backstory"
requires:
  ivy: ">=1.0.0"
  base_game: ">=1.0.0"
files:
  - scenario/new_route.yaml
```

## Implementation Considerations

### Loading System

- Scan `mods/` directory on startup
- Dependency resolution
- Conflict detection (same file modified by multiple mods)
- Load order management

### Security

- Sandboxed mod execution
- No arbitrary code execution
- Asset validation

### Distribution

- Workshop integration (Steam, itch.io)
- Mod manager UI in-game
- Version compatibility checking

## References

- [Ren'Py Modding](https://www.renpy.org/)
- [Naninovel Community Modding](https://naninovel.com/guide/community-modding)
