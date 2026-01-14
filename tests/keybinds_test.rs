use ivy::input::test::TestInput;
use ivy::runtime::{Action, KeyBindings};
use macroquad::prelude::KeyCode;

#[test]
fn test_default_keybindings() {
    let bindings = KeyBindings::default();

    assert_eq!(bindings.get(Action::Advance).key.0, KeyCode::Enter);
    assert_eq!(bindings.get(Action::Rollback).key.0, KeyCode::Up);
    assert_eq!(bindings.get(Action::AutoMode).key.0, KeyCode::A);
    assert_eq!(bindings.get(Action::SkipMode).key.0, KeyCode::S);
}

#[test]
fn test_is_pressed_with_simple_key() {
    let bindings = KeyBindings::default();
    let mut input = TestInput::new();

    // Key not pressed
    assert!(!bindings.is_pressed_with(Action::Advance, &input));

    // Press Enter
    input.press_key(KeyCode::Enter);
    assert!(bindings.is_pressed_with(Action::Advance, &input));
}

#[test]
fn test_is_pressed_with_wrong_key() {
    let bindings = KeyBindings::default();
    let mut input = TestInput::new();

    // Press wrong key (Space instead of Enter)
    input.press_key(KeyCode::Space);
    assert!(!bindings.is_pressed_with(Action::Advance, &input));
}

#[test]
fn test_is_pressed_with_modifier() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier, SerializableKeyCode};

    let mut bindings = KeyBindings::default();
    // Set QuickSave to Shift+S
    bindings.quick_save = KeyBinding {
        key: SerializableKeyCode(KeyCode::S),
        modifier: Some(Modifier::Shift),
    };

    let mut input = TestInput::new();

    // Just S without Shift - should not trigger
    input.press_key(KeyCode::S);
    assert!(!bindings.is_pressed_with(Action::QuickSave, &input));

    input.clear_frame();

    // S with Shift held - should trigger
    input.hold_key(KeyCode::LeftShift);
    input.press_key(KeyCode::S);
    assert!(bindings.is_pressed_with(Action::QuickSave, &input));
}

#[test]
fn test_is_pressed_with_right_modifier() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier, SerializableKeyCode};

    let mut bindings = KeyBindings::default();
    bindings.quick_save = KeyBinding {
        key: SerializableKeyCode(KeyCode::S),
        modifier: Some(Modifier::Shift),
    };

    let mut input = TestInput::new();

    // Right Shift should also work
    input.hold_key(KeyCode::RightShift);
    input.press_key(KeyCode::S);
    assert!(bindings.is_pressed_with(Action::QuickSave, &input));
}

#[test]
fn test_is_pressed_with_ctrl_modifier() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier, SerializableKeyCode};

    let mut bindings = KeyBindings::default();
    bindings.settings = KeyBinding {
        key: SerializableKeyCode(KeyCode::P),
        modifier: Some(Modifier::Ctrl),
    };

    let mut input = TestInput::new();

    // Ctrl+P should trigger
    input.hold_key(KeyCode::LeftControl);
    input.press_key(KeyCode::P);
    assert!(bindings.is_pressed_with(Action::Settings, &input));
}

#[test]
fn test_is_pressed_with_alt_modifier() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier, SerializableKeyCode};

    let mut bindings = KeyBindings::default();
    bindings.debug = KeyBinding {
        key: SerializableKeyCode(KeyCode::D),
        modifier: Some(Modifier::Alt),
    };

    let mut input = TestInput::new();

    // Alt+D should trigger
    input.hold_key(KeyCode::LeftAlt);
    input.press_key(KeyCode::D);
    assert!(bindings.is_pressed_with(Action::Debug, &input));
}

#[test]
fn test_multiple_keys_pressed() {
    let bindings = KeyBindings::default();
    let mut input = TestInput::new();

    input.press_key(KeyCode::Enter);
    input.press_key(KeyCode::A);

    // Both should trigger
    assert!(bindings.is_pressed_with(Action::Advance, &input));
    assert!(bindings.is_pressed_with(Action::AutoMode, &input));
}

#[test]
fn test_clear_frame_resets_pressed_state() {
    let bindings = KeyBindings::default();
    let mut input = TestInput::new();

    input.press_key(KeyCode::Enter);
    assert!(bindings.is_pressed_with(Action::Advance, &input));

    input.clear_frame();
    assert!(!bindings.is_pressed_with(Action::Advance, &input));
}

#[test]
fn test_hold_key_persists_after_clear_frame() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier, SerializableKeyCode};

    let mut bindings = KeyBindings::default();
    bindings.quick_save = KeyBinding {
        key: SerializableKeyCode(KeyCode::S),
        modifier: Some(Modifier::Shift),
    };

    let mut input = TestInput::new();

    // Hold Shift
    input.hold_key(KeyCode::LeftShift);
    input.press_key(KeyCode::S);
    assert!(bindings.is_pressed_with(Action::QuickSave, &input));

    // Clear frame - Shift should still be held
    input.clear_frame();
    input.press_key(KeyCode::S);
    assert!(bindings.is_pressed_with(Action::QuickSave, &input));

    // Release Shift
    input.release_key(KeyCode::LeftShift);
    input.clear_frame();
    input.press_key(KeyCode::S);
    assert!(!bindings.is_pressed_with(Action::QuickSave, &input));
}
