#![allow(dead_code, unused_variables, unused_imports)]

use anyhow::{Context, anyhow};
use evdev::uinput::VirtualDevice;
use evdev::{AttributeSet, EventType, InputEvent, KeyCode, RelativeAxisCode};
use log::{debug, error, trace, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(super) enum Direction {
    #[serde(alias = "P")]
    #[serde(alias = "p")]
    #[serde(alias = "Pressed")]
    #[serde(alias = "pressed")]
    Press,
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    #[serde(alias = "Released")]
    #[serde(alias = "released")]
    Release,
    #[serde(alias = "C")]
    #[serde(alias = "c")]
    #[serde(alias = "Clicked")]
    #[serde(alias = "clicked")]
    #[default]
    Click,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(super) enum Coordinate {
    #[serde(alias = "A")]
    #[serde(alias = "a")]
    #[default]
    Absolute,
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Relative,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub(super) enum ScrollAxis {
    #[serde(alias = "H")]
    #[serde(alias = "h")]
    Horizontal,
    #[serde(alias = "V")]
    #[serde(alias = "v")]
    #[default]
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(super) enum Token {
    #[serde(alias = "KC")]
    #[serde(alias = "kc")]
    KeyCode(KeyCode, #[serde(default)] Direction),
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Raw(u16, #[serde(default)] Direction),
    #[serde(alias = "M")]
    #[serde(alias = "m")]
    MoveMouse(i32, i32, #[serde(default)] Coordinate),
    #[serde(alias = "S")]
    #[serde(alias = "s")]
    Scroll(i32, #[serde(default)] ScrollAxis),
}

// from libc/compat, not sure why evdev doesn't reexport that
const KEY_MAX: u16 = 0x2ff;

pub(super) struct Uinput {
    device: VirtualDevice,
}

impl Uinput {
    pub(super) fn new() -> Result<Self, anyhow::Error> {
        let mut keys = AttributeSet::<KeyCode>::new();
        for code in 0u16..=KEY_MAX {
            keys.insert(KeyCode::new(code));
        }

        let mut rel = AttributeSet::<RelativeAxisCode>::new();
        rel.insert(RelativeAxisCode::REL_X);
        rel.insert(RelativeAxisCode::REL_Y);
        rel.insert(RelativeAxisCode::REL_WHEEL);
        rel.insert(RelativeAxisCode::REL_HWHEEL);

        let device = VirtualDevice::builder()
            .context("uinput builder() failed")?
            .name("havner toolkit uinput device")
            .with_keys(&keys)
            .context("uinput builder.with_keys() failed")?
            .with_relative_axes(&rel)
            .context("uinput builder.with_rel_axes() failed")?
            .build()
            .context("uinput builder.build() failed")?;

        Ok(Self { device })
    }

    pub(super) fn execute(&mut self, token: Token) -> anyhow::Result<()> {
        match token {
            Token::KeyCode(kc, d) => self.key_code(kc, d),
            Token::Raw(c, d) => self.raw(c, d),
            Token::MoveMouse(x, y, c) => self.move_mouse(x, y, c),
            Token::Scroll(l, a) => self.scroll(l, a),
        }
    }

    // fn key(&mut self, key: Key, direction: Direction) -> anyhow::Result<()> {
    //     Ok(())
    // }

    fn key_code(&mut self, key_code: KeyCode, direction: Direction) -> anyhow::Result<()> {
        self.raw(key_code.0, direction)
    }

    fn raw(&mut self, keycode: u16, direction: Direction) -> anyhow::Result<()> {
        let events: &[InputEvent] = match direction {
            Direction::Click => &[
                InputEvent::new(EventType::KEY.0, keycode, 1),
                InputEvent::new(EventType::KEY.0, keycode, 0),
            ],
            Direction::Press => &[InputEvent::new(EventType::KEY.0, keycode, 1)],
            Direction::Release => &[InputEvent::new(EventType::KEY.0, keycode, 0)],
        };

        self.device.emit(events).context("emit raw events failed")?;
        Ok(())
    }

    // fn button(&mut self, button: Button, direction: Direction) -> anyhow::Result<()> {
    //     let keycode = match button {
    //         Button::Left => KeyCode::BTN_LEFT,
    //         Button::Right => KeyCode::BTN_RIGHT,
    //         Button::Middle => KeyCode::BTN_MIDDLE,
    //         Button::Back => KeyCode::BTN_BACK,
    //         Button::Forward => KeyCode::BTN_FORWARD,
    //         _ => return inverr!("scroll buttons are not supported by uinput"),
    //     };
    //     let events: &[InputEvent] = match direction {
    //         Direction::Click => &[InputEvent::new(EventType::KEY.0, keycode.0, 1),
    //                              InputEvent::new(EventType::KEY.0, keycode.0, 0)],
    //         Direction::Press => &[InputEvent::new(EventType::KEY.0, keycode.0, 1)],
    //         Direction::Release => &[InputEvent::new(EventType::KEY.0, keycode.0, 0)],
    //     };

    //     self.device.emit(events).or(simerr!("sending uinput events failed"))?;
    //     Ok(())
    // }

    fn move_mouse(&mut self, x: i32, y: i32, coordinate: Coordinate) -> anyhow::Result<()> {
        // uinput doesn't know about screen coordinates, simulate absolute
        if coordinate == Coordinate::Absolute {
            let top_left_corner: &[InputEvent] = &[
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, i32::MIN),
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, i32::MIN),
            ];
            self.device.emit(top_left_corner).context("emit mouse events failed")?;
        }

        let events: &[InputEvent] = &[
            InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, x),
            InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, y),
        ];
        self.device.emit(events).context("emit mouse events failed")?;

        Ok(())
    }

    fn scroll(&mut self, length: i32, axis: ScrollAxis) -> anyhow::Result<()> {
        let axis = match axis {
            ScrollAxis::Vertical => RelativeAxisCode::REL_WHEEL.0,
            ScrollAxis::Horizontal => RelativeAxisCode::REL_HWHEEL.0,
        };

        let events: &[InputEvent] = &[
            InputEvent::new(EventType::RELATIVE.0, axis, length),
            InputEvent::new(EventType::RELATIVE.0, axis, length),
        ];
        self.device.emit(events).context("emit scroll events failed")?;

        Ok(())
    }
}
