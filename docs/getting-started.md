# Getting Started with ivy

This guide will help you create your first visual novel game with ivy.

## Prerequisites

### Install Rust

ivy is written in Rust. Install it from [rustup.rs](https://rustup.rs/):

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download and run rustup-init.exe from https://rustup.rs/
```

After installation, restart your terminal and verify:

```bash
rustc --version
cargo --version
```

### Clone ivy

```bash
git clone https://github.com/dqn/ivy.git
cd ivy
```

## Your First Scenario

### 1. Create a scenario file

Create `assets/hello.yaml`:

```yaml
title: My First Visual Novel

script:
  - text: "Welcome to my first visual novel!"

  - speaker: "Alice"
    text: "Hello! I'm Alice."

  - speaker: "Alice"
    text: "Nice to meet you!"

  - text: "What will you say?"
    choices:
      - label: "Nice to meet you too!"
        jump: friendly
      - label: "..."
        jump: shy

  - label: friendly
    speaker: "Alice"
    text: "You're so friendly! I like that."
    jump: ending

  - label: shy
    speaker: "Alice"
    text: "Oh, you're a bit shy? That's okay!"
    jump: ending

  - label: ending
    text: "THE END"
```

### 2. Run your game

```bash
cargo run
```

The game window will open. Click or press Enter to advance the text.

## Adding Images

### Background

Add a background image to your scene:

```yaml
script:
  - background: "assets/backgrounds/park.png"
    text: "A beautiful day at the park."
```

### Character Sprite

Add a character sprite:

```yaml
script:
  - character: "assets/characters/alice.png"
    char_pos: center
    speaker: "Alice"
    text: "Here I am!"
```

Character positions: `left`, `center`, `right`

### Clearing Images

Use empty string to clear:

```yaml
script:
  - background: ""
    text: "The background fades away..."

  - character: ""
    text: "Alice leaves."
```

## Adding Audio

### Background Music (BGM)

```yaml
script:
  - bgm: "assets/audio/theme.ogg"
    text: "Music starts playing..."

  - bgm: ""
    text: "Music stops."
```

### Sound Effects (SE)

```yaml
script:
  - se: "assets/audio/door.ogg"
    text: "*knock knock*"
```

### Voice

```yaml
script:
  - voice: "assets/audio/alice_01.ogg"
    speaker: "Alice"
    text: "Hello!"
```

## Variables and Conditions

### Setting Variables

```yaml
script:
  - set:
      name: has_key
      value: true
    text: "You found a key!"
```

### Conditional Jumps

```yaml
script:
  - if:
      var: has_key
      is: true
      jump: open_door
    text: "Do you have the key?"
    jump: locked

  - label: open_door
    text: "The door opens!"

  - label: locked
    text: "The door is locked."
```

### Player Input

```yaml
script:
  - input:
      var: player_name
      prompt: "What is your name?"
      default: "Player"

  - text: "Hello, {var:player_name}!"
```

## Visual Effects

### Transitions

```yaml
script:
  - background: "assets/bg_night.png"
    transition:
      type: fade
      duration: 1.0
    text: "Night falls..."
```

Transition types: `fade`, `fade_white`, `dissolve`, `wipe`, `slide`, `pixelate`, `iris`, `blinds`

### Screen Shake

```yaml
script:
  - shake:
      type: both
      intensity: 15.0
      duration: 0.5
    text: "An earthquake!"
```

### Particles

```yaml
script:
  - particles: "snow"
    particle_intensity: 0.8
    text: "It's snowing..."
```

Particle types: `snow`, `rain`, `sakura`, `sparkle`, `leaves`

## Controls

### Keyboard / Mouse
- **Click / Enter**: Advance text
- **A**: Toggle auto mode
- **S**: Toggle skip mode
- **L**: Show backlog
- **↑ / Mouse wheel up**: Rollback
- **F5**: Quick save
- **F9**: Quick load
- **Escape**: Return to title / Exit

### Gamepad
- **A**: Advance text
- **B**: Cancel
- **X**: Toggle auto mode
- **Y**: Toggle skip mode
- **LB**: Rollback

## Development Tools

### Scenario Validation

Check your scenario for errors:

```bash
cargo run --bin ivy-validate -- assets/hello.yaml
```

### Live Preview

Preview changes in real-time:

```bash
cargo run --bin ivy-preview -- assets/hello.yaml
```

Open http://127.0.0.1:3000 in your browser.

### Hot Reload

Changes to YAML files are automatically detected and reloaded during development.

## Building for Release

### Native Build

```bash
cargo build --release
```

The executable will be in `target/release/`.

### WASM Build (Web)

```bash
./build-wasm.sh
```

The web build will be in `web/`.

## Next Steps

- Check the [YAML Syntax Guide](yaml-syntax.md) for all available commands
- See [FAQ](faq.md) for common issues and solutions
- Explore the `assets/` folder for example scenarios

## Getting Help

- [GitHub Issues](https://github.com/dqn/ivy/issues) - Report bugs or request features
- Check `CLAUDE.md` for detailed documentation on all features
