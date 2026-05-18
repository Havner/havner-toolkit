use crate::uinput::types::{Direction, Token};
use evdev::uinput::VirtualDevice;
use evdev::{
    AbsInfo, AbsoluteAxisCode, AttributeSet, EventType, InputEvent, KeyCode, RelativeAxisCode,
    SynchronizationCode, UinputAbsSetup,
};

pub(crate) struct Devices {
    keyboard: VirtualDevice,
    gamepad: VirtualDevice,
}

const KEY_MAX: u16 = 0x2ff;

impl Devices {
    pub fn new() -> Result<Self, anyhow::Error> {
        let mut keys = AttributeSet::<KeyCode>::new();
        for code in 0u16..=KEY_MAX {
            keys.insert(KeyCode::new(code));
        }

        let mut rel = AttributeSet::<RelativeAxisCode>::new();
        rel.insert(RelativeAxisCode::REL_X);
        rel.insert(RelativeAxisCode::REL_Y);
        rel.insert(RelativeAxisCode::REL_WHEEL);
        rel.insert(RelativeAxisCode::REL_HWHEEL);

        let keyboard = VirtualDevice::builder()?
            .name("OpenDeck uinput simulation device")
            .with_keys(&keys)?
            .with_relative_axes(&rel)?
            .build()?;

        let mut g_keys = AttributeSet::<KeyCode>::new();

        g_keys.insert(KeyCode::BTN_SOUTH);   // A
        g_keys.insert(KeyCode::BTN_EAST);    // B
        g_keys.insert(KeyCode::BTN_NORTH);   // X
        g_keys.insert(KeyCode::BTN_WEST);    // Y

        g_keys.insert(KeyCode::BTN_TL);      // Left Bumper (LB)
        g_keys.insert(KeyCode::BTN_TR);      // Right Bumper (RB)

        g_keys.insert(KeyCode::BTN_TL2);      // Left Bumper (LB)
        g_keys.insert(KeyCode::BTN_TR2);      // Right Bumper (RB)

        // Menu / Center Buttons
        g_keys.insert(KeyCode::BTN_SELECT);  // Back / View / Share
        g_keys.insert(KeyCode::BTN_START);   // Start / Menu
        g_keys.insert(KeyCode::BTN_MODE);    // Xbox Guide Button (optional)

        // Stick Clicks (L3 / R3)
        g_keys.insert(KeyCode::BTN_THUMBL);  // Left Stick Click
        g_keys.insert(KeyCode::BTN_THUMBR);  // Right Stick Click

        // Note on D-PAD:
        // On most modern controllers, the D-Pad is handled via HAT axes (ABS_HAT0X, ABS_HAT0Y)
        // rather than buttons. If you want them as buttons, use:
        g_keys.insert(KeyCode::BTN_DPAD_UP);
        g_keys.insert(KeyCode::BTN_DPAD_DOWN);
        g_keys.insert(KeyCode::BTN_DPAD_LEFT);
        g_keys.insert(KeyCode::BTN_DPAD_RIGHT);

        let gamepad = VirtualDevice::builder()?
            .name("OpenDeck Gamepad")
            // MANDATORY: BTN_GAMEPAD makes Wine recognize it as a controller
            .with_keys(&g_keys)?
            .with_absolute_axis(&UinputAbsSetup::new(
                AbsoluteAxisCode::ABS_X,
                AbsInfo::new(0, -32768, 32767, 0, 4096, 0),
            ))?
            .with_absolute_axis(&UinputAbsSetup::new(
                AbsoluteAxisCode::ABS_Y,
                AbsInfo::new(0, -32768, 32767, 0, 4096, 0),
            ))?
            .build()?;

        Ok(Self { keyboard, gamepad })
    }

    pub fn execute(&mut self, token: Token) -> anyhow::Result<()> {
        match token {
            Token::Key(k, d) => self.key(k, d),
            Token::KeyCode(kc, d) => self.key_code(kc, d),
            Token::Raw(c, d) => self.raw(c, d),
            Token::Button(b, d) => self.button(b, d),
            Token::MoveMouse(x, y, c) => self.move_mouse(x, y, c),
            Token::Scroll(l, a) => self.scroll(l, a),
        }
    }

    fn key(&mut self, key: crate::uinput::types::Key, direction: Direction) -> anyhow::Result<()> {
        self.raw(key.code(), direction)
    }

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

        let device = if keycode >= 0x130 { &mut self.gamepad } else { &mut self.keyboard };

        device.emit(events)?;

        device.emit(&[InputEvent::new(
            EventType::SYNCHRONIZATION.0,
            SynchronizationCode::SYN_REPORT.0,
            0,
        )])?;

        Ok(())
    }

    fn button(
        &mut self,
        button: crate::uinput::types::Button,
        direction: Direction,
    ) -> anyhow::Result<()> {
        let keycode = button.code();
        match direction {
            // for some strange reason click doesn't work for mouse in one emit
            Direction::Click => {
                self.raw(keycode, Direction::Press)?;
                self.raw(keycode, Direction::Release)
            }
            _ => self.raw(keycode, direction),
        }
    }

    fn move_mouse(
        &mut self,
        x: i32,
        y: i32,
        coordinate: crate::uinput::types::Coordinate,
    ) -> anyhow::Result<()> {
        // uinput doesn't know about screen coordinates, simulate absolute
        if coordinate == crate::uinput::types::Coordinate::Absolute {
            let top_left_corner: &[InputEvent] = &[
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, i32::MIN),
                InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, i32::MIN),
            ];
            self.keyboard.emit(top_left_corner)?;
        }

        let events: &[InputEvent] = &[
            InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_X.0, x),
            InputEvent::new(EventType::RELATIVE.0, RelativeAxisCode::REL_Y.0, y),
        ];
        self.keyboard.emit(events)?;

        Ok(())
    }

    fn scroll(
        &mut self,
        length: i32,
        axis: crate::uinput::types::ScrollAxis,
    ) -> anyhow::Result<()> {
        let axis = match axis {
            crate::uinput::types::ScrollAxis::Vertical => RelativeAxisCode::REL_WHEEL.0,
            crate::uinput::types::ScrollAxis::Horizontal => RelativeAxisCode::REL_HWHEEL.0,
        };

        let events: &[InputEvent] = &[
            InputEvent::new(EventType::RELATIVE.0, axis, length),
            InputEvent::new(EventType::RELATIVE.0, axis, length),
        ];
        self.keyboard.emit(events)?;

        Ok(())
    }
}
