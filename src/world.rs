use ext::*;
use controls::Action;
use block;
use entity;
use entity::{EntityWrapper, Player, Josef};

use std::collections::HashMap;


#[derive(PartialEq, Eq)]
pub struct World<'a> {
    pub blocks: Vec<Vec<&'a block::Block>>,
    pub entities: HashMap<u64, entity::EntityWrapper>,
    auto: Option<MoveDir>,
    last: Option<MoveDir>
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum MoveDir { Up, Left, Down, Right }

impl MoveDir {
    pub fn to_vec(&self) -> (i8, i8) {
        match self {
            &MoveDir::Up => (0, -1),
            &MoveDir::Down => (0, 1),
            &MoveDir::Left => (-1, 0),
            &MoveDir::Right => (1, 0),
        }
    }
}

impl <'a> World<'a> {
    pub fn empty() -> World<'a> {
        World {
            blocks: vec![],
            entities: HashMap::new(),
            auto: None,
            last: None
        }
    }

    pub fn tick(&mut self) {
        for k in self.entities.clone().keys() {
            if let Some(f) = self.entities.get(k).map(|x| x.get_tick_fn()) {
                f(self, *k);
            }
        }

        // Automove
        if let Some(auto) = self.auto {
            if self.move_player_side(&auto) {
                self.auto = None;
                self.last = None;
            }
        }
        self.draw();
    }

    pub fn get_player_id(&self) -> Option<u64> {
        for (k, x) in &self.entities {
            if let &entity::EntityWrapper::WPlayer(_) = x {
                return Some(*k);
            }
        }
        None
    }

    pub fn do_action(&mut self, action: &Action) {
        if let &Action::Restart = action {
            let (w, h) = (self.blocks.len(), self.blocks[0].len());
            self.generate(w, h);
            return;
        }

        self.auto = match *action {
            Action::RunDown  => { Some(MoveDir::Down) }
            Action::RunUp    => { Some(MoveDir::Up) }
            Action::RunLeft  => { Some(MoveDir::Left) }
            Action::RunRight => { Some(MoveDir::Right) }
            _                => { None }
        };


        let move_dir: Option<MoveDir> = match *action {
            Action::MoveDown  => { Some(MoveDir::Down) }
            Action::MoveUp    => { Some(MoveDir::Up) }
            Action::MoveLeft  => { Some(MoveDir::Left) }
            Action::MoveRight => { Some(MoveDir::Right) }
            _                 => { None }
        };

        if let Some(x) = move_dir {
            self.auto = None;
            self.get_player_id().map(|id| self.move_entity(id, &x));
            self.last = Some(x);
        }

        let move_dir_side: Option<MoveDir> = match *action {
            Action::SideDown  => { Some(MoveDir::Down) }
            Action::SideUp    => { Some(MoveDir::Up) }
            Action::SideLeft  => { Some(MoveDir::Left) }
            Action::SideRight => { Some(MoveDir::Right) }
            _                 => { None }
        };

        if let Some(x) = move_dir_side {
            self.auto = None;
            self.move_player_side(&x);
        }
    }

    fn move_player_side(&mut self, move_dir: &MoveDir) -> bool {
        if self.get_player_id().map(|id| self.move_entity(id, move_dir)) == Some(true) {
            match *move_dir {
                MoveDir::Up | MoveDir::Down => {
                    let mut d1 = MoveDir::Left;
                    let mut d2 = MoveDir::Right;
                    if let Some(last) = self.last {
                        if last == d2 {
                            d1 = MoveDir::Right;
                            d2 = MoveDir::Left;
                        }
                    } else if rand() < 0.5 {
                        d1 = MoveDir::Right;
                        d2 = MoveDir::Left;
                    }

                    self.last = Some(d1);
                    if self.get_player_id()
                        .map(|id| self.move_entity(id, &d1))
                            == Some(true) {
                        if self.get_player_id()
                            .map(|id| self.move_entity(id, &d2))
                                == Some(true) {
                            return true;
                        }
                    }
                }
                MoveDir::Left | MoveDir::Right => {
                    let mut d1 = MoveDir::Up;
                    let mut d2 = MoveDir::Down;
                    if let Some(last) = self.last {
                        if last == d2 {
                            d1 = MoveDir::Down;
                            d2 = MoveDir::Right;
                        }
                    } else if rand() < 0.5 {
                        d1 = MoveDir::Down;
                        d2 = MoveDir::Up;
                    }

                    self.last = Some(d1);
                    if self.get_player_id()
                        .map(|id| self.move_entity(id, &d1))
                            == Some(true) {
                        if self.get_player_id()
                            .map(|id| self.move_entity(id, &d2))
                                == Some(true) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }


    fn move_entity(&mut self, en_id: u64, move_dir: &MoveDir) -> bool {

        self.last = Some(*move_dir);
        if let Some(en_move_fn) = self.entities.get(&en_id).map(|x| x.get_move_fn()) {
            en_move_fn(self, en_id, *move_dir)
        } else {
            false
        }

    }

    pub fn draw(&self) {
        // Draw world
        for (x, col) in self.blocks.iter().enumerate() {
            for (y, block) in col.iter().enumerate() {
                block.get_shape().draw((x as u16, y as u16));
            }
        }

        // Draw entities
        self.entities.iter()
            .for_each(|(_, x)| {
                x.get_shape().draw(x.get_pos());
                x.draw_other(self, x.get_pos());
            });
    }

    pub fn generate(&mut self, width: usize, height: usize) {
        log("Generating!");

        self.blocks = vec![];
        self.auto = None;

        for x in 0..width {
            self.blocks.push(vec![]);
            for _ in 0..height {
                self.blocks[x].push(&*block::WALL);
            }
        }

        let mut placed = vec![];
        for _ in 0..10000 {
            if rand() < 0.001 || placed.is_empty() {
                let x = (rand() * width as f64) as usize;
                let y = (rand() * height as f64) as usize;
                self.blocks[x][y] = &*block::GROUND;
                placed.push((x, y, random_dir()));
            } else {
                let idx = (rand() * placed.len() as f64) as usize;
                let (x, y, mut dir) = placed[idx];

                if rand() < 0.05 {
                    dir = random_dir();
                }

                let dirv = dir.to_vec();

                let (nx, ny) = (x + dirv.0 as usize, y + dirv.1 as usize);

                let can_place = self.blocks.get(nx).and_then(|col| col.get(ny)) == Some(&&*block::WALL);
                if can_place {
                    self.blocks[nx][ny] = &*block::GROUND;
                    placed.push((nx, ny, dir));
                    self.entities = HashMap::new();
                    self.add_entity(
                        EntityWrapper::WPlayer(
                            Player {
                                pos: (nx as u16, ny as u16),
                            }
                            )
                        );
                }
            }
        }

        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.blocks[x][y] = &*block::TELEPORTER;

        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.blocks[x][y] = &*block::MOVER;

        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.add_entity(EntityWrapper::WJosef(Josef { pos: (x as u16, y as u16), countdown: 0, speed: 10 }));

        log("Done!");
    }

    fn add_entity(&mut self, entity: EntityWrapper) {
        loop {
            let key = (rand() * <u64>::max_value() as f64) as u64;
            if !self.entities.contains_key(&key) {
                self.entities.insert(key, entity);
                break;
            }
        }
    }
}

pub fn random_dir() -> MoveDir {
    match (rand() * 4.) as usize {
        0 => MoveDir::Left,
        1 => MoveDir::Right,
        2 => MoveDir::Up,
        3 => MoveDir::Down,
        _ => MoveDir::Left
    }
}
