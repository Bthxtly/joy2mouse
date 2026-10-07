use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Config {
    pub misc: Misc,
    pub buttons: Buttons,
    pub cursor: Cursor,
    pub wheel: Wheel,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Misc {
    pub device_name: String,
    pub virtual_divece_name: Option<String>,
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
    pub fn new(path: &str) -> Self {
        let content =
            std::fs::read_to_string(path).unwrap_or_else(|e| panic!("Failed to read {path}: {e}"));

        toml::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse {path}: {e}"))
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
