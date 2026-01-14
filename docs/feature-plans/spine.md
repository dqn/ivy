# Spine Integration

Skeletal animation using Spine runtime.

## Overview

Spine is a 2D skeletal animation tool that allows for smooth character animations with smaller file sizes compared to frame-by-frame animation.

## Features

- Skeletal animation (bones, meshes, IK)
- Smooth transitions between animations
- Runtime manipulation of skeleton
- Smaller file size than sprite sheets

## Use Cases

- Complex character animations
- Combat sequences
- Detailed idle animations
- Procedural animation blending

## Implementation Considerations

### YAML Syntax

```yaml
characters:
  hero:
    spine:
      skeleton: "assets/spine/hero/hero.skel"
      atlas: "assets/spine/hero/hero.atlas"
      animations:
        idle: "idle"
        walk: "walk"
        attack: "attack_01"

script:
  - character:
      name: hero
      animation: idle
      loop: true
    text: "Ready for action."
```

### Technical Requirements

- Spine runtime library integration
- Support for Spine 4.x format
- Animation blending/mixing
- Attachment swapping for outfit changes

### Comparison with Live2D

| Aspect | Spine | Live2D |
|--------|-------|--------|
| Animation style | Skeletal | Mesh deformation |
| Best for | Action, movement | Subtle expressions |
| File size | Smaller | Larger |
| Learning curve | Moderate | Steeper |
| License | Per-seat | Revenue-based |

## References

- [Spine Runtime](http://esotericsoftware.com/)
- [Naninovel Spine Integration](https://naninovel.com/)
