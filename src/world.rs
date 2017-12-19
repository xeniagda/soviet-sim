use ext::*;
use controls::Action;
use block;
use entity;
use entity::Entity;

#[derive(PartialEq, Eq)]
pub struct World<'a> {
    pub blocks: Vec<Vec<&'a block::Block>>,
    pub entities: Vec<entity::EntityWrapper>
}

impl <'a> World<'a> {
    pub fn new(blocks: Vec<Vec<&block::Block>>, entities: Vec<entity::EntityWrapper>) -> World {
        World {
            blocks: blocks,
            entities: entities
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.entities.len() {
            let mut en = self.entities[i].clone();
            // en.tick(&mut self);
            self.entities[i] = en;
        }
    }

    pub fn get_player(&self) -> Option<&entity::Player> {
        for x in &self.entities {
            match x {
                &entity::EntityWrapper::WPlayer(ref pl) => { return Some(&pl); }
                _ => {}
            }
        }
        None
    }

    pub fn get_player_mut(&mut self) -> Option<&mut entity::Player> {
        for x in self.entities.iter_mut() {
            match x {
                &mut entity::EntityWrapper::WPlayer(ref mut pl) => { return Some(pl); }
                _ => {}
            }
        }
        None
    }

    pub fn do_action(&mut self, action: &Action) {
        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(pl) = self.get_player_mut() {

            let move_dir: Option<(i8, i8)> = match *action {
                Action::MoveDown  => { Some((0, 1)) }
                Action::MoveUp    => { Some((0, -1)) }
                Action::MoveLeft  => { Some((-1, 0)) }
                Action::MoveRight => { Some((1, 0)) }
                _                 => { None }
            };

            if let Some((dx, dy)) = move_dir {
                pl.get_pos_mut().0 += dx as u16;
                pl.get_pos_mut().1 += dy as u16;

                new_pos_and_dir = Some((pl.pos.clone(), (dx, dy)));
            }
        }

        if let Some((pos, dir)) = new_pos_and_dir {
            let id = self.blocks[pos.0 as usize][pos.1 as usize].get_id();
            let mut blkf = block::BLOCK_FUNCS.lock().unwrap();
            log(&format!("Walk on id {}", id));

            match blkf.get(id) {
                Some(f) => {
                    if !f(self) {
                        log("Move back!");
                        if let Some(pl) = self.get_player_mut() {
                            pl.get_pos_mut().0 -= dir.0 as u16;
                            pl.get_pos_mut().1 -= dir.1 as u16;
                        }
                    } else {
                        log("Don't Move back!");
                    }
                }
                None => {
                    log("No function registered!");
                }
            }
        }

    }

    pub fn draw(&self) {
        // Draw world
        for (x, col) in self.blocks.iter().enumerate() {
            for (y, block) in col.iter().enumerate() {
                block.get_shape().draw((x as u16, y as u16));
                // put_char((x as u16, y as u16), block.get_ch(), block.get_col(), block.get_bg());
            }
        }

        // Draw player
        self.entities.iter().for_each(|x| x.get_shape().draw(x.get_pos()));
        // put_char(self.player_pos, '@', (0, 255, 0), (0, 0, 0));
    }

    pub fn generate(&mut self, width: usize, height: usize) {
        self.blocks = vec![];
        for x in 0..width {
            self.blocks.push(vec![]);
            for _ in 0..height {
                self.blocks[x].push(&*block::GROUND);
            }
        }
        for _ in 0..400 {
            let x = (rand() * width as f64) as usize;
            let y = (rand() * height as f64) as usize;
            self.blocks[x][y] = &*block::WALL;
        }
        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.blocks[x][y] = &*block::TELEPORTER;
    }
}
