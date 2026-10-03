use evdev::EventType;

// evtest events
pub const EV_SYN: u16 = EventType::SYNCHRONIZATION.0;
pub const EV_KEY: u16 = EventType::KEY.0;
pub const EV_ABS: u16 = EventType::ABSOLUTE.0;
pub const EV_FF: u16 = EventType::FORCEFEEDBACK.0;

// gamepad buttons
pub const BTN_SOUTH: u16 = 304;
pub const BTN_EAST: u16 = 305;
pub const BTN_NORTH: u16 = 307;
pub const BTN_WEST: u16 = 308;
pub const BTN_TL: u16 = 310;
pub const BTN_TR: u16 = 311;
pub const BTN_SELECT: u16 = 314;
pub const BTN_START: u16 = 315;
pub const BTN_MODE: u16 = 316;
pub const BTN_THUMBL: u16 = 317;
pub const BTN_THUMBR: u16 = 318;

// gamepad joystick
pub const ABS_X: u16 = 0;
pub const ABS_Y: u16 = 1;
pub const ABS_RX: u16 = 3;
pub const ABS_RY: u16 = 4;
pub const ABS_Z: u16 = 2;
pub const ABS_RZ: u16 = 5;
pub const ABS_HAT0X: u16 = 16;
pub const ABS_HAT0Y: u16 = 17;
