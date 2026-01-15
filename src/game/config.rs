use macroquad::prelude::*;

pub const SCENARIO_PATH: &str = "assets/sample.yaml";
pub const QUICK_SAVE_PATH: &str = "saves/save.json";
pub const FONT_PATH: &str = "assets/fonts/NotoSansJP-Regular.ttf";

pub fn window_conf() -> Conf {
    Conf {
        window_title: "ivy".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}
