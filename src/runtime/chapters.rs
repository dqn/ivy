use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::platform;

const CHAPTERS_PATH: &str = "saves/chapters.json";

/// Chapter definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    /// Unique chapter ID.
    pub id: String,
    /// Chapter title displayed in menu.
    pub title: String,
    /// Label to jump to when starting this chapter.
    pub start_label: String,
    /// Optional description.
    #[serde(default)]
    pub description: String,
}

/// Chapter progress tracking.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChapterProgress {
    /// Set of unlocked chapter IDs.
    unlocked: HashSet<String>,
    /// Set of completed chapter IDs.
    completed: HashSet<String>,
}

impl ChapterProgress {
    /// Load chapter progress from file.
    pub fn load() -> Self {
        if !platform::file_exists(CHAPTERS_PATH) {
            return Self::default();
        }

        match platform::read_file(CHAPTERS_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save chapter progress to file.
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = platform::write_file(CHAPTERS_PATH, &json);
        }
    }

    /// Check if a chapter is unlocked.
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains(id)
    }

    /// Check if a chapter is completed.
    pub fn is_completed(&self, id: &str) -> bool {
        self.completed.contains(id)
    }

    /// Unlock a chapter.
    pub fn unlock(&mut self, id: &str) {
        self.unlocked.insert(id.to_string());
        self.save();
    }

    /// Mark a chapter as completed.
    pub fn complete(&mut self, id: &str) {
        self.completed.insert(id.to_string());
        self.save();
    }

    /// Get unlocked chapter count.
    pub fn unlocked_count(&self) -> usize {
        self.unlocked.len()
    }

    /// Get completed chapter count.
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
}

/// Chapter manager that holds chapter definitions.
#[derive(Debug, Clone, Default)]
pub struct ChapterManager {
    /// List of chapters.
    chapters: Vec<Chapter>,
    /// Progress tracking.
    progress: ChapterProgress,
}

impl ChapterManager {
    /// Create a new chapter manager.
    pub fn new() -> Self {
        Self {
            chapters: Vec::new(),
            progress: ChapterProgress::load(),
        }
    }

    /// Set chapters from scenario definition.
    pub fn set_chapters(&mut self, chapters: Vec<Chapter>) {
        self.chapters = chapters;

        // First chapter is always unlocked
        if let Some(first) = self.chapters.first() {
            self.progress.unlock(&first.id);
        }
    }

    /// Get all chapters.
    pub fn chapters(&self) -> &[Chapter] {
        &self.chapters
    }

    /// Get a chapter by ID.
    pub fn get_chapter(&self, id: &str) -> Option<&Chapter> {
        self.chapters.iter().find(|c| c.id == id)
    }

    /// Check if a chapter is unlocked.
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.progress.is_unlocked(id)
    }

    /// Check if a chapter is completed.
    pub fn is_completed(&self, id: &str) -> bool {
        self.progress.is_completed(id)
    }

    /// Unlock a chapter.
    pub fn unlock(&mut self, id: &str) {
        self.progress.unlock(id);
    }

    /// Mark a chapter as completed and unlock the next chapter.
    pub fn complete(&mut self, id: &str) {
        self.progress.complete(id);

        // Find the current chapter index and unlock the next one
        if let Some(idx) = self.chapters.iter().position(|c| c.id == id)
            && let Some(next) = self.chapters.get(idx + 1) {
            self.progress.unlock(&next.id);
            }
    }

    /// Check if chapters are defined.
    pub fn has_chapters(&self) -> bool {
        !self.chapters.is_empty()
    }
}
