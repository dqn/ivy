# Live2D Integration

2D character animation using Live2D Cubism SDK.

## Overview

Live2D is a technology for animating 2D illustrations, allowing characters to move naturally with breathing, blinking, and lip sync without 3D modeling.

## Features

### Basic Animations

- Idle motion (breathing, subtle movement)
- Eye blinking (automatic or controlled)
- Lip sync (volume-based mouth movement)
- Expression changes

### YAML Syntax

```yaml
characters:
  sakura:
    live2d:
      model: "assets/live2d/sakura/sakura.model3.json"
      motions:
        idle: "idle_01"
        happy: "motion_happy"
        sad: "motion_sad"
      expressions:
        normal: "exp_normal"
        smile: "exp_smile"

script:
  - character:
      name: sakura
      motion: idle
      expression: smile
    voice: "assets/voice/line_001.ogg"
    text: "Hello there!"
```

## Implementation Considerations

### SDK Integration

- Live2D Cubism SDK for Native (C++)
- Wrapper for Rust FFI
- WASM support considerations

### Lip Sync

Two approaches:
1. **Volume-based**: Simple, uses audio amplitude
2. **Phoneme-based**: Complex, analyzes vowel sounds

```yaml
# Volume-based lip sync parameters
live2d:
  lip_sync:
    sensitivity: 0.7  # 0.0-1.0
    parameter: "PARAM_MOUTH_OPEN_Y"
```

### Auto Animations

- Breathing: `PARAM_BREATH`
- Eye blink: `PARAM_EYE_L_OPEN`, `PARAM_EYE_R_OPEN`
- Random eye movement

### Licensing

Live2D requires a license for commercial use. Free for indie developers with revenue under certain thresholds.

## References

- [Live2D Cubism SDK Manual](https://docs.live2d.com/en/cubism-sdk-manual/)
- [Live2D Lip Sync](https://docs.live2d.com/en/cubism-sdk-manual/lipsync/)
- [Naninovel Live2D Integration](https://naninovel.com/)
