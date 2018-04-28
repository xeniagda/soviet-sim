use world::World;
use shape::Shape;
use ext::*;
use block;

use super::Entity;


#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16),
    pub inventory: Vec<(block::Block, u64)>,
    pub active: usize,
    pub hunger: u16
}

impl Player {
    pub fn new(pos: (u16, u16)) -> Player {
        Player {
            pos: pos,
            active: 0,
            inventory: vec! [],
            hunger: 1
        }
    }

    pub fn pick_up(&mut self, block: block::Block) {
        if block.get_shape() ==  Shape::new('☭', (180, 0, 0), (0, 0, 0)) {
            self.hunger += 1;
        } else if let Some(&mut (_, ref mut count)) = self.inventory.iter_mut()
                .find(|x| x.0 == block) {
            *count += 1;
        } else {
            self.inventory.push((block, 1));
        }
        log(&format!("Inventory: {:?}", self.inventory));
    }
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: '@', col: (0, 255, 0), bg: (0, 0, 0) } }

    fn pre_draw(&self, _world: &World, size: &(u16, u16)) {
        for i in 0..5 {
            if i < self.hunger {
                put_char((i, size.1 - 1), &Shape::new('☭', (180, 0, 0), (0, 0, 0)));
            } else {
                put_char((i, size.1 - 1), &Shape::new('☭', (255, 255, 255), (0, 0, 0)));
            }
        }

        let mut x = 5;

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

