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

    let bindings = KeyBindings {
        quick_save: KeyBinding {
            key: SerializableKeyCode(KeyCode::S),
            modifier: Some(Modifier::Shift),
        },
        ..Default::default()
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

    let bindings = KeyBindings {
        quick_save: KeyBinding {
            key: SerializableKeyCode(KeyCode::S),
            modifier: Some(Modifier::Shift),
        },
        ..Default::default()
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

    let bindings = KeyBindings {
        settings: KeyBinding {
            key: SerializableKeyCode(KeyCode::P),
            modifier: Some(Modifier::Ctrl),
        },
        ..Default::default()
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

    let bindings = KeyBindings {
        debug: KeyBinding {
            key: SerializableKeyCode(KeyCode::D),
            modifier: Some(Modifier::Alt),
        },
        ..Default::default()
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

    let bindings = KeyBindings {
        quick_save: KeyBinding {
            key: SerializableKeyCode(KeyCode::S),
            modifier: Some(Modifier::Shift),
        },
        ..Default::default()
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

#[test]
fn test_all_default_bindings_exist() {
    let bindings = KeyBindings::default();

    // Verify all actions have default bindings
    for action in Action::all() {
        let binding = bindings.get(*action);
        // All default bindings should have valid keys
        assert_ne!(binding.key.0, KeyCode::Unknown);
    }
}

#[test]
fn test_action_all_returns_complete_list() {
    let actions = Action::all();
    assert_eq!(actions.len(), 10); // Currently 10 actions defined

    // Verify all expected actions are present
    assert!(actions.contains(&Action::Advance));
    assert!(actions.contains(&Action::Rollback));
    assert!(actions.contains(&Action::AutoMode));
    assert!(actions.contains(&Action::SkipMode));
    assert!(actions.contains(&Action::Backlog));
    assert!(actions.contains(&Action::QuickSave));
    assert!(actions.contains(&Action::QuickLoad));
    assert!(actions.contains(&Action::Settings));
    assert!(actions.contains(&Action::Debug));
    assert!(actions.contains(&Action::Screenshot));
}

#[test]
fn test_action_display_names() {
    assert_eq!(Action::Advance.display_name(), "Advance");
    assert_eq!(Action::Rollback.display_name(), "Rollback");
    assert_eq!(Action::AutoMode.display_name(), "Auto Mode");
    assert_eq!(Action::SkipMode.display_name(), "Skip Mode");
    assert_eq!(Action::QuickSave.display_name(), "Quick Save");
}

#[test]
fn test_keybinding_display() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier};

    let binding = KeyBinding::new(KeyCode::Enter);
    assert_eq!(binding.display(), "Enter");

    let shift_binding = KeyBinding::with_modifier(KeyCode::S, Modifier::Shift);
    assert_eq!(shift_binding.display(), "Shift+S");

    let ctrl_binding = KeyBinding::with_modifier(KeyCode::P, Modifier::Ctrl);
    assert_eq!(ctrl_binding.display(), "Ctrl+P");

    let alt_binding = KeyBinding::with_modifier(KeyCode::D, Modifier::Alt);
    assert_eq!(alt_binding.display(), "Alt+D");
}

#[test]
fn test_keybinding_serialization() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier};

    let binding = KeyBinding::with_modifier(KeyCode::S, Modifier::Shift);
    let json = serde_json::to_string(&binding).unwrap();
    let restored: KeyBinding = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.key.0, KeyCode::S);
    assert_eq!(restored.modifier, Some(Modifier::Shift));
}

#[test]
fn test_keybindings_full_serialization() {
    let original = KeyBindings::default();
    let json = serde_json::to_string(&original).unwrap();
    let restored: KeyBindings = serde_json::from_str(&json).unwrap();

    // Verify all bindings are preserved
    assert_eq!(restored.advance.key.0, original.advance.key.0);
    assert_eq!(restored.rollback.key.0, original.rollback.key.0);
    assert_eq!(restored.auto_mode.key.0, original.auto_mode.key.0);
    assert_eq!(restored.skip_mode.key.0, original.skip_mode.key.0);
    assert_eq!(restored.backlog.key.0, original.backlog.key.0);
    assert_eq!(restored.quick_save.key.0, original.quick_save.key.0);
    assert_eq!(restored.quick_load.key.0, original.quick_load.key.0);
}

#[test]
fn test_set_and_get_binding() {
    use ivy::runtime::keybinds::KeyBinding;

    let mut bindings = KeyBindings::default();

    // Change advance key
    let new_binding = KeyBinding::new(KeyCode::Space);
    bindings.set(Action::Advance, new_binding);

    assert_eq!(bindings.get(Action::Advance).key.0, KeyCode::Space);
}

#[test]
fn test_custom_keybindings_roundtrip() {
    use ivy::runtime::keybinds::{KeyBinding, Modifier};

    let mut bindings = KeyBindings::default();

    // Customize several bindings
    bindings.set(Action::Advance, KeyBinding::new(KeyCode::Space));
    bindings.set(Action::Rollback, KeyBinding::with_modifier(KeyCode::Z, Modifier::Ctrl));
    bindings.set(Action::QuickSave, KeyBinding::with_modifier(KeyCode::S, Modifier::Ctrl));

    // Serialize and deserialize
    let json = serde_json::to_string(&bindings).unwrap();
    let restored: KeyBindings = serde_json::from_str(&json).unwrap();

    // Verify customizations are preserved
    assert_eq!(restored.get(Action::Advance).key.0, KeyCode::Space);
    assert_eq!(restored.get(Action::Rollback).key.0, KeyCode::Z);
    assert_eq!(restored.get(Action::Rollback).modifier, Some(Modifier::Ctrl));
    assert_eq!(restored.get(Action::QuickSave).key.0, KeyCode::S);
    assert_eq!(restored.get(Action::QuickSave).modifier, Some(Modifier::Ctrl));
}

#[test]
fn test_modifier_variants() {
    use ivy::runtime::keybinds::Modifier;

    // Verify modifier serialization
    let modifiers = [Modifier::Shift, Modifier::Ctrl, Modifier::Alt];

    for modifier in modifiers {
        let json = serde_json::to_string(&modifier).unwrap();
        let restored: Modifier = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, modifier);
    }
}
