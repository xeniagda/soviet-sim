
use key::Key;

#[derive(Debug, PartialEq, Eq)]
pub struct Control<'a> {
    pub modifiers: &'a [Key],
    pub key: Key,
    pub desc: &'a str,
    pub action: Action,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    MoveUp, MoveDown, MoveLeft, MoveRight,
    RunUp,  RunDown,  RunLeft,  RunRight,
    SideUp,  SideDown,  SideLeft,  SideRight,
    Restart
}

const CONTROLS: &[Control] = &[
    Control {modifiers: &[Key::Letter(18)],     key: Key::Up,     desc: "Run upwards",                    action: Action::SideUp},
    Control {modifiers: &[Key::Letter(18)],     key: Key::Down,   desc: "Run downwards",                  action: Action::SideDown},
    Control {modifiers: &[Key::Letter(18)],     key: Key::Left,   desc: "Run left",                       action: Action::SideLeft},
    Control {modifiers: &[Key::Letter(18)],     key: Key::Right,  desc: "Run right",                      action: Action::SideRight},
    Control {modifiers: &[Key::Shift],          key: Key::Up,      desc: "Run upwards",                    action: Action::RunUp},
    Control {modifiers: &[Key::Shift],          key: Key::Down,    desc: "Run downwards",                  action: Action::RunDown},
    Control {modifiers: &[Key::Shift],          key: Key::Left,    desc: "Run left",                       action: Action::RunLeft},
    Control {modifiers: &[Key::Shift],          key: Key::Right,   desc: "Run right",                      action: Action::RunRight},
    Control {modifiers: &[],                    key: Key::Up,      desc: "Move the character upwards",     action: Action::MoveUp},
    Control {modifiers: &[],                    key: Key::Down,    desc: "Move the character downwards",   action: Action::MoveDown},
    Control {modifiers: &[],                    key: Key::Left,    desc: "Move the character left",        action: Action::MoveLeft},
    Control {modifiers: &[],                    key: Key::Right,   desc: "Move the character right",       action: Action::MoveRight},
    Control {modifiers: &[],                    key: Key::Letter(b'r' - b'a'),   desc: "Restart level",    action: Action::Restart},
];

pub fn parse_control<'a>(key: &'a Key, pressed: &[Key]) -> Option<&'static Control<'static>> {
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
