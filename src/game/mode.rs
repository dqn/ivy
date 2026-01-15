/// Game mode: title screen, settings, gallery, chapters, flowchart, or in-game.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum GameMode {
    #[default]
    Title,
    Settings,
    Gallery,
    Chapters,
    Flowchart,
    InGame,
}
