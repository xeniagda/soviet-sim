use move_dir::MoveDir;
use key::Key;

use std::collections::{HashSet, HashMap};

// From https://stackoverflow.com/a/27582993/1753929
macro_rules! hashmap(
    { $($key:expr => $value:expr),+, } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
    { $x:tt } => { hashmap{ $x, } };
);

#[derive(Debug, PartialEq, Eq)]
pub struct Control<'a> {
    pub modifiers: &'a [Key],
    pub keys: HashMap<Key, Action>,
    pub desc: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Move(MoveDir),
    Run(MoveDir),
    Place(MoveDir),
    Break(MoveDir),
    SuperMine(MoveDir),
    IncActive, DecActive,
    ToggleInventory, Die, Select
}

lazy_static! {
    pub static ref CONTROLS: Vec<Control<'static>> = vec![
        Control {
            modifiers: &[Key::Shift, Key::Ctrl],
            keys: hashmap!{
                Key::Arrow(MoveDir::Up)    => Action::Run(MoveDir::Up),
                Key::Arrow(MoveDir::Right) => Action::Run(MoveDir::Right),
                Key::Arrow(MoveDir::Down)  => Action::Run(MoveDir::Down),
                Key::Arrow(MoveDir::Left)  => Action::Run(MoveDir::Left),
            },
            desc: "Run",
        },

        Control {
            modifiers: &[Key::Shift],
            keys: hashmap!{
                Key::Arrow(MoveDir::Right) => Action::IncActive,
                Key::Arrow(MoveDir::Left)  => Action::DecActive,
            },
            desc: "Change the active slot",
        },

        Control {
            modifiers: &[Key::Letter(12), Key::Alt],
            keys: hashmap!{
                Key::Arrow(MoveDir::Up)    => Action::SuperMine(MoveDir::Up),
                Key::Arrow(MoveDir::Right) => Action::SuperMine(MoveDir::Right),
                Key::Arrow(MoveDir::Down)  => Action::SuperMine(MoveDir::Down),
                Key::Arrow(MoveDir::Left)  => Action::SuperMine(MoveDir::Left),
            },
            desc: "Supermine a block",
        },

        Control {
            modifiers: &[Key::Letter(12)],
            keys: hashmap!{
                Key::Arrow(MoveDir::Up)    => Action::Break(MoveDir::Up),
                Key::Arrow(MoveDir::Right) => Action::Break(MoveDir::Right),
                Key::Arrow(MoveDir::Down)  => Action::Break(MoveDir::Down),
                Key::Arrow(MoveDir::Left)  => Action::Break(MoveDir::Left),
            },
            desc: "Break a block",
        },

        Control {
            modifiers: &[Key::Letter(15)],
            keys: hashmap!{
                Key::Arrow(MoveDir::Up)    => Action::Place(MoveDir::Up),
                Key::Arrow(MoveDir::Right) => Action::Place(MoveDir::Right),
                Key::Arrow(MoveDir::Down)  => Action::Place(MoveDir::Down),
                Key::Arrow(MoveDir::Left)  => Action::Place(MoveDir::Left),
            },
            desc: "Place a block",
        },

        Control {
            modifiers: &[],
            keys: hashmap!{
                Key::Arrow(MoveDir::Up)    => Action::Move(MoveDir::Up),
                Key::Arrow(MoveDir::Right) => Action::Move(MoveDir::Right),
                Key::Arrow(MoveDir::Down)  => Action::Move(MoveDir::Down),
                Key::Arrow(MoveDir::Left)  => Action::Move(MoveDir::Left),
            },
            desc: "Move the character",
        },

        Control {
            modifiers: &[],
            keys: hashmap!{ Key::Letter(8) => Action::ToggleInventory, }, // I
            desc: "Open/close the inventory",
        },

        Control {
            modifiers: &[],
            keys: hashmap!{ Key::Letter(17) => Action::Die, }, // R
            desc: "Die",
        },

        Control {
            modifiers: &[],
            keys: hashmap!{ Key::Enter => Action::Select, },
            desc: "Select",
        },
    ];
}

pub fn parse_control<'a>(key: &'a Key, pressed: &HashSet<Key>) -> Option<&'static Action> {
    'outer: for control in CONTROLS.iter() {
        for modifier in control.modifiers {
            if !pressed.contains(modifier) {
                continue 'outer;
            }
        }
        for (k, action) in control.keys.iter() {
            if k == key {
                return Some(action);
            }
        }
    }
    None
}
