# Dynamic Camera

Camera controls for panning, zooming, and tilting scenes.

## Overview

Dynamic camera adds cinematic effects by allowing the viewport to move, zoom, and rotate, creating more dramatic and engaging scenes.

## Features

### Pan

Horizontal or vertical movement of the camera.

```yaml
script:
  - camera:
      pan:
        x: 100
        y: 0
      duration: 1.0
      easing: ease_out_quad
    text: "The camera pans to the right..."
```

### Zoom

Scale the view in or out.

```yaml
script:
  - camera:
      zoom: 1.5
      focus: center  # or specific coordinates
      duration: 0.5
    text: "A close-up shot."
```

### Tilt

Rotate the camera.

```yaml
script:
  - camera:
      tilt: 5  # degrees
      duration: 0.3
    text: "Something feels off..."
```

### Combined Effects

```yaml
script:
  - camera:
      pan: { x: 50, y: -20 }
      zoom: 1.2
      tilt: -3
      duration: 2.0
      easing: ease_in_out_cubic
```

## Implementation Considerations

- Apply transformations to the entire render target
- Use existing easing functions (14 types already implemented)
- Reset camera state between scenes or on specific commands
- Coordinate with character positions

## Use Cases

- Dramatic reveals
- Action sequences
- Horror/suspense moments
- Emphasizing character reactions

## References

- [TyranoBuilder Dynamic Camera](https://tyranobuilder.com/)
