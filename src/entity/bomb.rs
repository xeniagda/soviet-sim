use world::World;
use shape::Shape;
use ext::*;

use super::{Entity, EntityWrapper};

const BLINK_TIME: f64 = 100.;

#[derive(PartialEq, Eq, Clone)]
pub struct Bomb {
    pub countdown: u16,
    pub explode_time: u16,
    pub pos: (u16, u16),
}

impl Bomb {
    pub fn new(pos: (u16, u16), explode_time: u16) -> Bomb {
        Bomb {
            countdown: 0,
            explode_time: explode_time,
            pos: pos
        }
    }

    fn boom(world: &mut World, en_id: u64) where Self: Sized {
        log("boom");
        world.entities.remove(&en_id);
    }
}

impl Entity for Bomb {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape {
        let amount_exploded = (self.countdown as f64) / (self.explode_time as f64);

        if (amount_exploded * amount_exploded * BLINK_TIME).sin() > 0. {
            Shape::new('B', (255, 30, 255), (0, 100, 0))
        } else {
            Shape::new('B', (255, 30, 255), (100, 0, 100))
        }
    }
    fn get_name(&self) -> String { "Bomb".into() }

    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        let mut boom = false;
        if let Some(EntityWrapper::WBomb(ref mut this)) = world.entities.get_mut(&en_id) {
            if this.countdown >= this.explode_time {
                boom = true;
            } else {
                this.countdown += 1;
            }
        }
        if boom {
            Bomb::boom(world, en_id);
        }
    }

    fn on_collision(world: &mut World, me_id: u64, _other_id: u64) -> bool
        where Self: Sized {


        Bomb::boom(world, me_id);
        true
    }
}

