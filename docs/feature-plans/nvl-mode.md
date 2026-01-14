# NVL Mode

Full-screen text display mode, similar to traditional novels.

## Overview

NVL (Novel) mode displays multiple lines of text across the entire screen, as opposed to ADV mode which shows one line at a time in a small text box at the bottom.

## Use Cases

- Prose-heavy visual novels with extensive narration
- Story sections that require the reader to digest larger chunks of text
- Introspective or atmospheric scenes

## Examples

- Fate/stay night
- Saya no Uta
- Higurashi When They Cry

## Implementation Considerations

### YAML Syntax

```yaml
script:
  - text: "This is ADV mode text."

  - nvl: true
    text: "This switches to NVL mode."

  - nvl: false
    text: "Back to ADV mode."
```

### Features

- Full-screen semi-transparent text box
- Multiple lines displayed at once
- Page-based text progression (clear screen when full)
- Dynamic switching between ADV and NVL within the same scenario

### Visual Design

- Text covers most of the screen (80-90%)
- Semi-transparent background overlay
- Scrollable text history within the current page
- Clear visual distinction from ADV mode

## References

- [Ren'Py NVL Mode Tutorial](https://www.renpy.org/doc/html/nvl_mode.html)
- [ADV vs NVL Style in Ren'Py](https://dev.to/codefatale/adv-vs-nvl-style-in-renpy-2a8m)
