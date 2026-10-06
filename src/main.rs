// Create a virtual mouse, just while this is running.
// Generally this requires root.

use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

use evdev::{AttributeSet, KeyCode, RelativeAxisCode, uinput::VirtualDevice};

mod translate_event;
use translate_event::*;

pub fn pick_device() -> evdev::Device {
    if let Some(dev_file) = std::env::args_os().nth(2) {
        evdev::Device::open(dev_file).unwrap()
    } else {
        evdev::enumerate()
            .filter(|t| {
                t.1.name()
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains("microsoft")
            })
            .map(|t| t.1)
            .last()
            .expect("no joystick device found")
    }
}

fn build_device(name: &str) -> std::io::Result<VirtualDevice> {
    VirtualDevice::builder()?
        .name(name)
        .with_keys(&AttributeSet::from_iter([
            KeyCode::BTN_LEFT,
            KeyCode::BTN_RIGHT,
            KeyCode::BTN_MIDDLE,
            KeyCode::BTN_SIDE,
            KeyCode::BTN_EXTRA,
            KeyCode::KEY_LEFT,
            KeyCode::KEY_RIGHT,
            KeyCode::KEY_UP,
            KeyCode::KEY_DOWN,
            KeyCode::KEY_SPACE,
            KeyCode::KEY_LEFTMETA,
            KeyCode::KEY_LEFTCTRL,
            KeyCode::KEY_LEFTSHIFT,
            KeyCode::KEY_LEFTALT,
            KeyCode::KEY_NEXTSONG,
            KeyCode::KEY_PLAYPAUSE,
            KeyCode::KEY_PREVIOUSSONG,
        ]))?
        .with_relative_axes(&AttributeSet::from_iter([
            RelativeAxisCode::REL_X,
            RelativeAxisCode::REL_Y,
            RelativeAxisCode::REL_WHEEL,
            RelativeAxisCode::REL_HWHEEL,
        ]))?
        .build()
}

fn main() -> std::io::Result<()> {
    let mut d = pick_device();
    let mut device = build_device("virtual mouse")?;
    for path in device.enumerate_dev_nodes_blocking()? {
        println!("Available as {}", path?.display());
    }

    let mut joy2mouse = Joy2mouse::default();
    let (tx, rx) = mpsc::channel();

    // Read gamepad events in concurrency
    thread::spawn(move || {
        loop {
            match d.fetch_events() {
                Ok(events) => {
                    for event in events {
                        if tx.send(event).is_err() {
                            return;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch gamepad event: {e}");
                    return;
                }
            }
        }
    });

    let mut should_scrool = 0;
    loop {
        while let Ok(joy_event) = rx.try_recv() {
            if let Some(click) = joy2mouse.translate_joy_event(joy_event) {
                dbg!(&click);
                device.emit(&[click])?;
            }
        }

        let events = joy2mouse.yield_movement_event();
        device.emit(&[events.0, events.1])?;

        // decrease scroll sensitivity by emitting scroll events less
        if should_scrool == 0 {
            let events = joy2mouse.yield_scroll_movement();
            device.emit(&[events.0, events.1])?;
        }
        should_scrool = (should_scrool + 1) % 15;

        sleep(Duration::from_millis(5));
    }
}
