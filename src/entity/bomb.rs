use world::World;
use shape::Shape;
use ext::*;

use super::{Entity, EntityWrapper};

const BLINK_TIME: f64 = 50.;
const BOMB_RADIUS: u16 = 5;

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
        let (x, y) =
            if let Some(EntityWrapper::WBomb(ref mut this)) = world.entities.get_mut(&en_id) {
                this.pos
            } else {
                return;
            };

        world.entities.remove(&en_id);
        for (i, entity) in world.entities.clone() {
            let (x_, y_) = entity.get_pos();
            let (dx, dy) = (x - x_, y - y_);
            if dx * dx + dy * dy < BOMB_RADIUS * BOMB_RADIUS {
                entity.get_hurt_fn()(world, i, 5);
            }
        }
    }
}

impl Entity for Bomb {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape {
        let amount_exploded = (self.countdown as f64) / (self.explode_time as f64);

        let explode_amount = amount_exploded * amount_exploded * BLINK_TIME;

        // Equivalent of explode_amount % 1, but fmod is not supperted in wasm
        let background =
            if explode_amount - explode_amount as u64 as f64 > 0.5 {
                (0, 100, 0)
            } else {
                (100, 0, 100)
            };

        let foreground =
            if self.countdown < self.explode_time / 4 * 3 {
                (255, 30, 255)
            } else {
                (255, 255, 255)
            };

        Shape::new('B', foreground, background)
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

    fn pre_draw(&self, _world: &World, _size: &(u16, u16)) {
        if self.countdown > self.explode_time - BOMB_RADIUS {
            let draw_radius = BOMB_RADIUS + self.countdown - self.explode_time;
            for x in -(draw_radius as i64)..=(draw_radius as i64) {
                for y in -(draw_radius as i64)..=(draw_radius as i64) {
                    if x * x + y * y < (draw_radius * draw_radius) as i64 {
                        put_char((x as u16 + self.pos.0, y as u16 + self.pos.1),
                            &Shape::new(' ', (0, 0, 0), (255, 255, 255)));
                    }
                }
            }
        }
    }
}

