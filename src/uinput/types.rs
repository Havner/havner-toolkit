use evdev::KeyCode;
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(super) enum Key {
    LeftShift = KeyCode::KEY_LEFTSHIFT.0,
    RightShift = KeyCode::KEY_RIGHTSHIFT.0,
    LeftCtrl = KeyCode::KEY_LEFTCTRL.0,
    RightCtrl = KeyCode::KEY_RIGHTCTRL.0,
    LeftAlt = KeyCode::KEY_LEFTALT.0,
    RightAlt = KeyCode::KEY_RIGHTALT.0,
    LeftMeta = KeyCode::KEY_LEFTMETA.0,
    RightMeta = KeyCode::KEY_RIGHTMETA.0,

    Esc = KeyCode::KEY_ESC.0,
    Tab = KeyCode::KEY_TAB.0,
    Capslock = KeyCode::KEY_CAPSLOCK.0,
    Backspace = KeyCode::KEY_BACKSPACE.0,
    Enter = KeyCode::KEY_ENTER.0,
    Space = KeyCode::KEY_SPACE.0,
    Compose = KeyCode::KEY_COMPOSE.0,
    Menu = KeyCode::KEY_MENU.0,

    Up = KeyCode::KEY_UP.0,
    Down = KeyCode::KEY_DOWN.0,
    Left = KeyCode::KEY_LEFT.0,
    Right = KeyCode::KEY_RIGHT.0,

    Insert = KeyCode::KEY_INSERT.0,
    Delete = KeyCode::KEY_DELETE.0,
    Home = KeyCode::KEY_HOME.0,
    End = KeyCode::KEY_END.0,
    PageUp = KeyCode::KEY_PAGEUP.0,
    PageDown = KeyCode::KEY_PAGEDOWN.0,

    Grave = KeyCode::KEY_GRAVE.0,
    K102nd = KeyCode::KEY_102ND.0,
    Minus = KeyCode::KEY_MINUS.0,
    Equal = KeyCode::KEY_EQUAL.0,
    LeftBrace = KeyCode::KEY_LEFTBRACE.0,
    RightBrace = KeyCode::KEY_RIGHTBRACE.0,
    Backslash = KeyCode::KEY_BACKSLASH.0,
    SemiColon = KeyCode::KEY_SEMICOLON.0,
    Apostrophe = KeyCode::KEY_APOSTROPHE.0,
    Comma = KeyCode::KEY_COMMA.0,
    Dot = KeyCode::KEY_DOT.0,
    Slash = KeyCode::KEY_SLASH.0,

    D1 = KeyCode::KEY_1.0,
    D2 = KeyCode::KEY_2.0,
    D3 = KeyCode::KEY_3.0,
    D4 = KeyCode::KEY_4.0,
    D5 = KeyCode::KEY_5.0,
    D6 = KeyCode::KEY_6.0,
    D7 = KeyCode::KEY_7.0,
    D8 = KeyCode::KEY_8.0,
    D9 = KeyCode::KEY_9.0,
    D0 = KeyCode::KEY_0.0,

    Q = KeyCode::KEY_Q.0,
    W = KeyCode::KEY_W.0,
    E = KeyCode::KEY_E.0,
    R = KeyCode::KEY_R.0,
    T = KeyCode::KEY_T.0,
    Y = KeyCode::KEY_Y.0,
    U = KeyCode::KEY_U.0,
    I = KeyCode::KEY_I.0,
    O = KeyCode::KEY_O.0,
    P = KeyCode::KEY_P.0,

    A = KeyCode::KEY_A.0,
    S = KeyCode::KEY_S.0,
    D = KeyCode::KEY_D.0,
    F = KeyCode::KEY_F.0,
    G = KeyCode::KEY_G.0,
    H = KeyCode::KEY_H.0,
    J = KeyCode::KEY_J.0,
    K = KeyCode::KEY_K.0,
    L = KeyCode::KEY_L.0,

    Z = KeyCode::KEY_Z.0,
    X = KeyCode::KEY_X.0,
    C = KeyCode::KEY_C.0,
    V = KeyCode::KEY_V.0,
    B = KeyCode::KEY_B.0,
    N = KeyCode::KEY_N.0,
    M = KeyCode::KEY_M.0,

    F1 = KeyCode::KEY_F1.0,
    F2 = KeyCode::KEY_F2.0,
    F3 = KeyCode::KEY_F3.0,
    F4 = KeyCode::KEY_F4.0,
    F5 = KeyCode::KEY_F5.0,
    F6 = KeyCode::KEY_F6.0,
    F7 = KeyCode::KEY_F7.0,
    F8 = KeyCode::KEY_F8.0,
    F9 = KeyCode::KEY_F9.0,
    F10 = KeyCode::KEY_F10.0,
    F11 = KeyCode::KEY_F11.0,
    F12 = KeyCode::KEY_F12.0,

    Print = KeyCode::KEY_PRINT.0,
    SysRq = KeyCode::KEY_SYSRQ.0,
    ScrollLock = KeyCode::KEY_SCROLLLOCK.0,
    Pause = KeyCode::KEY_PAUSE.0,

    NumLock = KeyCode::KEY_NUMLOCK.0,
    KpSlash = KeyCode::KEY_KPSLASH.0,
    KpAsterisk = KeyCode::KEY_KPASTERISK.0,
    KpMinus = KeyCode::KEY_KPMINUS.0,
    KpPlus = KeyCode::KEY_KPPLUS.0,
    KpEnter = KeyCode::KEY_KPENTER.0,
    KpDot = KeyCode::KEY_KPDOT.0,
    Kp7 = KeyCode::KEY_KP7.0,
    Kp8 = KeyCode::KEY_KP8.0,
    Kp9 = KeyCode::KEY_KP9.0,
    Kp4 = KeyCode::KEY_KP4.0,
    Kp5 = KeyCode::KEY_KP5.0,
    Kp6 = KeyCode::KEY_KP6.0,
    Kp1 = KeyCode::KEY_KP1.0,
    Kp2 = KeyCode::KEY_KP2.0,
    Kp3 = KeyCode::KEY_KP3.0,
    Kp0 = KeyCode::KEY_KP0.0,

    KpComma = KeyCode::KEY_KPCOMMA.0,
    KpEqual = KeyCode::KEY_KPEQUAL.0,
    KpPlusMinus = KeyCode::KEY_KPPLUSMINUS.0,
    KpLeftParen = KeyCode::KEY_KPLEFTPAREN.0,
    KpRightParen = KeyCode::KEY_KPRIGHTPAREN.0,

    Mute = KeyCode::KEY_MUTE.0,
    VolumeDown = KeyCode::KEY_VOLUMEDOWN.0,
    VolumeUp = KeyCode::KEY_VOLUMEUP.0,
    MicMute = KeyCode::KEY_MICMUTE.0,

    PlayPause = KeyCode::KEY_PLAYPAUSE.0,
    Play = KeyCode::KEY_PLAY.0,
    NextSong = KeyCode::KEY_NEXTSONG.0,
    PreviousSong = KeyCode::KEY_PREVIOUSSONG.0,
    FastForward = KeyCode::KEY_FASTFORWARD.0,
    Rewind = KeyCode::KEY_REWIND.0,

    StopCd = KeyCode::KEY_STOPCD.0,
    PlayCd = KeyCode::KEY_PLAYCD.0,
    PauseCd = KeyCode::KEY_PAUSECD.0,
    CloseCd = KeyCode::KEY_CLOSECD.0,
    EjectCd = KeyCode::KEY_EJECTCD.0,
    EjectCloseCd = KeyCode::KEY_EJECTCLOSECD.0,

    Back = KeyCode::KEY_BACK.0,
    Forward = KeyCode::KEY_FORWARD.0,

    BrightnessDown = KeyCode::KEY_BRIGHTNESSDOWN.0,
    BrightnessUp = KeyCode::KEY_BRIGHTNESSUP.0,
    BrightnessCycle = KeyCode::KEY_BRIGHTNESS_CYCLE.0,
    BrightnessAuto = KeyCode::KEY_BRIGHTNESS_AUTO.0,
    KbdIllumToggle = KeyCode::KEY_KBDILLUMTOGGLE.0,
    KbdIllumDown = KeyCode::KEY_KBDILLUMDOWN.0,
    KbdIllumUp = KeyCode::KEY_KBDILLUMUP.0,

    Zenkakuhankaku = KeyCode::KEY_ZENKAKUHANKAKU.0,
    Ro = KeyCode::KEY_RO.0,
    Katakana = KeyCode::KEY_KATAKANA.0,
    Hiragana = KeyCode::KEY_HIRAGANA.0,
    Henkan = KeyCode::KEY_HENKAN.0,
    Katakanahiragana = KeyCode::KEY_KATAKANAHIRAGANA.0,
    Muhenkan = KeyCode::KEY_MUHENKAN.0,
    KpJpComma = KeyCode::KEY_KPJPCOMMA.0,
    Hangeul = KeyCode::KEY_HANGEUL.0,
    Hanja = KeyCode::KEY_HANJA.0,
    Yen = KeyCode::KEY_YEN.0,

    Linefeed = KeyCode::KEY_LINEFEED.0,
    Macro = KeyCode::KEY_MACRO.0,
    Power = KeyCode::KEY_POWER.0,
    Scale = KeyCode::KEY_SCALE.0,
    Stop = KeyCode::KEY_STOP.0,
    Again = KeyCode::KEY_AGAIN.0,
    Props = KeyCode::KEY_PROPS.0,
    Undo = KeyCode::KEY_UNDO.0,
    Front = KeyCode::KEY_FRONT.0,
    Copy = KeyCode::KEY_COPY.0,
    Open = KeyCode::KEY_OPEN.0,
    Paste = KeyCode::KEY_PASTE.0,
    Find = KeyCode::KEY_FIND.0,
    Cut = KeyCode::KEY_CUT.0,
    Help = KeyCode::KEY_HELP.0,
    Calc = KeyCode::KEY_CALC.0,
    Setup = KeyCode::KEY_SETUP.0,
    Sleep = KeyCode::KEY_SLEEP.0,
    Wakeup = KeyCode::KEY_WAKEUP.0,
    File = KeyCode::KEY_FILE.0,
    SendFile = KeyCode::KEY_SENDFILE.0,
    DeleteFile = KeyCode::KEY_DELETEFILE.0,
    Xfer = KeyCode::KEY_XFER.0,
    Prog1 = KeyCode::KEY_PROG1.0,
    Prog2 = KeyCode::KEY_PROG2.0,
    Www = KeyCode::KEY_WWW.0,
    Msdos = KeyCode::KEY_MSDOS.0,
    Coffee = KeyCode::KEY_COFFEE.0,
    RotateDisplay = KeyCode::KEY_ROTATE_DISPLAY.0,
    CycleWindows = KeyCode::KEY_CYCLEWINDOWS.0,
    Mail = KeyCode::KEY_MAIL.0,
    Bookmarks = KeyCode::KEY_BOOKMARKS.0,
    Computer = KeyCode::KEY_COMPUTER.0,
    Record = KeyCode::KEY_RECORD.0,
    Phone = KeyCode::KEY_PHONE.0,
    Iso = KeyCode::KEY_ISO.0,
    Config = KeyCode::KEY_CONFIG.0,
    Homepage = KeyCode::KEY_HOMEPAGE.0,
    Refresh = KeyCode::KEY_REFRESH.0,
    Exit = KeyCode::KEY_EXIT.0,
    Move = KeyCode::KEY_MOVE.0,
    Edit = KeyCode::KEY_EDIT.0,
    ScrollUp = KeyCode::KEY_SCROLLUP.0,
    ScrollDown = KeyCode::KEY_SCROLLDOWN.0,
    New = KeyCode::KEY_NEW.0,
    Redo = KeyCode::KEY_REDO.0,
    F13 = KeyCode::KEY_F13.0,
    F14 = KeyCode::KEY_F14.0,
    F15 = KeyCode::KEY_F15.0,
    F16 = KeyCode::KEY_F16.0,
    F17 = KeyCode::KEY_F17.0,
    F18 = KeyCode::KEY_F18.0,
    F19 = KeyCode::KEY_F19.0,
    F20 = KeyCode::KEY_F20.0,
    F21 = KeyCode::KEY_F21.0,
    F22 = KeyCode::KEY_F22.0,
    F23 = KeyCode::KEY_F23.0,
    F24 = KeyCode::KEY_F24.0,
    Prog3 = KeyCode::KEY_PROG3.0,
    Prog4 = KeyCode::KEY_PROG4.0,
    Dashboard = KeyCode::KEY_DASHBOARD.0,
    Suspend = KeyCode::KEY_SUSPEND.0,
    Close = KeyCode::KEY_CLOSE.0,
    BassBoost = KeyCode::KEY_BASSBOOST.0,
    Hp = KeyCode::KEY_HP.0,
    Camera = KeyCode::KEY_CAMERA.0,
    Sound = KeyCode::KEY_SOUND.0,
    Question = KeyCode::KEY_QUESTION.0,
    Email = KeyCode::KEY_EMAIL.0,
    Chat = KeyCode::KEY_CHAT.0,
    Search = KeyCode::KEY_SEARCH.0,
    Connect = KeyCode::KEY_CONNECT.0,
    Finance = KeyCode::KEY_FINANCE.0,
    Sport = KeyCode::KEY_SPORT.0,
    Shop = KeyCode::KEY_SHOP.0,
    Alterase = KeyCode::KEY_ALTERASE.0,
    Cancel = KeyCode::KEY_CANCEL.0,
    Media = KeyCode::KEY_MEDIA.0,
    SwitchVideoMode = KeyCode::KEY_SWITCHVIDEOMODE.0,
    Send = KeyCode::KEY_SEND.0,
    Reply = KeyCode::KEY_REPLY.0,
    ForwardMail = KeyCode::KEY_FORWARDMAIL.0,
    Save = KeyCode::KEY_SAVE.0,
    Documents = KeyCode::KEY_DOCUMENTS.0,
    Battery = KeyCode::KEY_BATTERY.0,
    Bluetooth = KeyCode::KEY_BLUETOOTH.0,
    Wlan = KeyCode::KEY_WLAN.0,
    Uwb = KeyCode::KEY_UWB.0,
    Unknown = KeyCode::KEY_UNKNOWN.0,
    VideoNext = KeyCode::KEY_VIDEO_NEXT.0,
    VideoPrev = KeyCode::KEY_VIDEO_PREV.0,
    DisplayOff = KeyCode::KEY_DISPLAY_OFF.0,
    Wwan = KeyCode::KEY_WWAN.0,
    RfKill = KeyCode::KEY_RFKILL.0,
}

