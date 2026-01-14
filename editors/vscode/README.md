# Ivy Visual Novel - VSCode Extension

Language support for Ivy visual novel scenario files.

## Features

- Syntax highlighting for `.ivy.yaml` and `.ivy.yml` files
- Code snippets for common scenario patterns
- YAML-based scenarios also get snippets

## Installation

### From Source

1. Copy the `editors/vscode` directory to your VSCode extensions folder:
   - Windows: `%USERPROFILE%\.vscode\extensions\ivy-vn`
   - macOS/Linux: `~/.vscode/extensions/ivy-vn`

2. Restart VSCode

### Using the Extension

For dedicated ivy scenario files, use the `.ivy.yaml` or `.ivy.yml` extension to get full syntax highlighting.

For regular `.yaml` files, snippets are still available with the `ivy-` prefix.

## Snippets

All snippets start with `ivy-` prefix:

| Prefix | Description |
|--------|-------------|
| `ivy-scenario` | New scenario template |
| `ivy-text` | Text dialogue |
| `ivy-speaker` | Dialogue with speaker |
| `ivy-label` | Label definition |
| `ivy-jump` | Jump command |
| `ivy-choices` | Choice options |
| `ivy-choices-timeout` | Timed choices |
| `ivy-bg` | Background change |
| `ivy-char` | Character display |
| `ivy-char-anim` | Character with animation |
| `ivy-chars` | Multiple characters |
| `ivy-bgm` | Background music |
| `ivy-se` | Sound effect |
| `ivy-voice` | Voice playback |
| `ivy-transition` | Transition effect |
| `ivy-shake` | Screen shake |
| `ivy-set` | Variable assignment |
| `ivy-if` | Conditional jump |
| `ivy-input` | Text input |
| `ivy-wait` | Wait duration |
| `ivy-particles` | Particle effects |
| `ivy-cinematic` | Cinematic mode |
| `ivy-camera-pan` | Camera pan |
| `ivy-camera-zoom` | Camera zoom |
| `ivy-nvl` | NVL mode |
| `ivy-achievement` | Achievement unlock |
| `ivy-video` | Video playback |
| `ivy-video-bg` | Video background |
| `ivy-ambient` | Ambient audio |
| `ivy-ambient-stop` | Stop ambient |
| `ivy-chapter` | Chapter definition |
| `ivy-modular-char` | Modular character |
| `ivy-color` | Colored text tag |
| `ivy-ruby` | Ruby (furigana) tag |
| `ivy-var` | Variable reference |

## Syntax Highlighting

The extension highlights:

- **Keywords**: `label`, `jump`, `if`, `choices`, `title`, `script`, `chapters`
- **Display tags**: `text`, `speaker`
- **Media tags**: `background`, `character`, `bgm`, `se`, `voice`, `video`
- **Animation tags**: `transition`, `shake`, `camera`, `char_enter`, `char_exit`
- **System tags**: `set`, `input`, `wait`, `timeout`, `achievement`
- **Values**: positions, transition types, easing functions
- **Inline tags**: `{color:...}`, `{ruby:...}`, `{var:...}`
