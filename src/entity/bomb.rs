use level::Level;
use shape::Shape;
use ext::*;
use block;

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

    fn boom(level: &mut Level, en_id: u64) where Self: Sized {
        let (x, y) =
            if let Some(EntityWrapper::WBomb(ref mut this)) = level.entities.get_mut(&en_id) {
                this.pos
            } else {
                return;
            };

        level.entities.remove(&en_id);
        for (i, entity) in level.entities.clone() {
            let (x_, y_) = entity.get_pos();
            let (dx, dy) = (x - x_, y - y_);
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < BOMB_RADIUS * BOMB_RADIUS {
                entity.get_hurt_fn()(level, i, (BOMB_RADIUS + 1) / (dist_sq / (BOMB_RADIUS + 1) + 1));
            }
        }

        for dx in -(BOMB_RADIUS as i16) .. BOMB_RADIUS as i16 + 1 {
            for dy in -(BOMB_RADIUS as i16) .. BOMB_RADIUS as i16 + 1 {
                let dist_sq = (dx * dx + dy * dy) as u16;
                if dist_sq < BOMB_RADIUS * BOMB_RADIUS {
                    let (x_, y_) = (x.wrapping_add(dx as u16), y.wrapping_add(dy as u16));
                    let breakability = level.get_at((x_, y_))
                            .map(|blk| blk.is_breakable())
                            .unwrap_or(block::Breakability::NotBreakable);

                    match breakability {
                        block::Breakability::NotBreakable => {}
                        block::Breakability::ByBomb | block::Breakability::Breakable => {
                            let mut needed = BOMB_RADIUS as f64 / dist_sq as f64 * 0.3;
                            if breakability == block::Breakability::ByBomb {
                                needed = needed * 2.;
                            }
                            if rand() < needed {
                                if let Some(blk) = level.get_at_mut((x_, y_)) {
                                    *blk = block::GROUND.clone();
                                }
                            }
                        }
                    }
                }
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

    fn tick(level: &mut Level, en_id: u64) where Self: Sized {
        let mut boom = false;
        if let Some(EntityWrapper::WBomb(ref mut this)) = level.entities.get_mut(&en_id) {
            if this.countdown >= this.explode_time {
                boom = true;
            } else {
                this.countdown += 1;
            }
        }
        if boom {
            Bomb::boom(level, en_id);
        }
    }

    fn on_collision(level: &mut Level, me_id: u64, _other_id: u64) -> bool
        where Self: Sized {

        if let Some(EntityWrapper::WBomb(this)) = level.entities.get_mut(&me_id) {
            this.countdown = this.explode_time - BOMB_RADIUS + 1;
        }

        true
    }

    fn pre_draw(&self, _level: &Level, _size: &(u16, u16), scroll: &(i16, i16)) {
        if self.countdown > self.explode_time - BOMB_RADIUS {
            let draw_radius = BOMB_RADIUS + self.countdown - self.explode_time;
            for x in -(draw_radius as i64)..=(draw_radius as i64) {
                for y in -(draw_radius as i64)..=(draw_radius as i64) {
                    if x * x + y * y < (draw_radius * draw_radius) as i64 {
                        let pos = (x as u16 + self.pos.0 - scroll.0 as u16, y as u16 + self.pos.1 - scroll.1 as u16);
                        put_char(pos, &Shape::new('#', (255, 128, 0), (255, 255, 255)));
                    }
                }
            }
        }
    }
}

