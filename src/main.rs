use std::{
    io::{Error, ErrorKind, Result},
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

use evdev::{AttributeSet, Device, KeyCode, RelativeAxisCode, uinput::VirtualDevice};

mod config;
mod constants;
mod joy2mouse;

use config::Config;
use joy2mouse::Joy2mouse;

fn pick_device(config: &Config) -> Result<Device> {
    if let Some(dev_file) = std::env::args_os().nth(2) {
        evdev::Device::open(dev_file)
    } else {
        evdev::enumerate()
            .filter(|t| {
                t.1.name()
                    .unwrap_or_default()
                    .to_lowercase()
                    .contains(&config.misc.device_name)
            })
            .map(|t| t.1)
            .last()
            .ok_or(Error::new(ErrorKind::NotFound, "no gamepad found"))
    }
}

fn build_device(config: &Config) -> Result<VirtualDevice> {
    VirtualDevice::builder()?
        .name(
            &config
                .misc
                .virtual_divece_name
                .clone()
                .unwrap_or("virtual mouse".to_string()),
        )
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

fn main() -> Result<()> {
    let config = Config::new("assets/config.toml");
    let mut device = build_device(&config)?;
    let mut joy2mouse = Joy2mouse::new(config.clone());

    loop {
        // keep trying connect to device every 5 seconds if unavailable
        if let Ok(mut d) = pick_device(&config) {
            eprintln!("Gamepad is available now");

            // Read gamepad events in concurrency
            let (tx, rx) = mpsc::channel();
            let (tx_die, rx_die) = mpsc::channel();
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
                            let _ = tx_die.send(true);
                            return;
                        }
                    }
                }
            });

            let mut should_scrool = 0;
            loop {
                if rx_die.try_recv().is_ok() {
                    break;
                }

                while let Ok(joy_event) = rx.try_recv() {
                    if let Some(click) = joy2mouse.translate_joy_event(joy_event) {
                        if !joy2mouse.on() {
                            break;
                        }
                        dbg!(&click);
                        device.emit(&[click])?;
                    }
                }

                if !joy2mouse.on() {
                    continue;
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
        eprintln!("Waiting for reconnection...");
        sleep(Duration::from_millis(5000));
    }
}
