use world::{MoveDir, World};
use shape::Shape;
use ext::*;
use block;

use super::{Entity, EntityWrapper, Police};

const SHOW_PATH_FINDING: bool = false;

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub countdown: u16,
    pub speed: u16,
    pub path: Vec<MoveDir>,
    pub visited: Vec<(u16, u16)>,
    pub pos: (u16, u16),
}

impl Josef {
    pub fn new(pos: (u16, u16), speed: u16) -> Josef {
        Josef {
            countdown: 0,
            speed: speed,
            path: vec! [],
            visited: vec! [],
            pos: pos
        }
    }
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
    fn get_shape(&self) -> Shape { Shape { ch: 'J', col: (255, 0, 0), bg: (0, 0, 0) } }


    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        let mut pos_to_place = None;
        if let Some(EntityWrapper::WJosef(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.countdown == 0 {
                pos_to_place = Some(this.pos);
                this.countdown = this.speed;
            } else {
                this.countdown -= 1;
            }
        }
        if let Some(to_place) = pos_to_place {
            world.add_entity(EntityWrapper::WPolice(Police::new(to_place, 20)));
        }
    }

    fn on_collision(world: &mut World, _me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(EntityWrapper::WPlayer(_)) = world.entities.get(&other_id) {
            let (w, h) = (world.blocks.len(), world.blocks[0].len());
            world.generate(w, h);
        }
        false
    }

    fn pre_draw(&self, _world: &World, _size: &(u16, u16)) {
        if SHOW_PATH_FINDING {
            let mut pos = self.get_pos();

            for pos in &self.visited {
                recolor(*pos, (0, 255, 0), (0, 0, 0));
            }

            for dir in self.path.iter().skip(1) {
                let (dx, dy) = dir.to_vec();
                pos = (pos.0 + dx as u16, pos.1 + dy as u16);

                recolor(pos, (255, 0, 0), (0, 0, 0));
            }
        }
    }
}

