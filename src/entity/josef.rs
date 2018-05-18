use std::u16;

use world::{World, MetaAction};
use shape::Shape;
use move_dir::{MoveDir, DIRECTIONS};
use ext::*;

use super::{Entity, EntityWrapper, Police};

const SHOW_PATH_FINDING: bool = false;

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub police_countdown: u16,
    pub police_speed: u16,
    pub walk_countdown: u16,
    pub walk_speed: u16,
    pub path: Vec<MoveDir>,
    pub pos: (u16, u16),
}

impl Josef {
    pub fn new(pos: (u16, u16), police_speed: u16, walk_speed: u16) -> Josef {
        Josef {
            police_countdown: 0,
            police_speed: police_speed,
            walk_countdown: 0,
            walk_speed: walk_speed,
            path: vec![],
            pos: pos
        }
    }
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: 'J', col: (255, 0, 0), bg: (0, 0, 0) } }
    fn get_name(&self) -> String { "Josef".into() }


    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        // Place police
        let mut pos_to_place = None;
        if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.police_countdown == 0 {
                pos_to_place = Some(this.pos);
                this.police_countdown = this.police_speed;
            } else {
                this.police_countdown -= 1;
            }
        }
        if let Some(to_place) = pos_to_place {
            world.add_entity(
                EntityWrapper::WPolice(Police::new(to_place, world.difficulty.get_police_speed()))
                );
        }

        let player_pos =
            if let Some(EntityWrapper::WPlayer(ref player)) =
                world.get_player_id().and_then(|x| world.entities.get(&x))
            {
                player.pos
            } else {
                return;
            };

        let mut to_move = None;

        if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.path.len() > 1 {
                if this.walk_countdown == 0 {
                    to_move = Some(this.path.remove(0));
                    this.walk_countdown = this.walk_speed;
                } else {
                    this.walk_countdown -= 1;
                }
            } else {
                // Walk away from player
                let mut paths: Vec<(u16, Vec<MoveDir>, (u16, u16))> = vec![(0, vec![], this.pos)];
                let mut best_path: Option<(u16, (Vec<MoveDir>, (u16, u16)))> = None;

                for _ in 0..1000 {
                    if paths.len() == 0 {
                        break;
                    }

                    if let Some((_, from, pos)) = paths.pop() {
                        for direction in &DIRECTIONS {
                            let new_pos = direction.move_vec(pos);

                            if paths.iter().any(|x| x.2 == new_pos) {
                                continue;
                            }

                            if let Some(block) = world.blocks
                                .get(new_pos.0 as usize)
                                    .and_then(|x| x.get(new_pos.1 as usize))
                            {
                                if block.is_passable() {
                                    let mut new_from = from.clone();
                                    new_from.push(*direction);

                                    // Check score

                                    let (dx, dy) = (new_pos.0 - player_pos.0, new_pos.1 - player_pos.1);

                                    let dist = dx * dx + dy * dy;
                                    let mut score = dist * 3 - new_from.len() as u16;
                                    if dist > 100 {
                                        score -= 50;
                                    }

                                    for i in 0..paths.len() + 1 {
                                        if paths.get(i).map(|x| x.0).unwrap_or(u16::MAX) > score {
                                            paths.insert(i, (score, new_from.clone(), new_pos));
                                            break;
                                        }
                                    }


                                    let best_score =
                                        if let Some((best_score, _)) = best_path {
                                            best_score
                                        } else {
                                            0
                                        };

                                    if score > best_score {
                                        this.path = new_from.clone();
                                        best_path = Some((score, (new_from, new_pos)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(to_move) = to_move {
            if !Josef::move_dir(world, en_id, to_move) || rand() < 0.25 {
                if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
                    this.path = vec![];
                }
            }
        }
    }

    fn on_collision(world: &mut World, _me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(EntityWrapper::WPlayer(_)) = world.entities.get(&other_id) {
            world.do_metaaction(MetaAction::Win);
        }
        false
    }

    fn pre_draw(&self, _world: &World, _size: &(u16, u16), scroll: &(i16, i16)) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            log(&format!("Path: {:?}", self.path));

            for dir in self.path.iter() {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                recolor((pos.0 - scroll.0 as u16, pos.1 - scroll.1 as u16), (255, 0, 0), (0, 0, 0));
            }
        }
    }
}
