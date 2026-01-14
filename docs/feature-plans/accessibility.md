# Accessibility Features

Features to make ivy games accessible to players with disabilities.

## Overview

Accessibility features ensure that visual novels can be enjoyed by players with visual, auditory, or motor impairments.

## Features

### Font Size Adjustment

Allow players to increase or decrease text size.

- Range: 50% - 200% of default
- Affects all text (dialogue, menus, UI)
- Text box size adjusts accordingly

### High Contrast Mode

Improve readability with high contrast colors.

- White text on black background
- Increased text border/shadow
- Simplified UI elements

### Screen Reader Support

Enable text-to-speech for visually impaired players.

- Self-voicing mode (built-in TTS)
- Clipboard voicing (for external screen readers)
- Announce scene changes, character names

### Dyslexia-Friendly Font

Support for dyslexia-friendly fonts.

- OpenDyslexic font option
- Adjustable letter spacing (kerning)
- Line height adjustment

## Implementation Considerations

### Accessibility Menu

Accessible via `Shift+A` (following Ren'Py convention).

```yaml
accessibility:
  font_size: 100  # percentage
  high_contrast: false
  self_voicing: false
  font: default  # or "opendyslexic"
  line_spacing: 1.0
```

### UI Requirements

- All interactive elements must be keyboard accessible
- Menu items should have text labels (not just images)
- Sufficient color contrast ratios (WCAG AA)
- Focus indicators for keyboard navigation

### Platform Considerations

- Native TTS APIs (platform-specific)
- WASM: Web Speech API
- Font loading for alternative fonts

## References

- [Ren'Py Accessibility](https://www.renpy.org/doc/html/self_voicing.html)
- [VNDev Wiki - Accessibility](https://vndev.wiki/Accessibility)
- [Making VNs Accessible for Screen Readers](https://metaphorsandmoonlight.com/how-to-make-visual-novels-accessible-for-screen-readers/)
