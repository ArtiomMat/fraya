#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers (top row)
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Modifiers
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,

    // Editing
    Enter,
    Escape,
    Backspace,
    Delete,
    Insert,
    Tab,
    Space,

    // Navigation
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,

    // Locks
    CapsLock,
    NumLock,
    ScrollLock,

    // Symbols
    Minus,
    Equal,
    BracketLeft,
    BracketRight,
    Backslash,
    Semicolon,
    Quote,
    Grave,
    Comma,
    Period,
    Slash,

    // Numpad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadEnter,
    NumpadDecimal,

    // System
    PrintScreen,
    Pause,
    Menu,

    // Media
    VolumeUp,
    VolumeDown,
    VolumeMute,
    MediaPlayPause,
    MediaStop,
    MediaNextTrack,
    MediaPreviousTrack,
}

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum WindowEvent {
    Unknown,
    Quit,
    KeyDown(Key),
    KeyUp(Key),
    MouseDelta(f32, f32),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
}

/// A thin wrapper for the sdl3 crate's EventPollIterator.
pub struct WindowEventIterator<'a> {
    pub(super) inner: sdl3::event::EventPollIterator<'a>,
}

impl Iterator for WindowEventIterator<'_> {
    type Item = WindowEvent;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next()? {
            sdl3::event::Event::Quit { .. } => Some(WindowEvent::Quit),

            sdl3::event::Event::KeyDown {
                keycode: Some(kc), ..
            } => from_sdl_keycode(kc).map(WindowEvent::KeyDown),

            sdl3::event::Event::KeyUp {
                keycode: Some(kc), ..
            } => from_sdl_keycode(kc).map(WindowEvent::KeyUp),

            sdl3::event::Event::MouseMotion { xrel, yrel, .. } => {
                Some(WindowEvent::MouseDelta(xrel, yrel))
            }

            sdl3::event::Event::MouseButtonDown { mouse_btn, .. } => {
                from_sdl_mouse_button(mouse_btn).map(WindowEvent::MouseButtonDown)
            }
            
            sdl3::event::Event::MouseButtonUp { mouse_btn, .. } => {
                from_sdl_mouse_button(mouse_btn).map(WindowEvent::MouseButtonUp)
            }

            _ => Some(WindowEvent::Unknown),
        }
    }
}

fn from_sdl_keycode(keycode: sdl3::keyboard::Keycode) -> Option<Key> {
    use sdl3::keyboard::Keycode::*;
    Some(match keycode {
        A => Key::A,
        B => Key::B,
        C => Key::C,
        D => Key::D,
        E => Key::E,
        F => Key::F,
        G => Key::G,
        H => Key::H,
        I => Key::I,
        J => Key::J,
        K => Key::K,
        L => Key::L,
        M => Key::M,
        N => Key::N,
        O => Key::O,
        P => Key::P,
        Q => Key::Q,
        R => Key::R,
        S => Key::S,
        T => Key::T,
        U => Key::U,
        V => Key::V,
        W => Key::W,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,

        Return => Key::Enter,
        Escape => Key::Escape,
        Backspace => Key::Backspace,
        Delete => Key::Delete,
        Insert => Key::Insert,
        Tab => Key::Tab,
        Space => Key::Space,

        Up => Key::ArrowUp,
        Down => Key::ArrowDown,
        Left => Key::ArrowLeft,
        Right => Key::ArrowRight,
        Home => Key::Home,
        End => Key::End,
        PageUp => Key::PageUp,
        PageDown => Key::PageDown,

        LShift => Key::ShiftLeft,
        RShift => Key::ShiftRight,
        LCtrl => Key::ControlLeft,
        RCtrl => Key::ControlRight,
        LAlt => Key::AltLeft,
        RAlt => Key::AltRight,
        LGui => Key::MetaLeft,
        RGui => Key::MetaRight,

        CapsLock => Key::CapsLock,

        // Function keys
        F1 => Key::F1,
        F2 => Key::F2,
        F3 => Key::F3,
        F4 => Key::F4,
        F5 => Key::F5,
        F6 => Key::F6,
        F7 => Key::F7,
        F8 => Key::F8,
        F9 => Key::F9,
        F10 => Key::F10,
        F11 => Key::F11,
        F12 => Key::F12,
        // Symbols
        Minus => Key::Minus,
        Equals => Key::Equal,
        LeftBracket => Key::BracketLeft,
        RightBracket => Key::BracketRight,
        Backslash => Key::Backslash,
        Semicolon => Key::Semicolon,
        Comma => Key::Comma,
        Period => Key::Period,
        Slash => Key::Slash,
        // System
        PrintScreen => Key::PrintScreen,
        Pause => Key::Pause,
        Application => Key::Menu,
        // Media
        VolumeUp => Key::VolumeUp,
        VolumeDown => Key::VolumeDown,
        Mute => Key::VolumeMute,
        // Numbers
        _0 => Key::Num0,
        _1 => Key::Num1,
        _2 => Key::Num2,
        _3 => Key::Num3,
        _4 => Key::Num4,
        _5 => Key::Num5,
        _6 => Key::Num6,
        _7 => Key::Num7,
        _8 => Key::Num8,
        _9 => Key::Num9,
        // Locks
        NumLockClear => Key::NumLock,
        ScrollLock => Key::ScrollLock,
        // Symbols
        Apostrophe => Key::Quote,
        Grave => Key::Grave,
        // Media
        MediaPlayPause => Key::MediaPlayPause,
        MediaStop => Key::MediaStop,
        MediaNextTrack => Key::MediaNextTrack,
        MediaPreviousTrack => Key::MediaPreviousTrack,

        // Numpad
        Kp0 => Key::Numpad0,
        Kp1 => Key::Numpad1,
        Kp2 => Key::Numpad2,
        Kp3 => Key::Numpad3,
        Kp4 => Key::Numpad4,
        Kp5 => Key::Numpad5,
        Kp6 => Key::Numpad6,
        Kp7 => Key::Numpad7,
        Kp8 => Key::Numpad8,
        Kp9 => Key::Numpad9,
        KpPlus => Key::NumpadAdd,
        KpMinus => Key::NumpadSubtract,
        KpMultiply => Key::NumpadMultiply,
        KpDivide => Key::NumpadDivide,
        KpEnter => Key::NumpadEnter,
        KpPeriod => Key::NumpadDecimal,
        
        _ => return None,
    })
}

fn from_sdl_mouse_button(btn: sdl3::mouse::MouseButton) -> Option<MouseButton> {
    use sdl3::mouse::MouseButton::*;
    Some(match btn {
        Left => MouseButton::Left,
        Middle => MouseButton::Middle,
        Right => MouseButton::Right,
        _ => return None,
    })
}
