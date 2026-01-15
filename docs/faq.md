# Frequently Asked Questions (FAQ)

Common issues and their solutions when working with ivy.

## Installation & Build Errors

### "cargo: command not found"

**Problem**: Rust is not installed or not in your PATH.

**Solution**:
1. Install Rust from [rustup.rs](https://rustup.rs/)
2. Restart your terminal
3. Verify with `rustc --version`

### "error: linker `cc` not found" (Linux)

**Problem**: C compiler is not installed.

**Solution**:
```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc
```

### "error: linker `link.exe` not found" (Windows)

**Problem**: Visual Studio Build Tools are not installed.

**Solution**:
1. Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Install "Desktop development with C++"
3. Restart your terminal

### Build is very slow

**Problem**: First build compiles all dependencies.

**Solution**: This is normal for the first build. Subsequent builds will be much faster due to caching. Use `cargo build --release` for optimized builds.

## YAML Syntax Errors

### "YAML parse error at line X"

**Problem**: Invalid YAML syntax.

**Common causes**:
- Using tabs instead of spaces (YAML requires spaces)
- Missing quotes around text with special characters
- Incorrect indentation

**Solution**:
```yaml
# Bad - uses tabs
script:
	- text: "Hello"

# Good - uses spaces (2 spaces per level)
script:
  - text: "Hello"
```

### "unknown field `choice`"

**Problem**: Using `choice` instead of `choices`.

**Solution**:
```yaml
# Bad
- text: "Pick one"
  choice:
    - label: "A"

# Good
- text: "Pick one"
  choices:
    - label: "A"
      jump: label_a
```

### "unknown field `pos`"

**Problem**: Using `pos` instead of `char_pos`.

**Solution**:
```yaml
# Bad
- character: "alice.png"
  pos: left

# Good
- character: "alice.png"
  char_pos: left
```

### "unknown field `goto`"

**Problem**: Using `goto` instead of `jump`.

**Solution**:
```yaml
# Bad
- goto: ending

# Good
- jump: ending
```

### "missing field `title`"

**Problem**: Scenario file is missing the required `title` field.

**Solution**:
```yaml
# Add title at the top of your file
title: My Visual Novel

script:
  - text: "Hello!"
```

### "missing field `script`"

**Problem**: Scenario file is missing the required `script` field.

**Solution**:
```yaml
title: My Visual Novel

# Add script with at least one command
script:
  - text: "Hello!"
```

## Runtime Errors

### "failed to load image: assets/..."

**Problem**: Image file not found or invalid format.

**Solution**:
1. Check the file path is correct (case-sensitive on Linux/macOS)
2. Ensure the file exists in the specified location
3. Use supported formats: PNG, JPG, BMP

```yaml
# Make sure the path matches exactly
- background: "assets/backgrounds/park.png"
```

### "failed to load audio: assets/..."

**Problem**: Audio file not found or invalid format.

**Solution**:
1. Check the file path is correct
2. Use supported formats: OGG, WAV, MP3 (OGG recommended)

### Character sprite not showing

**Problem**: Character image path is set but sprite doesn't appear.

**Possible causes**:
1. File path is incorrect
2. Image dimensions are too large
3. Character was cleared with empty string

**Solution**:
```yaml
# Ensure character is set with valid path
- character: "assets/characters/alice.png"
  char_pos: center
  text: "Hello!"
```

### Background not changing

**Problem**: Background stays the same after setting a new one.

**Solution**: Check that you're using a new path, not empty string:
```yaml
# This clears the background
- background: ""

# This sets a new background
- background: "assets/bg_new.png"
```

## Validation Errors

### "Undefined label reference: X"

**Problem**: Jumping to a label that doesn't exist.

**Solution**:
```yaml
# Make sure the label is defined
- label: my_label
  text: "This is the target"

# Then jump to it
- jump: my_label
```

### "Duplicate label: X"

**Problem**: Same label name used multiple times.

**Solution**: Use unique label names:
```yaml
# Bad
- label: start
  text: "First"
- label: start
  text: "Second"

# Good
- label: start
  text: "First"
- label: continue
  text: "Second"
```

### "Unused label: X"

**Problem**: Label is defined but never jumped to.

**Note**: This is a warning, not an error. The scenario will still run.

**Solution**: Either remove the unused label or add a jump to it.

### "Self-referencing jump at label: X"

**Problem**: A label jumps directly to itself, creating an infinite loop.

**Solution**:
```yaml
# Bad - infinite loop
- label: loop
  jump: loop

# Good - jump to different label
- label: loop
  text: "Processing..."
  jump: next_step
```

## Save/Load Issues

### Save file not working

**Problem**: Game doesn't save or load properly.

**Solution**:
1. Check `saves/` directory exists
2. Ensure write permissions
3. Don't modify save files manually

### "Failed to parse save data"

**Problem**: Save file is corrupted.

**Solution**: Delete the corrupted save file in `saves/` directory and start fresh.

## Performance Issues

### Game runs slowly

**Possible causes**:
1. Running debug build instead of release
2. Very large images
3. Too many particles

**Solution**:
```bash
# Run release build for better performance
cargo run --release

# Or build release binary
cargo build --release
```

### High memory usage

**Possible causes**:
1. Many large images loaded
2. Long backlog history

**Solution**:
- Use appropriately sized images (1920x1080 max for backgrounds)
- Consider compressing images

## WASM/Web Issues

### "wasm32-unknown-unknown" target not found

**Problem**: WASM target not installed.

**Solution**:
```bash
rustup target add wasm32-unknown-unknown
```

### "wasm-bindgen not found"

**Problem**: wasm-bindgen CLI not installed.

**Solution**:
```bash
cargo install wasm-bindgen-cli
```

### Audio not playing in browser

**Problem**: Browser blocks autoplay audio.

**Solution**: User must interact with the page (click) before audio can play. This is a browser security feature.

## Getting More Help

If your issue isn't listed here:

1. Run the validator: `cargo run --bin ivy-validate -- your_scenario.yaml`
2. Check the error message carefully - ivy provides helpful hints
3. Search [GitHub Issues](https://github.com/dqn/ivy/issues)
4. Create a new issue with:
   - ivy version (`cargo --version`)
   - Operating system
   - Complete error message
   - Minimal scenario file that reproduces the issue
