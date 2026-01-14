# Video Background

Use video files as animated backgrounds.

## Overview

Video backgrounds allow using looping video files instead of static images for backgrounds, adding atmosphere and dynamism to scenes.

## Use Cases

- Animated scenery (flowing water, moving clouds, flickering lights)
- Environmental effects (rain on window, fireplace)
- Futuristic/sci-fi interfaces with animated elements

## Implementation Considerations

### YAML Syntax

```yaml
script:
  - background:
      video: "assets/backgrounds/rain_window.webm"
      loop: true
    text: "The rain continues to fall..."

  - background:
      video: "assets/backgrounds/sunrise.webm"
      loop: false
      on_end: "assets/backgrounds/day.png"  # fallback to static image
    text: "A new day begins."
```

### Technical Requirements

- Leverage existing video playback infrastructure
- Support looping videos seamlessly
- Handle transition to/from static backgrounds
- Consider performance (video decoding overhead)
- Support alpha channel videos for overlay effects

### Supported Formats

- WebM (VP9) - recommended for web compatibility
- MP4 (H.264) - native playback
- Ensure format compatibility across platforms (native vs WASM)

## References

- [TyranoBuilder Video Backdrops](https://tyranobuilder.com/)
