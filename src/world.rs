use std::mem::replace;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::{Deref, DerefMut};

use block;
use difficulty::Difficulty;
use level::{Level, MetaAction, GenerationSettings};

pub struct Callback(pub Box<Fn(&mut World)>);

unsafe impl Send for Callback {}

pub struct World {
    pub active_level: Level,
    pub difficulty: Difficulty,
    pub other_levels: Vec<Level>,
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

    pub fn generate(&mut self) {
        let mut start_level = self.new_level();
        start_level.generate(GenerationSettings::default_for_difficulty(self.difficulty, true));
        self.active_level = start_level;

        let mut other_level = self.new_level();
        other_level.generate(
            GenerationSettings {
                amount_of_walls: 1.,
                width: 15,
                height: 15,
                block_probs: hashmap!{
                    block::WALL.clone()   => 0.5,
                    block::STONE.clone()  => 0.495,
                    block::STAIRS.clone() => 0.005,
                },
                ..GenerationSettings::default_for_difficulty(self.difficulty, false)
            });

        self.other_levels.push(other_level);
    }

    fn new_level(&self) -> Level {
        Level::empty(self.difficulty, self.action_sender.clone(), self.callback_send.clone())
    }

    pub fn set_active(&mut self, idx: usize) {
        let mut next_active = self.other_levels.remove(idx);
        if let Some(player_id) = self.active_level.get_player_id() {
            let player = self.active_level.entities.remove(&player_id).unwrap();
            next_active.add_entity(player);
        }

        let old_active = replace(&mut self.active_level, next_active);

        self.other_levels.push(old_active);
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
