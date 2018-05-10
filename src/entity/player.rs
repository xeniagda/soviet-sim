use world::{World, HOTBAR_HEIGHT, MetaAction};
use shape::Shape;
use ext::*;
use crafting::Recipe;
use inventory::InventoryItem;
use move_dir::MoveDir;
use block;
use super::EntityWrapper;

use super::Entity;

const COMMUNISM_WIDTH: u16 = 10;

#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16),
    pub inventory: Vec<(InventoryItem, u64)>,
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

    pub fn place(world: &mut World, dir: MoveDir, en_id: u64) where Self: Sized {
        let mut entity_positions = world.entities.clone().into_iter().map(|(_, x)| x.get_pos());

        let mut to_place: Option<(InventoryItem, (u16, u16))> = None;

        if let Some(EntityWrapper::WPlayer(ref mut this)) = world.entities.get_mut(&en_id) {
            let place_pos = (this.pos.0 + dir.to_vec().0 as u16, this.pos.1 + dir.to_vec().1 as u16);

            if world.blocks
                .get(place_pos.0 as usize)
                    .and_then(|x| x.get(place_pos.1 as usize)) != Some(&block::GROUND)
            {
                return;
            }

            if entity_positions
                .any(|x| x == place_pos)
            {
                log("entity on tile");
                return;
            }

            if let Some((ref item, ref mut amount)) = this.inventory.get_mut(this.active) {
                *amount -= 1;

                to_place = Some((item.clone(), place_pos));

                if amount == &0 {
                    this.inventory.remove(this.active);
                    if this.active >= this.inventory.len() {
                        this.active = this.inventory.len() - 1;
                    }
                }
            }
        }

        if let Some((to_place, (x, y))) = to_place {
            if !to_place.place_pos(world, (x, y), dir) {
                // Give back
                if let Some(EntityWrapper::WPlayer(ref mut this)) = world.entities.get_mut(&en_id) {
                    this.pick_up(to_place);
                }
            }
        }
    }

    pub fn pick_up(&mut self, item: InventoryItem) {
        if let Some(&mut (_, ref mut count)) = self.inventory.iter_mut()
                .find(|x| x.0 == item) {
            *count += 1;
        } else {
            self.inventory.push((item, 1));
        }
    }

    pub fn craft(&mut self, rec: &Recipe) -> bool {

        // Is craftable?
        for (c_item, c_amount) in rec.needed.iter() {
            // Has item?
            if self.inventory
                .iter()
                .find(|(item, _)| item == c_item)
                .map(|(_, amount)| amount < c_amount)
                .unwrap_or(true)
            {
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
                if self.active >= self.inventory.len() {
                    self.active = self.inventory.len() - 1;
                }
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
    fn get_name(&self) -> String { "Player".into() }

    fn hurt(world: &mut World, en_id: u64, amount: u16) where Self: Sized {
        let mut action_restart = None;
        if let Some(EntityWrapper::WPlayer(ref mut this)) = world.entities.get_mut(&en_id) {
            this.hunger -= amount;

            if this.hunger == 0 {
                action_restart = Some(true);
            } else {
                action_restart = Some(false);
            }
        }

        if let Some(restart) = action_restart {
            if restart {
                world.do_metaaction(MetaAction::Die);
            }
        }
    }

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

