use level::Level;
use shape::Shape;
use ext::*;
use block;
use move_dir::{MoveDir, DIRECTIONS};
use inventory;

use super::{Entity, EntityWrapper};

const SHOW_PATH_FINDING: bool = false;

#[derive(PartialEq, Eq, Clone)]
pub struct Police {
    pub walk_countdown: u16,
    pub walk_speed: u16,
    pub hurt_countdown: u16,
    pub hurt_speed: u16,
    pub path: Vec<MoveDir>,
    pub visited: Vec<(u16, u16)>,
    pub pos: (u16, u16),
}

impl Police {
    pub fn new(pos: (u16, u16), walk_speed: u16, hurt_speed: u16) -> Police {
        Police {
            walk_countdown: 0,
            walk_speed: walk_speed,
            hurt_countdown: 0,
            hurt_speed: hurt_speed,
            path: vec! [],
            visited: vec! [],
            pos: pos
        }
    }
}

impl Entity for Police {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: 'T', col: (255, 0, 0), bg: (0, 0, 0) } }
    fn get_name(&self) -> String { "Police".into() }

    fn hurt(level: &mut Level, en_id: u64, _amount: u16) {
        let mut drops: Vec<(u16, u16)> = Vec::new();

        if let Some(en) = level.entities.get(&en_id) {
            drops.push(en.get_pos());
        }

        level.entities.remove(&en_id);

        let mut i = 0;
        loop {
            if drops.is_empty() { break; }

            let idx = (rand() * drops.len() as f64) as usize;
            let (x, y) = drops[idx];
            if inventory::InventoryItem::Block(block::COMMUNISM.clone()).place_pos(level, (x, y), MoveDir::Up) {
                for dir in &DIRECTIONS {
                    drops.push(dir.move_vec((x, y)));
                }
                i += 1;
            }

            if i >= level.difficulty.get_communism_drop_rate() as usize ||
                rand() < 1. / level.difficulty.get_communism_drop_rate() {
                break;
            }
        }

    }

    fn tick(level: &mut Level, en_id: u64) where Self: Sized {
        let should_walk = {
            if let Some(&mut EntityWrapper::WPolice(ref mut this)) = level.entities.get_mut(&en_id) {
                if this.walk_countdown == 0 {
                    this.walk_countdown = this.walk_speed;
                    true
                } else {
                    this.walk_countdown -= 1;
                    false
                }
            } else {
                false
            }
        };

        if should_walk {
            let mut to_move = None;
            if let Some(EntityWrapper::WPolice(this)) = level.entities.get_mut(&en_id) {
                if !this.path.is_empty() {
                    to_move = Some(this.path.remove(0));
                }
            } else {
                return;
            }

            if let Some(to_move) = to_move {
                Police::move_dir(level, en_id, to_move);
            }

            if to_move == None || rand() < 0.2 {
                let (player_pos, my_pos);
                if let Some(EntityWrapper::WPolice(this)) = level.entities.get_mut(&en_id) {
                    my_pos = this.get_pos();
                } else {
                    return;
                }
                if let Some(player) = level.get_player_id().and_then(|x| level.entities.get(&x)) {
                    player_pos = player.get_pos();
                } else {
                    return;
                }

                let heur = |(x, y): (u16, u16,)| {
                    let (dx, dy) = (x as f64 - player_pos.0 as f64, y as f64 - player_pos.1 as f64);

                    let dist_sq = dx * dx + dy * dy;

                    Some(dist_sq.sqrt())
                };

                let path = level.find_path(
                    my_pos,
                    |block, _|
                        if block.is_passable()
                            { Some(1.) }
                            else { None },
                    heur,
                    1000,
                    true);

                if let Some(EntityWrapper::WPolice(ref mut this)) = level.entities.get_mut(&en_id) {
                    log(&format!("Path: {:?}", path));
                    this.path = path;
                }
            }
        }
    }

    fn on_collision(level: &mut Level, me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(EntityWrapper::WPolice(ref mut me)) = level.entities.get_mut(&me_id) {
            if me.hurt_countdown == 0 {
                me.hurt_countdown = me.hurt_speed;
                if let Some(en) = level.entities.get(&other_id) {
                    match en {
                        EntityWrapper::WPlayer(_) => {
                            en.get_hurt_fn()(level, other_id, 1);
                        }
                        _ => {}
                    }
                }
            } else {
                me.hurt_countdown -= 1;
            }
        }

        false
    }

    fn pre_draw(&self, _level: &Level, _size: &(u16, u16), scroll: &(i16, i16)) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            for pos in &self.visited {
                recolor((pos.0 - scroll.0 as u16, pos.1 - scroll.1 as u16), (0, 255, 0), (0, 0, 0));
            }

            for dir in self.path.iter().skip(1) {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                recolor((pos.0 - scroll.0 as u16, pos.1 - scroll.1 as u16), (255, 0, 0), (0, 0, 0));
            }
        }
    }
}
