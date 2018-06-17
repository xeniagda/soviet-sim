use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::{Deref, DerefMut};

use difficulty::Difficulty;
use level::{Level, MetaAction};

struct Callback(Box<Fn(&mut World)>);

unsafe impl Send for Callback {}

pub struct World {
    active_level: Level,
    difficulty: Difficulty,
    other_levels: Vec<Level>,
    fn_send: Sender<Callback>,
    fn_recv: Receiver<Callback>,
    action_sender: Sender<MetaAction>
}

impl World {
    pub fn empty(difficulty: Difficulty, action_sender: Sender<MetaAction>) -> World {
        let (fn_send, fn_recv) = channel();

        World {
            active_level: Level::empty(difficulty, action_sender.clone()),
            difficulty: difficulty,
            other_levels: vec![],
            fn_send: fn_send,
            fn_recv: fn_recv,
            action_sender: action_sender,
        }
    }
}

impl Deref for World {
    type Target = Level;

    fn deref(&self) -> &Level {
        &self.active_level
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Level {
        &mut self.active_level
    }
}
