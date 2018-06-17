use std::mem::replace;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::ops::{Deref, DerefMut};

use block;
use entity::EntityWrapper;
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
        start_level.generate(GenerationSettings::default_for_difficulty(self.difficulty, true, false));
        self.active_level = start_level;

        loop {
        let mut other_level = self.new_level();
            other_level.generate(
                GenerationSettings {
                    amount_of_walls: 1.,
                    width: 30,
                    height: 30,
                    new_pos_prob: 0.01,
                    new_dir_prob: 0.005,
                    block_probs: hashmap!{
                        block::WALL.clone()   => 0.5,
                        block::STONE.clone()  => 0.495,
                        block::STAIRS.clone() => 0.005,
                    },
                    ..GenerationSettings::default_for_difficulty(self.difficulty, false, true)
                });
            // Verify level: there exists a stair that Josef can reach

            // Find Josef
            let josef_pos = other_level.entities.values()
                    .find(|en| if let EntityWrapper::WJosef(_) = en { true } else { false })
                    .map(|en| en.get_pos())
                    .expect("No Josef!");

            let mut any_stair_good = false;
            'outer: for x in 0..other_level.blocks.len() {
                for y in 0..other_level.blocks[x].len() {
                    if other_level.blocks[x][y] != block::STAIRS.clone() { continue }

                    let heur = |(x_, y_): (u16, u16,)| {
                        let (dx, dy) = (x_.wrapping_sub(x as u16), y_.wrapping_sub(y as u16));
                        let (dx_sq, dy_sq) = (dx.saturating_mul(dx), dy.saturating_mul(dy));
                        let dist = dx_sq.saturating_add(dy_sq) as i32;
                        if dist == 0 {
                            None
                        } else {
                            Some(-dist * 3)
                        }
                    };

                    let path = other_level.find_path(
                        josef_pos,
                        |block, _|
                            if block.is_passable()
                                { Some(1) }
                                else { None },
                        heur,
                        1000);

                    if !path.is_empty() {
                        any_stair_good = true;
                        break 'outer;
                    }
                }
            }

            if !any_stair_good {
                continue;
            }

            self.other_levels.push(other_level);
            break;
        }

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
