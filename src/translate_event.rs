use evdev::{EventType, InputEvent, RelativeAxisCode};

// evtest events
const EV_SYN: u16 = EventType::SYNCHRONIZATION.0;
const EV_KEY: u16 = EventType::KEY.0;
const EV_ABS: u16 = EventType::ABSOLUTE.0;
const EV_FF: u16 = EventType::FORCEFEEDBACK.0;

// gamepad buttons
const BTN_SOUTH: u16 = 304;
const BTN_EAST: u16 = 305;
const BTN_NORTH: u16 = 307;
const BTN_WEST: u16 = 308;
const BTN_TL: u16 = 310;
const BTN_TR: u16 = 311;
const BTN_SELECT: u16 = 314;
const BTN_START: u16 = 315;
const BTN_MODE: u16 = 316;
const BTN_THUMBL: u16 = 317;
const BTN_THUMBR: u16 = 318;

// gamepad joystick
const ABS_X: u16 = 0;
const ABS_Y: u16 = 1;
const ABS_RX: u16 = 3;
const ABS_RY: u16 = 4;
const ABS_Z: u16 = 2;
const ABS_RZ: u16 = 5;
const ABS_HAT0X: u16 = 16;
const ABS_HAT0Y: u16 = 17;

// mouse buttons
const MOUSE_LEFT: u16 = 272;
const MOUSE_RIGHT: u16 = 273;
const MOUSE_MIDDLE: u16 = 274;
#[allow(dead_code)]
const MOUSE_SIDE: u16 = 275;
#[allow(dead_code)]
const MOUSE_EXTRA: u16 = 276;

// keyboard keys
const KEY_UP: u16 = 103;
const KEY_LEFT: u16 = 105;
const KEY_RIGHT: u16 = 106;
const KEY_DOWN: u16 = 108;
const KEY_LEFT_META: u16 = 125;

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
        let et_key = EventType::KEY.0;

        let event_type = &joy_event.event_type();
        let code = joy_event.code();
        let value = joy_event.value();

        // evdev uses its own enumerator
        match event_type.0 {
            EV_KEY => {
                let key = match code {
                    BTN_SOUTH => None,
                    BTN_EAST => None,
                    BTN_NORTH => None,
                    BTN_WEST => None,
                    BTN_TL => Some(MOUSE_LEFT),
                    BTN_TR => Some(MOUSE_RIGHT),
                    BTN_SELECT => Some(KEY_LEFT_META),
                    BTN_START => None,
                    BTN_MODE => None,
                    BTN_THUMBL => None,
                    BTN_THUMBR => Some(MOUSE_MIDDLE),
                    _ => None,
                };
                click_event = key.map(|key| InputEvent::new_now(et_key, key, value));
            }

            EV_ABS => {
                let distance = if value.abs() < 16 { 0 } else { value >> 12 };
                // reverse scroll direction and decrease sensitivity
                let scroll_distance = -distance >> 1;
                match code {
                    ABS_X => self.cursor_move_distance_x = distance,
                    ABS_Y => self.cursor_move_distance_y = distance,
                    ABS_RX => self.wheel_scroll_distance_x = scroll_distance,
                    ABS_RY => self.wheel_scroll_distance_y = scroll_distance,
                    ABS_Z => {}
                    ABS_RZ => {}
                    ABS_HAT0X => {
                        match value {
                            -1 => {
                                self.hat0x_key = KEY_LEFT;
                            }
                            1 => {
                                self.hat0x_key = KEY_RIGHT;
                            }
                            _ => {}
                        }
                        click_event = Some(InputEvent::new_now(et_key, self.hat0x_key, value.abs()))
                    }
                    ABS_HAT0Y => {
                        match value {
                            -1 => {
                                self.hat0y_key = KEY_UP;
                            }
                            1 => {
                                self.hat0y_key = KEY_DOWN;
                            }
                            _ => {}
                        }
                        click_event = Some(InputEvent::new_now(et_key, self.hat0y_key, value.abs()))
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
