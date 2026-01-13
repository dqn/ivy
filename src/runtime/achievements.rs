use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::platform;

const ACHIEVEMENTS_PATH: &str = "saves/achievements.json";

/// Achievement unlock state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Achievements {
    /// Set of unlocked achievement IDs.
    unlocked: HashSet<String>,
}

impl Achievements {
    /// Load achievements from storage.
    pub fn load() -> Self {
        if !platform::file_exists(ACHIEVEMENTS_PATH) {
            return Self::default();
        }

        match platform::read_file(ACHIEVEMENTS_PATH) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    /// Save achievements to storage.
    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = platform::write_file(ACHIEVEMENTS_PATH, &json);
        }
    }

    /// Unlock an achievement. Returns true if newly unlocked.
    pub fn unlock(&mut self, id: &str) -> bool {
        if self.unlocked.contains(id) {
            return false;
        }
        self.unlocked.insert(id.to_string());
        self.save();
        true
    }

    /// Check if an achievement is unlocked.
    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains(id)
    }

    /// Get all unlocked achievement IDs.
    pub fn unlocked_ids(&self) -> Vec<String> {
        self.unlocked.iter().cloned().collect()
    }

    /// Get the number of unlocked achievements.
    pub fn unlocked_count(&self) -> usize {
        self.unlocked.len()
    }
}

/// Notification for a newly unlocked achievement.
#[derive(Debug, Clone)]
pub struct AchievementNotification {
    /// Achievement ID.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Description.
    pub description: String,
    /// Display timer.
    timer: f32,
    /// Animation progress (0 to 1).
    progress: f32,
}

/// Achievement notification manager.
#[derive(Debug, Default)]
pub struct AchievementNotifier {
    /// Queue of notifications to display.
    queue: Vec<AchievementNotification>,
    /// Currently displaying notification.
    current: Option<AchievementNotification>,
}

impl AchievementNotifier {
    /// Duration to show each notification.
    const DISPLAY_TIME: f32 = 3.0;
    /// Animation duration for slide in/out.
    const ANIM_TIME: f32 = 0.3;

    /// Queue a new achievement notification.
    pub fn notify(&mut self, id: &str, name: &str, description: &str) {
        self.queue.push(AchievementNotification {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            timer: 0.0,
            progress: 0.0,
        });
    }

    /// Update notification state.
    pub fn update(&mut self, dt: f32) {
        // If no current notification, try to get one from queue
        if self.current.is_none() && !self.queue.is_empty() {
            self.current = Some(self.queue.remove(0));
        }

        // Update current notification
        if let Some(ref mut notif) = self.current {
            notif.timer += dt;

            // Calculate animation progress
            if notif.timer < Self::ANIM_TIME {
                // Slide in
                notif.progress = notif.timer / Self::ANIM_TIME;
            } else if notif.timer < Self::DISPLAY_TIME - Self::ANIM_TIME {
                // Fully visible
                notif.progress = 1.0;
            } else if notif.timer < Self::DISPLAY_TIME {
                // Slide out
                notif.progress = (Self::DISPLAY_TIME - notif.timer) / Self::ANIM_TIME;
            } else {
                // Done
                self.current = None;
            }
        }
    }

    /// Get current notification for rendering (if any).
    pub fn current(&self) -> Option<&AchievementNotification> {
        self.current.as_ref()
    }

    /// Get animation progress (0 to 1) for current notification.
    pub fn progress(&self) -> f32 {
        self.current.as_ref().map(|n| n.progress).unwrap_or(0.0)
    }
}
