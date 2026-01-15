# YAML Syntax Guide for ivy

This guide explains all available commands and options for ivy scenario files.

## File Structure

Every scenario file must have:

```yaml
title: "Your Story Title"

script:
  - text: "First line of dialogue"
```

## Basic Commands

### Text Display

```yaml
# Simple text
- text: "Hello, world!"

# Text with speaker name
- speaker: "Alice"
  text: "Nice to meet you!"

# Multi-line text (use |)
- text: |
    This is the first line.
    This is the second line.
    They will be displayed together.
```

### Labels and Jumps

```yaml
# Define a label
- label: chapter1
  text: "Chapter 1 begins."

# Jump to a label
- jump: chapter1

# Jump is unconditional - execution moves to the label immediately
```

### Choices

```yaml
- text: "What do you want to do?"
  choices:
    - label: "Go left"
      jump: left_path
    - label: "Go right"
      jump: right_path
    - label: "Stay here"
      jump: stay

# Each choice needs:
#   label: Text shown to player
#   jump: Label to go to when selected
```

### Timed Choices

```yaml
- text: "Quick! Choose now!"
  timeout: 5.0  # 5 seconds
  choices:
    - label: "Option A"
      jump: option_a
      default: true  # Selected if time runs out
    - label: "Option B"
      jump: option_b
```

## Images

### Background

```yaml
# Set background
- background: "assets/backgrounds/park.png"
  text: "A beautiful park."

# Clear background (empty string)
- background: ""
  text: "Darkness surrounds you."

# Background persists until changed
- text: "Still in the park."  # Same background
```

### Character Sprites

```yaml
# Show character
- character: "assets/characters/alice.png"
  char_pos: center  # left, center, or right
  text: "Alice appears."

# Change position
- character: "assets/characters/alice.png"
  char_pos: left
  text: "Alice moves to the left."

# Clear character
- character: ""
  text: "Alice leaves."
```

### Character Animations

```yaml
# Enter animation
- character: "assets/characters/alice.png"
  char_enter:
    type: fade        # none, fade, slide_left, slide_right
    duration: 0.5
  text: "Alice fades in."

# Exit animation
- char_exit:
    type: slide_right
    duration: 0.3
  text: "Alice slides away."

# Idle animation (looping)
- character: "assets/characters/alice.png"
  char_idle:
    type: breath     # none, breath, bob, sway, pulse
    duration: 2.0
    intensity: 0.3
  text: "Alice breathes gently."
```

### Multiple Characters

```yaml
- characters:
    - image: "assets/characters/alice.png"
      pos: left
    - image: "assets/characters/bob.png"
      pos: right
  text: "Alice and Bob face each other."
```

### Modular Characters

Define layered sprites in the scenario header:

```yaml
modular_characters:
  sakura:
    base: "assets/characters/sakura/base.png"
    layers:
      - name: "expression"
        images:
          - "assets/characters/sakura/expr_neutral.png"
          - "assets/characters/sakura/expr_smile.png"
          - "assets/characters/sakura/expr_angry.png"
      - name: "outfit"
        images:
          - "assets/characters/sakura/outfit_school.png"
          - "assets/characters/sakura/outfit_casual.png"

script:
  # Use modular character
  - modular_char:
      name: sakura
      expression: 1    # Index of the expression (smile)
      outfit: 0        # Index of outfit (school)
    char_pos: center
    text: "Sakura smiles."

  # Change only expression
  - modular_char:
      name: sakura
      expression: 2    # angry
    text: "Now she looks angry."
```

## Audio

### Background Music (BGM)

```yaml
# Start BGM (loops automatically)
- bgm: "assets/audio/theme.ogg"
  text: "Music begins."

# Stop BGM
- bgm: ""
  text: "Music stops."
```

### Sound Effects (SE)

```yaml
# Play once
- se: "assets/audio/door_knock.ogg"
  text: "*knock knock*"
```

### Voice

```yaml
# Voice plays once
- voice: "assets/audio/alice_hello.ogg"
  speaker: "Alice"
  text: "Hello!"
```

### Ambient Audio (Layered)

```yaml
# Start ambient sound
- ambient:
    - id: rain
      path: "assets/audio/rain.ogg"
      volume: 0.6
      looped: true
      fade_in: 0.5
  text: "Rain starts falling."

# Add another layer
- ambient:
    - id: thunder
      path: "assets/audio/thunder.ogg"
      volume: 0.4
      looped: false
  text: "Thunder rumbles."

# Stop specific ambient
- ambient_stop:
    - id: rain
      fade_out: 1.0
  text: "The rain stops."
```

## Variables

### Setting Variables

```yaml
# Set string
- set:
    name: player_name
    value: "Unknown"

# Set number
- set:
    name: coins
    value: 100

# Set boolean
- set:
    name: has_key
    value: true
```

### Using Variables in Text

```yaml
- text: "Hello, {var:player_name}!"
- text: "You have {var:coins} coins."
```

### Conditional Jumps

```yaml
- if:
    var: has_key
    is: true
    jump: open_door
  text: "Checking if you have the key..."

# If condition is true, jumps to open_door
# If false, continues to next command
```

### Player Input

```yaml
- input:
    var: player_name
    prompt: "What is your name?"
    default: "Player"

- text: "Nice to meet you, {var:player_name}!"
```

## Visual Effects

### Transitions

```yaml
- background: "assets/bg_night.png"
  transition:
    type: fade          # See types below
    duration: 1.0
    easing: ease_in_out
  text: "Night falls..."
```

