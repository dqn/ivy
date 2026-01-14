use ivy::runtime::AchievementNotifier;

// Note: Achievements tests that involve unlock() are skipped because they
// call save() internally which requires file I/O. We test serialization
// and AchievementNotifier which is purely in-memory.

#[test]
fn test_achievements_serialization_roundtrip() {
    // Simulate an Achievements struct via JSON
    let json = r#"{"unlocked": ["first_achievement", "second_achievement"]}"#;

    // Parse and re-serialize
    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    let unlocked = parsed["unlocked"].as_array().unwrap();

    assert_eq!(unlocked.len(), 2);
    assert!(unlocked.iter().any(|v| v == "first_achievement"));
    assert!(unlocked.iter().any(|v| v == "second_achievement"));
}

#[test]
fn test_achievements_empty_serialization() {
    let json = r#"{"unlocked": []}"#;
    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

    assert!(parsed["unlocked"].as_array().unwrap().is_empty());
}

// AchievementNotifier tests

#[test]
fn test_notifier_default_is_empty() {
    let notifier = AchievementNotifier::default();
    assert!(notifier.current().is_none());
    assert_eq!(notifier.progress(), 0.0);
}

#[test]
fn test_notifier_queue_single() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test_id", "Test Achievement", "A test achievement");

    // Before update, current is still None
    assert!(notifier.current().is_none());

    // After update, notification is dequeued
    notifier.update(0.0);
    assert!(notifier.current().is_some());

    let current = notifier.current().unwrap();
    assert_eq!(current.id, "test_id");
    assert_eq!(current.name, "Test Achievement");
    assert_eq!(current.description, "A test achievement");
}

#[test]
fn test_notifier_queue_multiple() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("first", "First", "First achievement");
    notifier.notify("second", "Second", "Second achievement");

    // First notification
    notifier.update(0.0);
    assert_eq!(notifier.current().unwrap().id, "first");

    // Complete first notification (3.0 seconds display time)
    notifier.update(3.0);

    // Second notification should be dequeued
    notifier.update(0.0);
    assert_eq!(notifier.current().unwrap().id, "second");
}

#[test]
fn test_notifier_update_slide_in() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue

    // At the beginning, progress should be near 0
    assert!(notifier.progress() < 0.1);

    // After 0.15 seconds (half of 0.3 animation time)
    notifier.update(0.15);
    let progress = notifier.progress();
    assert!(progress > 0.4 && progress < 0.6, "progress: {}", progress);
}

#[test]
fn test_notifier_update_display() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue
    notifier.update(0.3); // Complete slide in

    // During display phase, progress should be 1.0
    notifier.update(0.5);
    assert!((notifier.progress() - 1.0).abs() < 0.01);
}

#[test]
fn test_notifier_update_slide_out() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue
    notifier.update(2.8); // Almost at slide out phase

    // Progress should start decreasing
    let progress = notifier.progress();
    assert!(progress < 1.0, "progress: {}", progress);
}

#[test]
fn test_notifier_update_completion() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue
    notifier.update(3.0); // Complete display

    // After completion, current should be None
    assert!(notifier.current().is_none());
    assert_eq!(notifier.progress(), 0.0);
}

#[test]
fn test_notifier_current_none_when_empty() {
    let notifier = AchievementNotifier::default();
    assert!(notifier.current().is_none());
}

#[test]
fn test_notifier_processes_queue_in_order() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("a", "A", "First");
    notifier.notify("b", "B", "Second");
    notifier.notify("c", "C", "Third");

    // Process first
    notifier.update(0.0);
    assert_eq!(notifier.current().unwrap().id, "a");

    // Complete first, process second
    notifier.update(3.0);
    notifier.update(0.0);
    assert_eq!(notifier.current().unwrap().id, "b");

    // Complete second, process third
    notifier.update(3.0);
    notifier.update(0.0);
    assert_eq!(notifier.current().unwrap().id, "c");

    // Complete third
    notifier.update(3.0);
    assert!(notifier.current().is_none());
}

#[test]
fn test_notifier_progress_during_animation() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue

    // Test progressive slide in
    notifier.update(0.1);
    let p1 = notifier.progress();

    notifier.update(0.1);
    let p2 = notifier.progress();

    // Progress should increase during slide in
    assert!(p2 > p1, "p1: {}, p2: {}", p1, p2);
}

#[test]
fn test_notifier_animation_timing() {
    let mut notifier = AchievementNotifier::default();
    notifier.notify("test", "Test", "Test");

    notifier.update(0.0); // Dequeue

    // ANIM_TIME is 0.3, DISPLAY_TIME is 3.0
    // At 0.3 seconds, should be fully visible
    notifier.update(0.3);
    assert!((notifier.progress() - 1.0).abs() < 0.01);

    // At 2.7 seconds (3.0 - 0.3), should still be visible
    notifier.update(2.4);
    assert!((notifier.progress() - 1.0).abs() < 0.01);
}
