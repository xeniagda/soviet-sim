use std::fmt::{Display, Formatter, Error};
use move_dir::MoveDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Key {
    Arrow(MoveDir),
    Letter(u8),
    Digit(u8),
    Shift, Alt, Ctrl, Meta,
    Enter, Backspace, Tab,
    Plus, Minus
}

impl Display for Key {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        let text = match self {
            Key::Arrow(dir) => dir.to_ch().to_string(),
            Key::Letter(ch) => format!("{}", (ch + 'A' as u8) as char),
            Key::Digit(ch)  => format!("{}", (ch + '0' as u8) as char),
            Key::Shift      => "Shift".into(),
            Key::Alt        => "Alt".into(),
            Key::Ctrl       => "Ctrl".into(),
            Key::Meta       => "Meta".into(),
            Key::Enter      => "Enter".into(),
            Key::Backspace  => "Backspace".into(),
            Key::Tab        => "Tab".into(),
            Key::Plus       => "Plus".into(),
            Key::Minus      => "Minus".into(),
        };
        write!(fmt, "{}", text)
    }
}

pub fn parse_key(code: u8) -> Option<Key> {
    match code {
        37 => Some(Key::Arrow(MoveDir::Left)),
        38 => Some(Key::Arrow(MoveDir::Up)),
        39 => Some(Key::Arrow(MoveDir::Right)),
        40 => Some(Key::Arrow(MoveDir::Down)),
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
