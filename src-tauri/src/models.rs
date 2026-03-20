use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AwakeMode {
    PreventSleep,
    JiggleCursor,
    Smart,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MovementPattern {
    Line,
    Square,
    Circle,
    ZigZag,
}

fn default_mode() -> AwakeMode {
    AwakeMode::Smart
}

fn default_interval_seconds() -> u64 {
    60
}

fn default_jiggle_pixels() -> i32 {
    10
}

fn default_movement_duration_ms() -> u64 {
    400
}

fn default_movement_pattern() -> MovementPattern {
    MovementPattern::Line
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default = "default_mode")]
    pub mode: AwakeMode,

    #[serde(default = "default_interval_seconds")]
    pub interval_seconds: u64,

    #[serde(default = "default_jiggle_pixels")]
    pub jiggle_pixels: i32,

    #[serde(default = "default_movement_duration_ms")]
    pub movement_duration_ms: u64,

    #[serde(default = "default_movement_pattern")]
    pub movement_pattern: MovementPattern,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            mode: default_mode(),
            interval_seconds: default_interval_seconds(),
            jiggle_pixels: default_jiggle_pixels(),
            movement_duration_ms: default_movement_duration_ms(),
            movement_pattern: default_movement_pattern(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStatus {
    pub is_running: bool,
    pub active_mode: AwakeMode,
}

impl Default for AppStatus {
    fn default() -> Self {
        Self {
            is_running: false,
            active_mode: AwakeMode::Smart,
        }
    }
}