use directories::ProjectDirs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Config {
    pub misc: Misc,
    pub buttons: Buttons,
    pub cursor: Cursor,
    pub wheel: Wheel,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Misc {
    pub device_name: String,
    pub virtual_divece_name: String,
    pub toggle_keys: Vec<u16>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Buttons {
    pub btn_south: Option<u16>,
    pub btn_east: Option<u16>,
    pub btn_north: Option<u16>,
    pub btn_west: Option<u16>,
    pub btn_tl: Option<u16>,
    pub btn_tr: Option<u16>,
    pub btn_select: Option<u16>,
    pub btn_start: Option<u16>,
    pub btn_mode: Option<u16>,
    pub btn_thumbl: Option<u16>,
    pub btn_thumbr: Option<u16>,
    pub abs_z: Option<u16>,
    pub abs_rz: Option<u16>,
    pub hat0_left: Option<u16>,
    pub hat0_right: Option<u16>,
    pub hat0_up: Option<u16>,
    pub hat0_down: Option<u16>,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Cursor {
    pub dead_zone: i32,
    pub _sensitivity: i32,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Wheel {
    pub x_reverse: bool,
    pub y_reverse: bool,
}

impl Config {
    pub fn load_config() -> Self {
        let Some(path) = Self::config_path() else {
            eprintln!("TODO");
            return Self::default();
        };

        if !path.exists() {
            eprintln!("Can not find configure file, using default settings.");
            return Self::default();
        }

        eprintln!("Reading configure file: {}", path.display());
        let content =
            std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("Fail to read config: {}", e));

        toml::from_str(&content).unwrap_or_else(|e| panic!("Fail to convert toml: {}", e.message()))
    }

    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("", "", "joy2mouse").map(|dirs| dirs.config_dir().join("config.toml"))
    }

    pub fn keys(&self) -> Vec<u16> {
        [
            self.buttons.btn_south,
            self.buttons.btn_east,
            self.buttons.btn_north,
            self.buttons.btn_west,
            self.buttons.btn_tl,
            self.buttons.btn_tr,
            self.buttons.btn_select,
            self.buttons.btn_start,
            self.buttons.btn_mode,
            self.buttons.btn_thumbl,
            self.buttons.btn_thumbr,
            self.buttons.abs_z,
            self.buttons.abs_rz,
            self.buttons.hat0_left,
            self.buttons.hat0_right,
            self.buttons.hat0_up,
            self.buttons.hat0_down,
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }
}

impl Default for Misc {
    fn default() -> Self {
        Self {
            device_name: "gamepad".to_string(),
            virtual_divece_name: "virtual mouse".to_string(),
            toggle_keys: Vec::default(),
        }
    }
}
