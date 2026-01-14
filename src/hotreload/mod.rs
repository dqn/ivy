//! Hot reload support for development.
//!
//! Watches scenario files for changes and triggers reload.
//! Only available on native platforms (not WASM).

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use anyhow::Result;
    use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
    use std::collections::HashSet;
    use std::path::PathBuf;
    use std::sync::mpsc::{self, Receiver};

    /// Hot reloader that watches files for changes.
    pub struct HotReloader {
        #[allow(dead_code)]
        watcher: RecommendedWatcher,
        rx: Receiver<Result<Event, notify::Error>>,
        watched_paths: HashSet<PathBuf>,
    }

    impl HotReloader {
        /// Create a new hot reloader.
        pub fn new() -> Result<Self> {
            let (tx, rx) = mpsc::channel();

            let watcher = notify::recommended_watcher(move |res| {
                let _ = tx.send(res);
            })?;

            Ok(Self {
                watcher,
                rx,
                watched_paths: HashSet::new(),
            })
        }

        /// Watch a file or directory for changes.
        pub fn watch(&mut self, path: &str) -> Result<()> {
            let path = PathBuf::from(path);
            if self.watched_paths.contains(&path) {
                return Ok(());
            }

            self.watcher.watch(&path, RecursiveMode::NonRecursive)?;
            self.watched_paths.insert(path);
            Ok(())
        }

        /// Check for file changes. Returns true if any watched file changed.
        pub fn poll(&mut self) -> bool {
            let mut changed = false;
            while let Ok(event) = self.rx.try_recv() {
                if let Ok(event) = event
                    && matches!(
                        event.kind,
                        notify::EventKind::Modify(_) | notify::EventKind::Create(_)
                    ) {
                        changed = true;
                    }
            }
            changed
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::HotReloader;

#[cfg(target_arch = "wasm32")]
pub struct HotReloader;

#[cfg(target_arch = "wasm32")]
impl HotReloader {
    /// Create a new hot reloader (no-op on WASM).
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    /// Watch a file or directory for changes (no-op on WASM).
    pub fn watch(&mut self, _path: &str) -> Result<()> {
        Ok(())
    }

    /// Check for file changes (always false on WASM).
    pub fn poll(&mut self) -> bool {
        false
    }
}
