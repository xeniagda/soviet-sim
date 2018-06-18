use level::{Level, MetaAction};
use shape::Shape;
use ext::*;
use move_dir::MoveDir;

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


    fn hurt(level: &mut Level, en_id: u64, amount: u16) where Self: Sized {
        if let Some(EntityWrapper::WJosef(ref mut this)) = level.entities.get_mut(&en_id) {
            if this.health < amount {
                level.do_metaaction(MetaAction::Win);
            }
            else {
                this.health -= amount;
                this.police_countdown = 0;
            }
        }
    }

    fn tick(level: &mut Level, en_id: u64) where Self: Sized {

        // Place police
        let mut pos_to_place = None;
        if let Some(EntityWrapper::WJosef(ref mut this)) = level.entities.get_mut(&en_id) {
            if this.police_countdown == 0 {
                pos_to_place = Some(this.pos);
                this.police_countdown = this.police_speed;
            } else {
                this.police_countdown -= 1;
            }
        }
        if let Some(to_place) = pos_to_place {
            level.add_entity(
                EntityWrapper::WPolice(
                    Police::new(to_place, level.difficulty.get_police_speed(), level.difficulty.get_police_hurt_rate())
                    )
                );
        }

        let player_pos =
            if let Some(EntityWrapper::WPlayer(ref player)) =
                level.get_player_id().and_then(|x| level.entities.get(&x))
            {
                player.pos
            } else {
                return;
            };

        let mut to_move = None;
        let mut should_move = false;

        if let Some(EntityWrapper::WJosef(ref mut this)) = level.entities.get_mut(&en_id) {
            if this.walk_countdown == 0 {
                if this.path.len() > 0 {
                    to_move = Some(this.path.remove(0));
                    this.walk_countdown = this.walk_speed;
                }
                should_move = true;
            } else {
                this.walk_countdown -= 1;
            }
        }

        if let Some(to_move) = to_move {
            Josef::move_dir(level, en_id, to_move);
        }

        let mut my_pos = None;
        if let Some(EntityWrapper::WJosef(ref mut this)) = level.entities.get_mut(&en_id) {
            my_pos = Some(this.pos);
        }

        if should_move {
            if let Some(my_pos) = my_pos {
                let heur = |(x, y): (u16, u16,)| {
                    let (dx, dy) = (x as f64 - player_pos.0 as f64, y as f64 - player_pos.1 as f64);

                    let dist_sq = dx * dx + dy * dy;

                    Some(500. - dist_sq.sqrt())
                };

                let path = level.find_path(
                    my_pos,
                    |block, _|
                        if block.is_passable()
                            { Some(1.) }
                            else { None },
                    heur,
                    10000,
                    true);

                if let Some(EntityWrapper::WJosef(ref mut this)) = level.entities.get_mut(&en_id) {
                    log(&format!("Path: {:?}", path));
                    this.path = path;
                }
            }
        }
    }

    fn on_collision(_level: &mut Level, _me_id: u64, _other_id: u64) -> bool
        where Self: Sized {

        false
    }

    fn pre_draw(&self, _level: &Level, _size: &(u16, u16), scroll: &(i16, i16)) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            for dir in self.path.iter() {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0.wrapping_add(dx as u16), pos.1.wrapping_add(dy as u16));

                if let (Some(x), Some(y)) = (pos.0.checked_sub(scroll.0 as u16), pos.1.checked_sub(scroll.1 as u16)) {
                    recolor((x, y), (255, 0, 0), (0, 0, 0));
                }
            }
        }
    }
}