**Transition types:**
- `none` - Instant change
- `fade` - Fade to black then new image
- `fade_white` - Fade to white
- `dissolve` - Cross-dissolve
- `wipe` - Wipe across screen
- `slide` - Slide in/out
- `pixelate` - Pixelate effect
- `iris` - Circular reveal
- `blinds` - Venetian blind effect

**Direction options (for wipe, slide, iris, blinds):**
```yaml
transition:
  type: wipe
  direction: left_to_right  # Or: right_to_left, top_to_bottom, bottom_to_top
```

### Screen Shake

```yaml
- shake:
    type: both        # horizontal, vertical, or both
    intensity: 15.0   # Pixels
    duration: 0.5
  text: "Earthquake!"
```

### Particles

```yaml
# Start particles
- particles: "snow"       # snow, rain, sakura, sparkle, leaves
  particle_intensity: 0.8 # 0.0 to 1.0
  text: "It's snowing."

# Stop particles
- particles: ""
  text: "Snow stops."
```

### Cinematic Bars (Letterbox)

```yaml
# Enable cinematic mode
- cinematic: true
  cinematic_duration: 0.5
  text: "A dramatic scene begins."

# Disable cinematic mode
- cinematic: false
  text: "Back to normal."
```

### Camera Effects

```yaml
# Zoom in
- camera:
    zoom: 1.5
    duration: 1.0
    easing: ease_out_quad
  text: "Zooming in..."

# Pan
- camera:
    pan:
      x: 100
      y: 50
    duration: 0.5
  text: "Panning..."

# Tilt
- camera:
    tilt: 5
    duration: 0.3
  text: "Camera tilts."

# Combined + Reset
- camera:
    pan: { x: 0, y: 0 }
    zoom: 1.0
    tilt: 0
    duration: 1.5
  text: "Reset camera."
```

**Focus points (for zoom):**
`center`, `top_left`, `top_center`, `top_right`, `left`, `right`, `bottom_left`, `bottom_center`, `bottom_right`

## Text Formatting

### Colored Text

```yaml
- text: "This is {color:red}important{/color} text."
- text: "{color:blue}Blue{/color} and {color:green}green{/color}."
```

### Ruby Text (Furigana)

```yaml
- text: "The {ruby:漢字:かんじ} are difficult."
```

## NVL Mode

NVL mode displays text in full-screen style (like a novel page).

```yaml
# Switch to NVL mode
- nvl: true
  text: "This text appears in NVL style."

- text: "More text accumulates on screen."

# Clear NVL buffer (new page)
- nvl_clear: true
  text: "Fresh page."

# Switch back to ADV mode
- nvl: false
  text: "Normal text box again."
```

## Chapters

Define chapters for chapter select feature:

```yaml
chapters:
  - id: ch1
    title: "Chapter 1: The Beginning"
    start_label: chapter1_start
  - id: ch2
    title: "Chapter 2: The Journey"
    start_label: chapter2_start

script:
  - label: chapter1_start
    text: "Chapter 1 begins..."
```

## Achievements

```yaml
- achievement:
    id: first_choice
    name: "Decision Maker"
    description: "Made your first choice"
  text: "Achievement unlocked!"
```

## Video (requires --features video)

### Full-screen Video

```yaml
- video:
    path: "assets/videos/opening.webm"
    skippable: true
    bgm_fade_out: 0.5
```

### Video Background

```yaml
# Start video background
- video_bg:
    path: "assets/videos/forest_loop.webm"
    looped: true
  text: "The forest comes alive."

# Stop video background
- video_bg:
    path: ""
  text: "Back to static image."
```

## Timing

### Wait

```yaml
- wait: 2.0  # Pause for 2 seconds
- text: "After the pause..."
```

## Easing Functions

Available for transitions, animations, and camera:

- `linear` - No easing
- `ease_in` - Slow start
- `ease_out` - Slow end
- `ease_in_out` - Slow start and end
- `ease_in_quad` / `ease_out_quad` / `ease_in_out_quad`
- `ease_in_cubic` / `ease_out_cubic` / `ease_in_out_cubic`
- `ease_in_back` / `ease_out_back` / `ease_in_out_back`
- `ease_out_bounce`

## Complete Example

```yaml
title: A Short Story

chapters:
  - id: ch1
    title: "The Meeting"
    start_label: start

script:
  - label: start
    background: "assets/bg_park.png"
    bgm: "assets/audio/peaceful.ogg"
    transition:
      type: fade
      duration: 1.0
    text: "A peaceful day in the park."

  - character: "assets/char_alice.png"
    char_pos: center
    char_enter:
      type: fade
      duration: 0.5
    speaker: "Alice"
    text: "Oh, hello there!"

  - input:
      var: name
      prompt: "What's your name?"
      default: "Stranger"

  - speaker: "Alice"
    text: "Nice to meet you, {var:name}!"

  - text: "How do you respond?"
    choices:
      - label: "Nice to meet you too!"
        jump: friendly
      - label: "Do I know you?"
        jump: confused

  - label: friendly
    set:
      name: friendship
      value: 1
    speaker: "Alice"
    text: "You seem nice!"
    jump: ending

  - label: confused
    speaker: "Alice"
    text: "Oh, I'm sorry. I thought..."
    jump: ending

  - label: ending
    character: ""
    transition:
      type: fade
      duration: 1.0
    text: "THE END"
    achievement:
      id: complete
      name: "Story Complete"
      description: "Finished the story"
```
