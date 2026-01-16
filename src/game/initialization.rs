use std::collections::HashMap;

use macroquad::prelude::*;

use crate::audio::AudioManager;
use crate::cache::TextureCache;
use crate::flowchart::{Flowchart, LayoutConfig, NodeId, NodeLayout};
use crate::hotreload::HotReloader;
use crate::i18n::LanguageConfig;
use crate::input::GamepadState;
use crate::render::{
    AchievementConfig, BacklogConfig, BacklogState, ChapterSelectConfig, ChapterSelectState,
    CharAnimationState, CharIdleState, ChoiceButtonConfig, ChoiceNavState, CinematicState,
    DebugConfig, DebugState, FlowchartConfig, FlowchartState, GalleryConfig, GalleryState,
    GameSettings, InputConfig, InputState, NvlConfig, NvlState, ParticleState, SettingsConfig,
    ShakeState, TextBoxConfig, TitleConfig, TransitionState, TypewriterState, VideoBackgroundState,
    VideoState,
};
use crate::runtime::{
    AchievementNotifier, Achievements, CameraAnimationState, CameraState, Chapter, ChapterManager,
    GameState, ReadState, Unlocks,
};
use crate::scenario::{CharPosition, ModularCharDef, Scenario, load_scenario};

use super::{FONT_PATH, SCENARIO_PATH};

/// All game state and configuration bundled together.
pub struct GameContext {
    // Scenario data
    pub scenario: Scenario,
    pub scenario_title: String,
    pub modular_char_defs: HashMap<String, ModularCharDef>,

    // Core game state
    pub game_state: Option<GameState>,
    pub last_index: Option<usize>,

    // Mode flags
    pub auto_mode: bool,
    pub auto_timer: f64,
    pub skip_mode: bool,
    pub show_backlog: bool,

    // Hot reloader
    pub hot_reloader: Option<HotReloader>,

    // Settings
    pub settings: GameSettings,

    // Configs (immutable after initialization)
    pub title_config: TitleConfig,
    pub settings_config: SettingsConfig,
    pub base_text_config: TextBoxConfig,
    pub choice_config: ChoiceButtonConfig,
    pub backlog_config: BacklogConfig,
    pub input_config: InputConfig,
    pub gallery_config: GalleryConfig,
    pub achievement_config: AchievementConfig,
    pub debug_config: DebugConfig,
    pub chapter_select_config: ChapterSelectConfig,
    pub flowchart_config: FlowchartConfig,
    pub layout_config: LayoutConfig,
    pub nvl_config: NvlConfig,

    // UI States
    pub backlog_state: BacklogState,
    pub debug_state: DebugState,
    pub input_state: InputState,
    pub gallery_state: GalleryState,
    pub chapter_select_state: ChapterSelectState,
    pub flowchart_state: FlowchartState,
    pub flowchart_cache: Option<(Flowchart, HashMap<NodeId, NodeLayout>)>,

    // Game systems
    pub chapter_manager: ChapterManager,
    pub unlocks: Unlocks,
    pub achievements: Achievements,
    pub read_state: ReadState,
    pub achievement_notifier: AchievementNotifier,
    pub language_config: LanguageConfig,

    // Resource management
    pub texture_cache: TextureCache,
    pub audio_manager: AudioManager,
    pub gamepad_state: GamepadState,

    // Animation and visual states
    pub transition_state: TransitionState,
    pub shake_state: ShakeState,
    pub typewriter_state: TypewriterState,
    pub last_text: Option<String>,

    // Wait state
    pub wait_timer: f32,
    pub in_wait: bool,

    // Character animation states (single character)
    pub char_anim_state: CharAnimationState,
    pub char_idle_state: CharIdleState,
    pub pending_idle: Option<crate::scenario::CharIdleAnimation>,

    // Character animation states (multiple characters)
    pub char_anim_states: HashMap<CharPosition, CharAnimationState>,
    pub char_idle_states: HashMap<CharPosition, CharIdleState>,
    pub pending_idles: HashMap<CharPosition, crate::scenario::CharIdleAnimation>,

    // Particle and cinematic states
    pub particle_state: ParticleState,
    pub cinematic_state: CinematicState,

    // Video states
    pub video_state: VideoState,
    pub video_bg_state: VideoBackgroundState,

    // NVL mode state
    pub nvl_state: NvlState,

    // Choice state
    pub choice_timer: Option<f32>,
    pub choice_total_time: Option<f32>,
    pub choice_nav_state: ChoiceNavState,
    pub last_mouse_pos: (f32, f32),

    // Camera state
    pub camera_state: CameraState,
    pub camera_anim_state: CameraAnimationState,

    // Input state
    pub awaiting_input: Option<String>,
}

