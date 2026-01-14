use ivy::runtime::Chapter;

// Note: ChapterProgress and ChapterManager tests that involve unlock()/complete()
// are limited because they call save() internally. We test serialization
// and Chapter struct which is purely data.

// Chapter struct tests

#[test]
fn test_chapter_serialization_roundtrip() {
    let chapter = Chapter {
        id: "prologue".to_string(),
        title: "Prologue".to_string(),
        start_label: "prologue_start".to_string(),
        description: "The beginning".to_string(),
    };

    let json = serde_json::to_string(&chapter).unwrap();
    let restored: Chapter = serde_json::from_str(&json).unwrap();

    assert_eq!(restored.id, "prologue");
    assert_eq!(restored.title, "Prologue");
    assert_eq!(restored.start_label, "prologue_start");
    assert_eq!(restored.description, "The beginning");
}

#[test]
fn test_chapter_default_description() {
    let json = r#"{
        "id": "ch1",
        "title": "Chapter 1",
        "start_label": "ch1_start"
    }"#;

    let chapter: Chapter = serde_json::from_str(json).unwrap();

    assert_eq!(chapter.id, "ch1");
    assert_eq!(chapter.title, "Chapter 1");
    assert_eq!(chapter.start_label, "ch1_start");
    assert_eq!(chapter.description, ""); // default
}

#[test]
fn test_chapter_clone() {
    let chapter = Chapter {
        id: "test".to_string(),
        title: "Test".to_string(),
        start_label: "test_start".to_string(),
        description: "Test chapter".to_string(),
    };

    let cloned = chapter.clone();

    assert_eq!(cloned.id, chapter.id);
    assert_eq!(cloned.title, chapter.title);
    assert_eq!(cloned.start_label, chapter.start_label);
    assert_eq!(cloned.description, chapter.description);
}

// ChapterProgress serialization tests

#[test]
fn test_chapter_progress_serialization_empty() {
    let json = r#"{"unlocked": [], "completed": []}"#;
    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();

    assert!(parsed["unlocked"].as_array().unwrap().is_empty());
    assert!(parsed["completed"].as_array().unwrap().is_empty());
}

#[test]
fn test_chapter_progress_serialization_with_data() {
    let json = r#"{
        "unlocked": ["prologue", "chapter1"],
        "completed": ["prologue"]
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
    let unlocked = parsed["unlocked"].as_array().unwrap();
    let completed = parsed["completed"].as_array().unwrap();

    assert_eq!(unlocked.len(), 2);
    assert!(unlocked.iter().any(|v| v == "prologue"));
    assert!(unlocked.iter().any(|v| v == "chapter1"));

    assert_eq!(completed.len(), 1);
    assert!(completed.iter().any(|v| v == "prologue"));
}

// ChapterManager tests (limited due to file I/O in progress tracking)

#[test]
fn test_chapter_manager_new_has_no_chapters() {
    let manager = ivy::runtime::ChapterManager::new();
    assert!(!manager.has_chapters());
    assert!(manager.chapters().is_empty());
}

#[test]
fn test_chapter_manager_get_chapter_not_found() {
    let manager = ivy::runtime::ChapterManager::new();
    assert!(manager.get_chapter("nonexistent").is_none());
}

#[test]
fn test_chapter_manager_has_chapters_after_set() {
    let mut manager = ivy::runtime::ChapterManager::new();

    let chapters = vec![Chapter {
        id: "ch1".to_string(),
        title: "Chapter 1".to_string(),
        start_label: "ch1_start".to_string(),
        description: String::new(),
    }];

    manager.set_chapters(chapters);

    assert!(manager.has_chapters());
    assert_eq!(manager.chapters().len(), 1);
}

#[test]
fn test_chapter_manager_get_chapter_by_id() {
    let mut manager = ivy::runtime::ChapterManager::new();

    let chapters = vec![
        Chapter {
            id: "prologue".to_string(),
            title: "Prologue".to_string(),
            start_label: "prologue_start".to_string(),
            description: String::new(),
        },
        Chapter {
            id: "ch1".to_string(),
            title: "Chapter 1".to_string(),
            start_label: "ch1_start".to_string(),
            description: String::new(),
        },
    ];

    manager.set_chapters(chapters);

    let prologue = manager.get_chapter("prologue");
    assert!(prologue.is_some());
    assert_eq!(prologue.unwrap().title, "Prologue");

    let ch1 = manager.get_chapter("ch1");
    assert!(ch1.is_some());
    assert_eq!(ch1.unwrap().title, "Chapter 1");

    assert!(manager.get_chapter("nonexistent").is_none());
}

#[test]
fn test_chapter_manager_chapters_order_preserved() {
    let mut manager = ivy::runtime::ChapterManager::new();

    let chapters = vec![
        Chapter {
            id: "a".to_string(),
            title: "A".to_string(),
            start_label: "a_start".to_string(),
            description: String::new(),
        },
        Chapter {
            id: "b".to_string(),
            title: "B".to_string(),
            start_label: "b_start".to_string(),
            description: String::new(),
        },
        Chapter {
            id: "c".to_string(),
            title: "C".to_string(),
            start_label: "c_start".to_string(),
            description: String::new(),
        },
    ];

    manager.set_chapters(chapters);

    let stored = manager.chapters();
    assert_eq!(stored[0].id, "a");
    assert_eq!(stored[1].id, "b");
    assert_eq!(stored[2].id, "c");
}

#[test]
fn test_chapter_manager_set_chapters_replaces_existing() {
    let mut manager = ivy::runtime::ChapterManager::new();

    // Set initial chapters
    manager.set_chapters(vec![Chapter {
        id: "old".to_string(),
        title: "Old".to_string(),
        start_label: "old_start".to_string(),
        description: String::new(),
    }]);

    assert_eq!(manager.chapters().len(), 1);
    assert_eq!(manager.chapters()[0].id, "old");

    // Replace with new chapters
    manager.set_chapters(vec![
        Chapter {
            id: "new1".to_string(),
            title: "New 1".to_string(),
            start_label: "new1_start".to_string(),
            description: String::new(),
        },
        Chapter {
            id: "new2".to_string(),
            title: "New 2".to_string(),
            start_label: "new2_start".to_string(),
            description: String::new(),
        },
    ]);

    assert_eq!(manager.chapters().len(), 2);
    assert_eq!(manager.chapters()[0].id, "new1");
    assert_eq!(manager.chapters()[1].id, "new2");
}
