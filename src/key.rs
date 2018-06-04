use std::fmt::{Display, Formatter, Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Key {
    Down, Up, Left, Right,
    Letter(u8),
    Digit(u8),
    Shift, Alt, Ctrl, Meta,
    Enter, Backspace, Tab,
    Plus, Minus
}

impl Display for Key {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Key::*;
        let text = match self {
            Down       => "Down".into(),
            Up         => "Up".into(),
            Left       => "Left".into(),
            Right      => "Right".into(),
            Letter(ch) => format!("'{}'", (ch + 'A' as u8) as char),
            Digit(ch)  => format!("{}", (ch + '0' as u8) as char),
            Shift      => "Shift".into(),
            Alt        => "Alt".into(),
            Ctrl       => "Ctrl".into(),
            Meta       => "Meta".into(),
            Enter      => "Enter".into(),
            Backspace  => "Backspace".into(),
            Tab        => "Tab".into(),
            Plus       => "Plus".into(),
            Minus      => "Minus".into(),
        };
        write!(fmt, "{}", text)
    }
}

pub fn parse_key(code: u8) -> Option<Key> {
    match code {
        37 => Some(Key::Left),
        38 => Some(Key::Up),
        39 => Some(Key::Right),
        40 => Some(Key::Down),
        17 => Some(Key::Ctrl),
        16 => Some(Key::Shift),
        18 => Some(Key::Alt),
        224 => Some(Key::Meta),
        13 => Some(Key::Enter),
        8 => Some(Key::Backspace),
        9 => Some(Key::Tab),
        171 => Some(Key::Plus),
        173 => Some(Key::Minus),
        65...90 => Some(Key::Letter(code - b'A')),
        48...57 => Some(Key::Digit(code - 48)),
        _ => None
    }
}
