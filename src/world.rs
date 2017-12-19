use ext::*;
use controls::Action;
use block;
use entity;
use entity::Entity;

#[derive(PartialEq, Eq)]
pub struct World<'a> {
    pub blocks: Vec<Vec<&'a block::Block>>,
    pub entities: Vec<entity::EntityWrapper>,
    auto: Option<MoveDir>
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MoveDir { Up, Left, Down, Right }

impl MoveDir {
    fn to_vec(&self) -> (i8, i8) {
        match self {
            &MoveDir::Up => (0, -1),
            &MoveDir::Down => (0, 1),
            &MoveDir::Left => (-1, 0),
            &MoveDir::Right => (1, 0),
        }
    }
}

impl <'a> World<'a> {
    pub fn new(blocks: Vec<Vec<&block::Block>>, entities: Vec<entity::EntityWrapper>) -> World {
        World {
            blocks: blocks,
            entities: entities,
            auto: None
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.entities.len() {
            let mut en = self.entities[i].clone();
            // en.tick(&mut self);
            self.entities[i] = en;
        }

        // Automove
        if let Some(auto) = self.auto {
            if self.move_player_side(&auto) {
                self.auto = None;
            }
            self.draw();
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
            self.move_player(&x);
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
        if self.move_player(move_dir) {
            match *move_dir {
                MoveDir::Up | MoveDir::Down => {
                    let mut d1 = MoveDir::Left;
                    let mut d2 = MoveDir::Right;
                    if rand() < 0.5 {
                        d1 = MoveDir::Right;
                        d2 = MoveDir::Left;
                    }
                    if self.move_player(&d1) {
                        if self.move_player(&d2) {
                            return true;
                        }
                    }
                }
                MoveDir::Left | MoveDir::Right => {
                    let mut d1 = MoveDir::Up;
                    let mut d2 = MoveDir::Down;
                    if rand() < 0.5 {
                        d1 = MoveDir::Down;
                        d2 = MoveDir::Up;
                    }
                    if self.move_player(&d1) {
                        if self.move_player(&d2) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn move_player(&mut self, move_dir: &MoveDir) -> bool {
        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(pl) = self.get_player_mut() {
            let (dx, dy) = move_dir.to_vec();
            pl.get_pos_mut().0 += dx as u16;
            pl.get_pos_mut().1 += dy as u16;

            new_pos_and_dir = Some((pl.pos.clone(), (dx, dy)));
        }

        if let Some((pos, dir)) = new_pos_and_dir {
            let id = self.blocks.get(pos.0 as usize).and_then(|x| x.get(pos.1 as usize)).map(|x| x.get_id()).unwrap_or(0);
            let mut blkf = block::BLOCK_FUNCS.lock().unwrap();

            match blkf.get(id) {
                Some(f) => {
                    if !f(self) {
                        if let Some(pl) = self.get_player_mut() {
                            pl.get_pos_mut().0 -= dir.0 as u16;
                            pl.get_pos_mut().1 -= dir.1 as u16;
                            return true;
                        }
                    }
                }
                None => {}
            }
        }
        false
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
        self.auto = None;

        for x in 0..width {
            self.blocks.push(vec![]);
            for _ in 0..height {
                self.blocks[x].push(&*block::GROUND);
            }
        }

        let mut placed = vec![];
        for _ in 0..1000 {
            if rand() < 0.2 || placed.is_empty() {
                let x = (rand() * width as f64) as usize;
                let y = (rand() * height as f64) as usize;
                self.blocks[x][y] = &*block::WALL;
                placed.push((x, y, random_dir()));
            } else {
                let idx = (rand() * placed.len() as f64) as usize;
                let (x, y, mut dir) = placed[idx];

                if rand() < 0.05 {
                    dir = random_dir();
                }

                let dirv = dir.to_vec();

                let (nx, ny) = (x + dirv.0 as usize, y + dirv.1 as usize);
                
                let can_place = self.blocks.get(nx).and_then(|col| col.get(ny)).is_some();
                if can_place {
                    self.blocks[nx][ny] = &*block::WALL;
                    placed.push((nx, ny, dir));
                }
            }
        }
        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.blocks[x][y] = &*block::TELEPORTER;
    }
}

fn random_dir() -> MoveDir {
    match (rand() * 4.) as usize {
        0 => MoveDir::Left,
        1 => MoveDir::Right,
        2 => MoveDir::Up,
        3 => MoveDir::Down,
        _ => MoveDir::Left
    }   
}
