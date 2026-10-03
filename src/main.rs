// Create a virtual mouse, just while this is running.
// Generally this requires root.

use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

use evdev::{AttributeSet, KeyCode, RelativeAxisCode, uinput::VirtualDevice};

mod config;
mod constants;
mod translate_event;

use config::Config;
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

fn build_device(name: &str, config: &Config) -> std::io::Result<VirtualDevice> {
    VirtualDevice::builder()?
        .name(name)
        .with_keys(&AttributeSet::from_iter(
            config.keys().iter().map(|key| KeyCode(*key)),
        ))?
        .with_relative_axes(&AttributeSet::from_iter([
            RelativeAxisCode::REL_X,
            RelativeAxisCode::REL_Y,
            RelativeAxisCode::REL_WHEEL,
            RelativeAxisCode::REL_HWHEEL,
        ]))?
        .build()
}

fn main() -> std::io::Result<()> {
    let config = Config::new("assets/config.toml");
    let mut d = pick_device();
    let mut device = build_device("virtual mouse", &config)?;
    let mut joy2mouse = Joy2mouse::new(config);
    for path in device.enumerate_dev_nodes_blocking()? {
        println!("Available as {}", path?.display());
    }

    // Read gamepad events in concurrency
    let (tx, rx) = mpsc::channel();
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
