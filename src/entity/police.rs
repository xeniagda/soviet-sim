use world::World;
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

    fn hurt(world: &mut World, en_id: u64, _amount: u16) {
        let mut drops: Vec<(u16, u16)> = Vec::new();

        if let Some(en) = world.entities.get(&en_id) {
            drops.push(en.get_pos());
        }

        world.entities.remove(&en_id);

        let mut i = 0;
        loop {
            if drops.is_empty() { break; }

            let idx = (rand() * drops.len() as f64) as usize;
            let (x, y) = drops[idx];
            if inventory::InventoryItem::Block(block::COMMUNISM.clone()).place_pos(world, (x, y), MoveDir::Up) {
                for dir in &DIRECTIONS {
                    drops.push(dir.move_vec((x, y)));
                }
                i += 1;
            }

            if i >= world.difficulty.get_communism_drop_rate() as usize ||
                rand() < 1. / world.difficulty.get_communism_drop_rate() {
                break;
            }
        }

    }

    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        let should_walk = {
            if let Some(&mut EntityWrapper::WPolice(ref mut this)) = world.entities.get_mut(&en_id) {
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
            let (player_pos, my_pos);
            if let Some(player) = world.get_player_id().and_then(|x| world.entities.get(&x)) {
                if let Some(this) = world.entities.get(&en_id) {
                    player_pos = player.get_pos();
                    my_pos = this.get_pos();
                } else {
                    return;
                }
            } else {
                return;
            }

            let mut visited = vec![ my_pos ];
            let mut paths: Vec<(_, Vec<MoveDir>)> = vec![ (my_pos, vec![]) ];

            let mut best_path = None;
            let mut best_score = u16::max_value();

            // Find closest path to player
            'outer: for _ in 0..100 {
                if let Some((ref pos, ref path)) = paths.clone().into_iter()
                    .min_by_key(|&(ref pos, ref path)| {
                        let (dx, dy) = (pos.0.wrapping_sub(player_pos.0), pos.1.wrapping_sub(player_pos.1));
                        let (dx_sq, dy_sq) = (dx.saturating_mul(dx), dy.saturating_mul(dy));
                        dx_sq.saturating_add(dy_sq).saturating_add(path.len() as u16 * 2)
                    }) {
                        paths.remove_item(&(*pos, path.clone()));

                        let mut dirs = vec! [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];
                        for _ in 0..4 {
                            let fidx = rand() * dirs.len() as f64;
                            let dir = dirs[fidx as usize];
                            dirs.remove(fidx as usize);

                            let (dx, dy) = dir.to_vec();
                            let new_pos = (pos.0.wrapping_add(dx as u16), pos.1.wrapping_add(dy as u16));

                            let mut new_path = path.clone();
                            new_path.push(dir);

                            if visited.contains(&new_pos) {
                                continue;
                            }
                            if new_pos == player_pos {
                                best_path = Some(new_path);
                                break 'outer;
                            }

                            let passable_block = world.blocks.get(new_pos.0 as usize)
                                .and_then(|x| x.get(new_pos.1 as usize))
                                .map(|x| x.is_passable())
                                .unwrap_or(false);
                            let passable_entity =
                                !world.entities.values()
                                .any(|x| x.get_pos() == new_pos);

                            if passable_block && passable_entity {
                                paths.push((new_pos, new_path.clone()));
                                visited.push(new_pos);

                                let (dx, dy) = (pos.0.wrapping_sub(player_pos.0), pos.1.wrapping_sub(player_pos.1));
                                let (dx_sq, dy_sq) = (dx.saturating_mul(dx), dy.saturating_mul(dy));
                                let score = dx_sq.saturating_add(dy_sq).saturating_add(path.len() as u16 * 2);
                                if score < best_score {
                                    best_path = Some(new_path);
                                    best_score = score;
                                }
                            }
                        }
                    } else {
                        break 'outer;
                    }
            }

            if let Some(best_path) = best_path.clone() {
                Police::move_dir(world, en_id, best_path[0]);
            }
            if let Some(&mut EntityWrapper::WPolice(ref mut this)) = world.entities.get_mut(&en_id) {
                this.path = best_path.unwrap_or(vec![]);
                this.visited = visited;
            }
        }


    }

    fn on_collision(world: &mut World, me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(EntityWrapper::WPolice(ref mut me)) = world.entities.get_mut(&me_id) {
            if me.hurt_countdown == 0 {
                me.hurt_countdown = me.hurt_speed;
                if let Some(en) = world.entities.get(&other_id) {
                    match en {
                        EntityWrapper::WPlayer(_) => {
                            en.get_hurt_fn()(world, other_id, 1);
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

    fn pre_draw(&self, _world: &World, _size: &(u16, u16), scroll: &(i16, i16)) {
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
