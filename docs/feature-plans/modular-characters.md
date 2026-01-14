# Modular Characters

Layered sprite system for compositing character parts dynamically.

## Overview

Modular characters (also known as LayeredImage or sprite compositing) allow combining multiple image layers (hair, eyes, mouth, clothes, etc.) at runtime instead of pre-rendering every combination.

## Benefits

- **Reduced file size**: 100MB+ can be reduced to ~60MB
- **Flexibility**: Easy to add new expressions, outfits, or accessories
- **Maintainability**: Update individual parts without re-exporting entire sprites

## Implementation Considerations

### YAML Syntax

```yaml
characters:
  sakura:
    base: "assets/characters/sakura/base.png"
    layers:
      hair:
        - "assets/characters/sakura/hair_normal.png"
        - "assets/characters/sakura/hair_wind.png"
      expression:
        - "assets/characters/sakura/expr_neutral.png"
        - "assets/characters/sakura/expr_smile.png"
        - "assets/characters/sakura/expr_angry.png"
      outfit:
        - "assets/characters/sakura/outfit_school.png"
        - "assets/characters/sakura/outfit_casual.png"

script:
  - character:
      name: sakura
      hair: 0
      expression: 1
      outfit: 0
    text: "Hello!"
```

### Layer Structure

1. **Base layer**: Body silhouette
2. **Hair back**: Hair behind the body
3. **Body/Outfit**: Clothing
4. **Face**: Eyebrows, eyes, mouth
5. **Hair front**: Hair in front of face
6. **Accessories**: Glasses, hats, etc.

### Rendering

- Layers rendered in order (back to front)
- Alpha blending for transparency
- Caching of frequently used combinations

## References

- [Ren'Py LayeredImage](https://www.renpy.org/doc/html/layeredimage.html)
- [TyranoBuilder 3.0 Modular Parts](https://www.nettosgameroom.com/2025/05/tyranobuilder-visual-novel-studio.html)
- [VNDev Wiki - Sprite](https://vndev.wiki/Sprite)
