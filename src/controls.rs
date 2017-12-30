
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
    MoveUp,   MoveDown,   MoveLeft,   MoveRight,
    RunUp,    RunDown,    RunLeft,    RunRight,
    SideUp,   SideDown,   SideLeft,   SideRight,
    PlaceUp,  PlaceDown,  PlaceLeft,  PlaceRight,
    BreakUp,  BreakDown,  BreakLeft,  BreakRight,
    IncActive, DecActive,
    Restart,
}

const CONTROLS: &[Control] = &[
    Control {
        modifiers: &[Key::Letter(18)],
        key: Key::Up,
        desc: "Run upwards",
        action: Action::SideUp
    },

    Control {
        modifiers: &[Key::Letter(18)],
        key: Key::Down,
        desc: "Run downwards",
        action: Action::SideDown
    },

    Control {
        modifiers: &[Key::Letter(18)],
        key: Key::Left,
        desc: "Run left",
        action: Action::SideLeft
    },

    Control {
        modifiers: &[Key::Letter(18)],
        key: Key::Right,
        desc: "Run right",
        action: Action::SideRight
    },


    Control {
        modifiers: &[Key::Shift],
        key: Key::Up,
        desc: "Run upwards",
        action: Action::RunUp
    },

    Control {
        modifiers: &[Key::Shift],
        key: Key::Down,
        desc: "Run downwards",
        action: Action::RunDown
    },

    Control {
        modifiers: &[Key::Shift],
        key: Key::Left,
        desc: "Run left",
        action: Action::RunLeft
    },

    Control {
        modifiers: &[Key::Shift],
        key: Key::Right,
        desc: "Run right",
        action: Action::RunRight
    },


    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Up,
        desc: "Break a block upwards",
        action: Action::BreakUp
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Down,
        desc: "Break a block downwards",
        action: Action::BreakDown
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Left,
        desc: "Break a block to the left",
        action: Action::BreakLeft
    },

    Control {
        modifiers: &[Key::Letter(12)],
        key: Key::Right,
        desc: "Break a block to the right",
        action: Action::BreakRight
    },


    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Up,
        desc: "Place a block upwards",
        action: Action::PlaceUp
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Down,
        desc: "Place a block downwards",
        action: Action::PlaceDown
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Left,
        desc: "Place a block to the left",
        action: Action::PlaceLeft
    },

    Control {
        modifiers: &[Key::Letter(15)],
        key: Key::Right,
        desc: "Place a block to the right",
        action: Action::PlaceRight
    },


    Control {
        modifiers: &[],
        key: Key::Up,
        desc: "Move the character upwards",
        action: Action::MoveUp
    },

    Control {
        modifiers: &[],
        key: Key::Down,
        desc: "Move the character downwards",
        action: Action::MoveDown
    },

    Control {
        modifiers: &[],
        key: Key::Left,
        desc: "Move the character left",
        action: Action::MoveLeft
    },

    Control {
        modifiers: &[],
        key: Key::Right,
        desc: "Move the character right",
        action: Action::MoveRight
    },

    Control {
        modifiers: &[],
        key: Key::Plus,
        desc: "Increase the active slot",
        action: Action::IncActive
    },

    Control {
        modifiers: &[],
        key: Key::Minus,
        desc: "Decrease the active slot",
        action: Action::DecActive
    },

    Control {
        modifiers: &[],
        key: Key::Letter(17),
        desc: "Restart level",
        action: Action::Restart
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