impl GameContext {
    /// Initialize all game state and configurations.
    /// Returns (GameContext, Option<Font>) - font is separate to avoid borrow conflicts.
    pub async fn new() -> anyhow::Result<(Self, Option<Font>)> {
        // Load scenario
        let scenario = load_scenario(SCENARIO_PATH)?;

        // Initialize hot reloader for development
        let hot_reloader = match HotReloader::new() {
            Ok(mut reloader) => {
                if let Err(e) = reloader.watch(SCENARIO_PATH) {
                    eprintln!("Failed to watch scenario file: {}", e);
                } else {
                    eprintln!("[Hot Reload] Watching {}", SCENARIO_PATH);
                }
                Some(reloader)
            }
            Err(e) => {
                eprintln!("Hot reload disabled: {}", e);
                None
            }
        };

        let scenario_title = scenario.title.clone();
        let scenario_chapters: Vec<Chapter> = scenario
            .chapters
            .iter()
            .map(|c| Chapter {
                id: c.id.clone(),
                title: c.title.clone(),
                start_label: c.start_label.clone(),
                description: c.description.clone(),
            })
            .collect();
        let modular_char_defs = scenario.modular_characters.clone();
        eprintln!("Loaded scenario: {}", scenario_title);

        // Load custom font for Japanese text support
        let custom_font = match load_ttf_font(FONT_PATH).await {
            Ok(font) => {
                eprintln!("Loaded custom font from {}", FONT_PATH);
                Some(font)
            }
            Err(e) => {
                eprintln!("Custom font not found ({}), using default font", e);
                None
            }
        };

        // Load settings
        let settings = GameSettings::load();
        eprintln!(
            "Loaded settings: BGM={:.0}%, SE={:.0}%, Voice={:.0}%, Auto={:.1}x",
            settings.bgm_volume * 100.0,
            settings.se_volume * 100.0,
            settings.voice_volume * 100.0,
            settings.auto_speed
        );

        // Initialize chapter manager
        let mut chapter_manager = ChapterManager::new();
        chapter_manager.set_chapters(scenario_chapters);

        Ok((
            Self {
                // Scenario data
                scenario,
                scenario_title,
                modular_char_defs,

                // Core game state
                game_state: None,
                last_index: None,

                // Mode flags
                auto_mode: false,
                auto_timer: 0.0,
                skip_mode: false,
                show_backlog: false,

                // Hot reloader
                hot_reloader,

                // Settings
                settings,

                // Configs
                title_config: TitleConfig::default(),
                settings_config: SettingsConfig::default(),
                base_text_config: TextBoxConfig::default(),
                choice_config: ChoiceButtonConfig::default(),
                backlog_config: BacklogConfig::default(),
                input_config: InputConfig::default(),
                gallery_config: GalleryConfig::default(),
                achievement_config: AchievementConfig::default(),
                debug_config: DebugConfig::default(),
                chapter_select_config: ChapterSelectConfig::default(),
                flowchart_config: FlowchartConfig::default(),
                layout_config: LayoutConfig::default(),
                nvl_config: NvlConfig::default(),

                // UI States
                backlog_state: BacklogState::default(),
                debug_state: DebugState::default(),
                input_state: InputState::default(),
                gallery_state: GalleryState::default(),
                chapter_select_state: ChapterSelectState::default(),
                flowchart_state: FlowchartState::new(),
                flowchart_cache: None,

                // Game systems
                chapter_manager,
                unlocks: Unlocks::load(),
                achievements: Achievements::load(),
                read_state: ReadState::load(),
                achievement_notifier: AchievementNotifier::default(),
                language_config: LanguageConfig::default(),

                // Resource management
                texture_cache: TextureCache::new(),
                audio_manager: AudioManager::new(),
                gamepad_state: GamepadState::new(),

                // Animation and visual states
                transition_state: TransitionState::default(),
                shake_state: ShakeState::default(),
                typewriter_state: TypewriterState::default(),
                last_text: None,

                // Wait state
                wait_timer: 0.0,
                in_wait: false,

                // Character animation states (single character)
                char_anim_state: CharAnimationState::default(),
                char_idle_state: CharIdleState::default(),
                pending_idle: None,

                // Character animation states (multiple characters)
                char_anim_states: HashMap::new(),
                char_idle_states: HashMap::new(),
                pending_idles: HashMap::new(),

                // Particle and cinematic states
                particle_state: ParticleState::default(),
                cinematic_state: CinematicState::default(),

                // Video states
                video_state: VideoState::new(),
                video_bg_state: VideoBackgroundState::new(),

                // NVL mode state
                nvl_state: NvlState::new(),

                // Choice state
                choice_timer: None,
                choice_total_time: None,
                choice_nav_state: ChoiceNavState::default(),
                last_mouse_pos: (0.0, 0.0),

                // Camera state
                camera_state: CameraState::default(),
                camera_anim_state: CameraAnimationState::default(),

                // Input state
                awaiting_input: None,
            },
            custom_font,
        ))
    }

    /// Get text config with accessibility settings applied.
    pub fn text_config(&self) -> TextBoxConfig {
        self.base_text_config.with_accessibility(
            self.settings.accessibility.font_scale_multiplier(),
            self.settings.accessibility.line_spacing,
            self.settings.accessibility.high_contrast,
        )
    }

    /// Start a new game.
    pub fn start_new_game(&mut self) -> anyhow::Result<()> {
        let new_scenario = load_scenario(SCENARIO_PATH)?;
        self.game_state = Some(GameState::new(new_scenario));
        self.reset_game_state();
        Ok(())
    }

    /// Start game from a specific chapter.
    pub fn start_from_chapter(&mut self, start_label: &str) -> anyhow::Result<()> {
        let new_scenario = load_scenario(SCENARIO_PATH)?;
        let mut new_state = GameState::new(new_scenario);
        new_state.jump_to_label(start_label);
        self.game_state = Some(new_state);
        self.reset_game_state();
        Ok(())
    }

    /// Reset game state for a new game or loaded game.
    pub fn reset_game_state(&mut self) {
        self.last_index = None;
        self.auto_mode = false;
        self.skip_mode = false;
        self.show_backlog = false;
    }

    /// Reload scenario (for hot reload).
    pub fn reload_scenario(&mut self, new_scenario: Scenario) {
        self.modular_char_defs = new_scenario.modular_characters.clone();
        self.scenario = new_scenario.clone();
        if let Some(ref mut state) = self.game_state {
            state.reload_scenario(new_scenario);
        }
        self.last_index = None;
        self.flowchart_state.dirty = true;
    }
}
