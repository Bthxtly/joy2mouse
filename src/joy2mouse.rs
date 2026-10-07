use crate::{config::Config, constants::*};
use evdev::{EventType, InputEvent, RelativeAxisCode};

#[derive(Default, Debug)]
pub struct Joy2mouse {
    config: Config,
    on: bool,
    toggle_keys_ispressed: Vec<bool>,
    cursor_move_distance_x: i32,
    cursor_move_distance_y: i32,
    wheel_scroll_distance_x: i32,
    wheel_scroll_distance_y: i32,
    hat0x_key: Option<u16>,
    hat0y_key: Option<u16>,
}

impl Joy2mouse {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            on: true,
            toggle_keys_ispressed: vec![false; config.misc.toggle_keys.len()],
            ..Default::default()
        }
    }

    pub fn on(&self) -> bool {
        self.on
    }

    pub fn yield_movement_event(&self) -> (InputEvent, InputEvent) {
        (
            InputEvent::new_now(
                EventType::RELATIVE.0,
                RelativeAxisCode::REL_X.0,
                self.cursor_move_distance_x,
            ),
            InputEvent::new_now(
                EventType::RELATIVE.0,
                RelativeAxisCode::REL_Y.0,
                self.cursor_move_distance_y,
            ),
        )
    }

    pub fn yield_scroll_movement(&self) -> (InputEvent, InputEvent) {
        (
            InputEvent::new_now(
                EventType::RELATIVE.0,
                RelativeAxisCode::REL_HWHEEL.0,
                self.wheel_scroll_distance_x,
            ),
            InputEvent::new_now(
                EventType::RELATIVE.0,
                RelativeAxisCode::REL_WHEEL.0,
                self.wheel_scroll_distance_y,
            ),
        )
    }

    pub fn translate_joy_event(&mut self, joy_event: InputEvent) -> Option<InputEvent> {
        let event_type = &joy_event.event_type();
        let code = joy_event.code();
        let value = joy_event.value();

        // if all keys in Config.misc.toggle_keys are pressed, then toggle self
        for (i, key) in self.config.misc.toggle_keys.iter().enumerate() {
            if *key == code {
                self.toggle_keys_ispressed[i] = value != 0;
            }
            if self.toggle_keys_ispressed.iter().all(|t| *t) {
                self.on = !self.on;
                self.toggle_keys_ispressed
                    .iter_mut()
                    .for_each(|t| *t = false);
            }
        }
        if !self.on() {
            return None;
        }

        let mut click_event = None;
        let buttons = &self.config.buttons;
        match event_type.0 {
            EV_KEY => {
                let key = match code {
                    BTN_SOUTH => buttons.btn_south,
                    BTN_EAST => buttons.btn_east,
                    BTN_NORTH => buttons.btn_north,
                    BTN_WEST => buttons.btn_west,
                    BTN_TL => buttons.btn_tl,
                    BTN_TR => buttons.btn_tr,
                    BTN_SELECT => buttons.btn_select,
                    BTN_START => buttons.btn_start,
                    BTN_MODE => buttons.btn_mode,
                    BTN_THUMBL => buttons.btn_thumbl,
                    BTN_THUMBR => buttons.btn_thumbr,
                    _ => None,
                };
                click_event = key.map(|key| InputEvent::new_now(EV_KEY, key, value));
            }
            EV_ABS => {
                let distance = if value.abs() < self.config.cursor.dead_zone {
                    0
                } else {
                    value >> 12
                };
                // decrease scrollment sensitivity
                let scroll_distance = distance >> 1;

                // reverse vertical and horizontal scrollment respectively
                let xr = if self.config.wheel.x_reverse { -1 } else { 1 };
                let yr = if self.config.wheel.y_reverse { -1 } else { 1 };

                // whether shoulders are pressed or not
                let on = value.clamp(0, 1);

                match code {
                    ABS_X => self.cursor_move_distance_x = distance,
                    ABS_Y => self.cursor_move_distance_y = distance,
                    ABS_RX => self.wheel_scroll_distance_x = scroll_distance * xr,
                    ABS_RY => self.wheel_scroll_distance_y = scroll_distance * yr,
                    ABS_Z => {
                        click_event = buttons
                            .abs_z
                            .map(|key| InputEvent::new_now(EV_KEY, key, on))
                    }
                    ABS_RZ => {
                        click_event = buttons
                            .abs_rz
                            .map(|key| InputEvent::new_now(EV_KEY, key, on))
                    }
                    ABS_HAT0X => {
                        match value {
                            -1 => self.hat0x_key = buttons.hat0_left,
                            1 => self.hat0x_key = buttons.hat0_right,
                            _ => {}
                        }
                        click_event = self
                            .hat0x_key
                            .map(|key| InputEvent::new_now(EV_KEY, key, value.abs()))
                    }
                    ABS_HAT0Y => {
                        match value {
                            -1 => self.hat0y_key = buttons.hat0_up,
                            1 => self.hat0y_key = buttons.hat0_down,
                            _ => {}
                        }
                        click_event = self
                            .hat0y_key
                            .map(|key| InputEvent::new_now(EV_KEY, key, value.abs()))
                    }
                    _ => unreachable!("gamepads don't generate this code: {code}"),
                };
            }
            EV_FF => todo!(),
            EV_SYN => { /* ignore */ }
            _ => unreachable!("gamepads don't generate this event: {event_type:?}"),
        }

        click_event
    }
}
