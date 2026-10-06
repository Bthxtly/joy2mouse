use evdev::{EventType, InputEvent, RelativeAxisCode};

mod constants;
use constants::*;

#[derive(Default, Debug)]
pub struct Joy2mouse {
    cursor_move_distance_x: i32,
    cursor_move_distance_y: i32,
    wheel_scroll_distance_x: i32,
    wheel_scroll_distance_y: i32,
    hat0x_key: u16,
    hat0y_key: u16,
}

impl Joy2mouse {
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
        let mut click_event = None;

        let event_type = &joy_event.event_type();
        let code = joy_event.code();
        let value = joy_event.value();

        // evdev uses its own enumerator
        match event_type.0 {
            EV_KEY => {
                let key = match code {
                    BTN_SOUTH => Some(KEY_SPACE),
                    BTN_EAST => Some(KEY_NEXTSONG),
                    BTN_NORTH => Some(KEY_PREVIOUSSONG),
                    BTN_WEST => Some(KEY_PLAYPAUSE),
                    BTN_TL => Some(MOUSE_LEFT),
                    BTN_TR => Some(MOUSE_RIGHT),
                    BTN_SELECT => Some(KEY_LEFT_META),
                    BTN_START => None,
                    BTN_MODE => None,
                    BTN_THUMBL => None,
                    BTN_THUMBR => Some(MOUSE_MIDDLE),
                    _ => None,
                };
                click_event = key.map(|key| InputEvent::new_now(EV_KEY, key, value));
            }

            EV_ABS => {
                let on = value.clamp(0, 1);
                let distance = if value.abs() < 16 { 0 } else { value >> 12 };
                // decrease sensitivity
                let scroll_distance = distance >> 1;
                match code {
                    ABS_X => self.cursor_move_distance_x = distance,
                    ABS_Y => self.cursor_move_distance_y = distance,
                    ABS_RX => self.wheel_scroll_distance_x = scroll_distance,
                    ABS_RY => self.wheel_scroll_distance_y = -scroll_distance, // natural scroll
                    ABS_Z => click_event = Some(InputEvent::new_now(EV_KEY, KEY_LEFTSHIFT, on)),
                    ABS_RZ => click_event = Some(InputEvent::new_now(EV_KEY, KEY_LEFTALT, on)),
                    ABS_HAT0X => {
                        match value {
                            -1 => self.hat0x_key = KEY_LEFT,
                            1 => self.hat0x_key = KEY_RIGHT,
                            _ => {}
                        }
                        click_event = Some(InputEvent::new_now(EV_KEY, self.hat0x_key, value.abs()))
                    }
                    ABS_HAT0Y => {
                        match value {
                            -1 => self.hat0y_key = KEY_UP,
                            1 => self.hat0y_key = KEY_DOWN,
                            _ => {}
                        }
                        click_event = Some(InputEvent::new_now(EV_KEY, self.hat0y_key, value.abs()))
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
