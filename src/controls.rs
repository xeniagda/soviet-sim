use move_dir::MoveDir;
use key::Key;

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct Control<'a> {
    pub modifiers: &'a [Key],
    pub key: Key,
    pub desc: &'a str,
    pub action: Action,
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

pub const CONTROLS: &[Control] = &[
    Control {
        modifiers: &[Key::Shift, Key::Ctrl],
        key: Key::Up,
        desc: "Run upwards",
        action: Action::Run(MoveDir::Up)
    },

    Control {
        modifiers: &[Key::Shift, Key::Ctrl],
        key: Key::Down,
        desc: "Run downwards",
        action: Action::Run(MoveDir::Down)
    },

    Control {
        modifiers: &[Key::Shift, Key::Ctrl],
        key: Key::Left,
        desc: "Run left",
        action: Action::Run(MoveDir::Left)
    },

    Control {
        modifiers: &[Key::Shift, Key::Ctrl],
        key: Key::Right,
        desc: "Run right",
        action: Action::Run(MoveDir::Right)
    },

    Control {
        modifiers: &[Key::Shift],
        key: Key::Right,
        desc: "Increase the active slot",
        action: Action::IncActive
    },

    Control {
        modifiers: &[Key::Shift],
        key: Key::Left,
        desc: "Decrease the active slot",
        action: Action::DecActive
    },

    Control {
        modifiers: &[Key::Letter(12), Key::Alt],
        key: Key::Up,
        desc: "Supermine a block upwards",
        action: Action::SuperMine(MoveDir::Up)
    },

    Control {
        modifiers: &[Key::Letter(12), Key::Alt],
        key: Key::Down,
        desc: "Supermine a block downwards",
        action: Action::SuperMine(MoveDir::Down)
    },

    Control {
        modifiers: &[Key::Letter(12), Key::Alt],
        key: Key::Left,
        desc: "Supermine a block to the left",
        action: Action::SuperMine(MoveDir::Left)
    },

    Control {
        modifiers: &[Key::Letter(12), Key::Alt],
        key: Key::Right,
        desc: "Supermine a block to the right",
        action: Action::SuperMine(MoveDir::Right)
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Up,
        desc: "Break a block upwards",
        action: Action::Break(MoveDir::Up)
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Down,
        desc: "Break a block downwards",
        action: Action::Break(MoveDir::Down)
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Left,
        desc: "Break a block to the left",
        action: Action::Break(MoveDir::Left)
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Right,
        desc: "Break a block to the right",
        action: Action::Break(MoveDir::Right)
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Up,
        desc: "Place a block upwards",
        action: Action::Place(MoveDir::Up)
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Down,
        desc: "Place a block downwards",
        action: Action::Place(MoveDir::Down)
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Left,
        desc: "Place a block to the left",
        action: Action::Place(MoveDir::Left)
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Right,
        desc: "Place a block to the right",
        action: Action::Place(MoveDir::Right)
    },


    Control {
        modifiers: &[],
        key: Key::Up,
        desc: "Move the character upwards",
        action: Action::Move(MoveDir::Up)
    },

    Control {
        modifiers: &[],
        key: Key::Down,
        desc: "Move the character downwards",
        action: Action::Move(MoveDir::Down)
    },

    Control {
        modifiers: &[],
        key: Key::Left,
        desc: "Move the character left",
        action: Action::Move(MoveDir::Left)
    },

    Control {
        modifiers: &[],
        key: Key::Right,
        desc: "Move the character right",
        action: Action::Move(MoveDir::Right)
    },

    Control {
        modifiers: &[],
        key: Key::Letter(8), // I
        desc: "Open/close the inventory",
        action: Action::ToggleInventory
    },

    Control {
        modifiers: &[],
        key: Key::Letter(17), // R
        desc: "Die",
        action: Action::Die
    },

    Control {
        modifiers: &[],
        key: Key::Enter,
        desc: "Select",
        action: Action::Select
    },
];

pub fn parse_control<'a>(key: &'a Key, pressed: &HashSet<Key>) -> Option<&'static Control<'static>> {
    for control in CONTROLS {
        if control.key != *key { continue; }
        let mut all_mods = true;
        for modifier in control.modifiers {
            if !pressed.contains(modifier) {
                all_mods = false;
                break;
            }
        }
        if all_mods {
            return Some(control);
        }
    }
    None
}
