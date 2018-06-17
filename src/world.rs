use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::{Deref, DerefMut};

use difficulty::Difficulty;
use level::{Level, MetaAction};

pub struct Callback(Box<Fn(&mut World)>);

unsafe impl Send for Callback {}

pub struct World {
    active_level: Level,
    difficulty: Difficulty,
    other_levels: Vec<Level>,
    callback_send: Sender<Callback>,
    callback_recv: Receiver<Callback>,
    action_sender: Sender<MetaAction>
}

impl World {
    pub fn empty(difficulty: Difficulty, action_sender: Sender<MetaAction>) -> World {
        let (callback_send, callback_recv) = channel();

        World {
            active_level: Level::empty(difficulty, action_sender.clone(), callback_send.clone()),
            difficulty: difficulty,
            other_levels: vec![],
            callback_send: callback_send,
            callback_recv: callback_recv,
            action_sender: action_sender,
        }
    }

    pub fn tick(&mut self) {
        for level in &mut self.other_levels.iter_mut() {
            level.tick();
        }
        self.active_level.tick();

        while let Ok(f) = self.callback_recv.try_recv() {
            f.0(self);
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
