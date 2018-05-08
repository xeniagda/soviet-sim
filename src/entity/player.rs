use world::{World, HOTBAR_HEIGHT};
use shape::Shape;
use ext::*;
use crafting::Recipe;
use block;

use super::Entity;

const COMMUNISM_WIDTH: u16 = 10;

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16),
    pub inventory: Vec<(block::Block, u64)>,
    pub active: usize,
    pub hunger: u16
}


impl Player {
    pub fn new(pos: (u16, u16), hunger: u16) -> Player {
        Player {
            pos: pos,
            active: 0,
            inventory: vec! [],
            hunger: hunger,
        }
    }

    pub fn pick_up(&mut self, block: block::Block) {
        if let Some(&mut (_, ref mut count)) = self.inventory.iter_mut()
                .find(|x| x.0 == block) {
            *count += 1;
        } else {
            self.inventory.push((block, 1));
        }
        log(&format!("Inventory: {:?}", self.inventory));
    }

    pub fn craft(&mut self, rec: &Recipe) -> bool {
        log(&format!("Crafting {:?}", rec));

        // Is craftable?
        for (c_item, c_amount) in rec.needed.iter() {
            // Has item?
            if self.inventory
                .iter()
                .find(|(item, _)| item == c_item)
                .map(|(_, amount)| amount < c_amount)
                .unwrap_or(true)
            {
                log(&format!("Not enough {:?}", c_item));
                return false;
            }
        }

        for (c_item, c_amount) in rec.needed.iter() {
            let (i, (_, ref mut amount)) = self.inventory.iter_mut()
                    .enumerate()
                    .find(|(_, (item, _))| item == c_item)
                    .expect("o no");

            *amount -= c_amount;

            if amount == &0 {
                self.inventory.remove(i);
            }
        }

        self.pick_up(rec.out.clone());

        true
    }
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: '@', col: (0, 255, 0), bg: (0, 0, 0) } }

    fn pre_draw(&self, _world: &World, size: &(u16, u16)) {
        if self.hunger > HOTBAR_HEIGHT * COMMUNISM_WIDTH {
            put_char((0, size.1 - HOTBAR_HEIGHT as u16), &Shape::new('☭', (180, 0, 0), (0, 0, 0)));
            let x = format!("x{}", self.hunger);
            for (i, ch) in x.chars().enumerate() {
                put_char(
                    (1 + i as u16, size.1 - HOTBAR_HEIGHT as u16),
                    &Shape::new(ch, (180, 180, 180), (0, 0, 0))
                    );
            }
        } else {
            for y in 0..self.hunger / COMMUNISM_WIDTH + 1 {
                for i in 0..COMMUNISM_WIDTH {
                    if i + y * COMMUNISM_WIDTH < self.hunger {
                        put_char(
                            (i, size.1 + y - HOTBAR_HEIGHT as u16),
                            &Shape::new('☭', (180, 0, 0), (0, 0, 0))
                            );
                    } else {
                        put_char(
                            (i, size.1 + y - HOTBAR_HEIGHT as u16),
                            &Shape::new('☭', (255, 255, 255), (0, 0, 0))
                            );
                    }
                }
            }
        }

        let mut x = COMMUNISM_WIDTH + 1;

        for (i, &(ref block, ref count)) in self.inventory.iter().enumerate() {
            block.get_shape().draw((x, size.1 - 2));
            let text = format!("x{}", count);

            if i == self.active {
                put_char((x, size.1 - 1), &Shape::new('^', (255, 255, 255), (0, 0, 0)));
            }

            for ch in text.chars() {
                x += 1;
                put_char((x, size.1 - 2), &Shape::new(ch, (255, 255, 255), (0, 0, 0)));
            }

            x += 3;
        }
    }
}

