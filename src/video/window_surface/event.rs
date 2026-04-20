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

        Num0 => Key::Num0,
        Num1 => Key::Num1,
        Num2 => Key::Num2,
        Num3 => Key::Num3,
        Num4 => Key::Num4,
        Num5 => Key::Num5,
        Num6 => Key::Num6,
        Num7 => Key::Num7,
        Num8 => Key::Num8,
        Num9 => Key::Num9,

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
