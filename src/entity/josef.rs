use world::{World, MetaAction};
use shape::Shape;
use ext::*;
use move_dir::MoveDir;

use super::{Entity, EntityWrapper, Police};

const SHOW_PATH_FINDING: bool = false;
const PLAYER_SAFE_DIST: i32 = 50;

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub police_countdown: u16,
    pub police_speed: u16,
    pub walk_countdown: u16,
    pub walk_speed: u16,
    pub path: Vec<MoveDir>,
    pub pos: (u16, u16),
    pub health: u16
}

impl Josef {
    pub fn new(pos: (u16, u16), police_speed: u16, walk_speed: u16, health: u16) -> Josef {
        Josef {
            police_countdown: 0,
            police_speed: police_speed,
            walk_countdown: 0,
            walk_speed: walk_speed,
            path: vec![],
            health: health,
            pos: pos
        }
    }
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: 'J', col: (255, 0, 0), bg: (0, 0, 0) } }
    fn get_name(&self) -> String { "Josef".into() }


    fn hurt(world: &mut World, en_id: u64, amount: u16) where Self: Sized {
        if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.health < amount {
                world.do_metaaction(MetaAction::Win);
            }
            else {
                this.health -= amount;
                this.police_countdown = 0;
            }
        }
    }

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
                EntityWrapper::WPolice(
                    Police::new(to_place, world.difficulty.get_police_speed(), world.difficulty.get_police_hurt_rate())
                    )
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
        let mut my_pos = None;

        if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.path.len() > 1 {
                if this.walk_countdown == 0 {
                    to_move = Some(this.path.remove(0));
                    this.walk_countdown = this.walk_speed;
                } else {
                    this.walk_countdown -= 1;
                }
            } else {
                my_pos = Some(this.pos);
            }
        }

        if let Some(my_pos) = my_pos {
            let heur = |(x, y): (u16, u16,)| {
                let (dx, dy) = (x.wrapping_sub(player_pos.0), y.wrapping_sub(player_pos.1));
                let (dx_sq, dy_sq) = (dx.saturating_mul(dx), dy.saturating_mul(dy));
                let dist = dx_sq.saturating_add(dy_sq) as i32;
                if dist > PLAYER_SAFE_DIST * PLAYER_SAFE_DIST {
                    Some(-dist)
                } else {
                    Some(dist * 3)
                }
            };

            let path = world.find_path(
                my_pos,
                |block, _|
                    if block.is_passable()
                        { Some(1) }
                        else { None },
                heur,
                1000);

            if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
                this.path = path;
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

    fn on_collision(_world: &mut World, _me_id: u64, _other_id: u64) -> bool
        where Self: Sized {

        false
    }

    fn pre_draw(&self, _world: &World, _size: &(u16, u16), scroll: &(i16, i16)) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            for dir in self.path.iter() {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                recolor((pos.0 - scroll.0 as u16, pos.1 - scroll.1 as u16), (255, 0, 0), (0, 0, 0));
            }
        }
    }
}
