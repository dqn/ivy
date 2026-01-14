use ivy::runtime::Unlocks;

// Note: Unlocks methods that involve unlock_image()/unlock_ending() call save()
// internally. We test serialization and query methods on pre-populated data.

#[test]
fn test_unlocks_new_is_empty() {
    let unlocks = Unlocks::new();
    assert!(unlocks.unlocked_images().is_empty());
    assert_eq!(unlocks.image_count(), 0);
}

#[test]
fn test_unlocks_serialization_roundtrip() {
    let json = r#"{
        "images": ["bg1.png", "bg2.png", "cg1.png"],
        "endings": ["good_end", "bad_end"]
    }"#;

    let unlocks: Unlocks = serde_json::from_str(json).unwrap();

    assert_eq!(unlocks.image_count(), 3);
    assert!(unlocks.is_image_unlocked("bg1.png"));
    assert!(unlocks.is_image_unlocked("bg2.png"));
    assert!(unlocks.is_image_unlocked("cg1.png"));
    assert!(unlocks.is_ending_unlocked("good_end"));
    assert!(unlocks.is_ending_unlocked("bad_end"));
}

#[test]
fn test_unlocks_is_image_unlocked_false() {
    let unlocks = Unlocks::new();
    assert!(!unlocks.is_image_unlocked("nonexistent.png"));
}

#[test]
fn test_unlocks_is_ending_unlocked_false() {
    let unlocks = Unlocks::new();
    assert!(!unlocks.is_ending_unlocked("nonexistent"));
}

#[test]
fn test_unlocks_unlocked_images_sorted() {
    let json = r#"{
        "images": ["zebra.png", "apple.png", "mango.png"],
        "endings": []
    }"#;

    let unlocks: Unlocks = serde_json::from_str(json).unwrap();
    let images = unlocks.unlocked_images();

    assert_eq!(images.len(), 3);
    assert_eq!(images[0], "apple.png");
    assert_eq!(images[1], "mango.png");
    assert_eq!(images[2], "zebra.png");
}

#[test]
fn test_unlocks_image_count() {
    let json = r#"{
        "images": ["a.png", "b.png", "c.png", "d.png", "e.png"],
        "endings": []
    }"#;

    let unlocks: Unlocks = serde_json::from_str(json).unwrap();
    assert_eq!(unlocks.image_count(), 5);
}

#[test]
fn test_unlocks_empty_serialization() {
    let json = r#"{"images": [], "endings": []}"#;
    let unlocks: Unlocks = serde_json::from_str(json).unwrap();

    assert!(unlocks.unlocked_images().is_empty());
    assert_eq!(unlocks.image_count(), 0);
    assert!(!unlocks.is_image_unlocked("any.png"));
    assert!(!unlocks.is_ending_unlocked("any"));
}

#[test]
fn test_unlocks_default_fields() {
    // Test that missing fields default to empty
    let json = r#"{}"#;
    let unlocks: Unlocks = serde_json::from_str(json).unwrap();

    assert!(unlocks.unlocked_images().is_empty());
    assert_eq!(unlocks.image_count(), 0);
}

#[test]
fn test_unlocks_partial_fields() {
    // Only images field
    let json = r#"{"images": ["test.png"]}"#;
    let unlocks: Unlocks = serde_json::from_str(json).unwrap();

    assert_eq!(unlocks.image_count(), 1);
    assert!(unlocks.is_image_unlocked("test.png"));
    assert!(!unlocks.is_ending_unlocked("any"));
}

#[test]
fn test_unlocks_serialize_and_deserialize() {
    let json = r#"{
        "images": ["cg1.png", "cg2.png"],
        "endings": ["true_end"]
    }"#;

    let unlocks: Unlocks = serde_json::from_str(json).unwrap();
    let serialized = serde_json::to_string(&unlocks).unwrap();
    let restored: Unlocks = serde_json::from_str(&serialized).unwrap();

    assert_eq!(restored.image_count(), 2);
    assert!(restored.is_image_unlocked("cg1.png"));
    assert!(restored.is_image_unlocked("cg2.png"));
    assert!(restored.is_ending_unlocked("true_end"));
}
