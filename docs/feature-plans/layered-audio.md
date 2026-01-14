# Layered Audio

Multiple simultaneous audio tracks for ambient soundscapes.

## Overview

Layered audio allows playing multiple audio tracks simultaneously, enabling rich ambient soundscapes by combining background music with environmental sounds.

## Use Cases

- BGM + rain sounds + distant thunder
- City ambiance (traffic + crowd + music from a nearby shop)
- Nature scenes (wind + birds + water stream)

## Implementation Considerations

### YAML Syntax

```yaml
script:
  - audio:
      bgm: "assets/audio/calm_theme.ogg"
      ambient:
        - path: "assets/audio/rain.ogg"
          volume: 0.6
          loop: true
        - path: "assets/audio/thunder.ogg"
          volume: 0.4
          loop: false
          interval: [10, 30]  # random interval between plays
    text: "A stormy night..."

  - audio:
      ambient:
        - stop: "rain"
    text: "The rain has stopped."
```

### Audio Channels

| Channel | Purpose | Count |
|---------|---------|-------|
| BGM | Background music | 1 |
| Ambient | Environmental sounds | Multiple |
| SE | Sound effects | Multiple |
| Voice | Character voices | 1 |

### Features

- Independent volume control per channel
- Crossfade between ambient tracks
- Random interval playback for natural variation
- Ducking (lower ambient volume during voice)

### Technical Considerations

- Mix multiple audio sources in real-time
- Manage audio resource loading/unloading
- Handle platform-specific audio APIs

## References

- [Visual Novel Maker Layered Audio](https://www.visualnovelmaker.com/)