impl Key {
    pub(super) fn code(self) -> u16 {
        self as u16
    }
}

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(super) enum Button {
    Left = KeyCode::BTN_LEFT.0,
    Middle = KeyCode::BTN_MIDDLE.0,
    Right = KeyCode::BTN_RIGHT.0,
    Back = KeyCode::BTN_SIDE.0,
    Forward = KeyCode::BTN_EXTRA.0,
}

impl Button {
    pub(super) fn code(self) -> u16 {
        self as u16
    }
}

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
    #[serde(alias = "K")]
    #[serde(alias = "k")]
    Key(Key, #[serde(default)] Direction),
    #[serde(alias = "KC")]
    #[serde(alias = "kc")]
    KeyCode(KeyCode, #[serde(default)] Direction),
    #[serde(alias = "R")]
    #[serde(alias = "r")]
    Raw(u16, #[serde(default)] Direction),
    #[serde(alias = "B")]
    #[serde(alias = "b")]
    Button(Button, #[serde(default)] Direction),
    #[serde(alias = "M")]
    #[serde(alias = "m")]
    MoveMouse(i32, i32, #[serde(default)] Coordinate),
    #[serde(alias = "S")]
    #[serde(alias = "s")]
    Scroll(i32, #[serde(default)] ScrollAxis),
}
