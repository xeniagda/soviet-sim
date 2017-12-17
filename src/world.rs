use ext::*;
use controls::Action;
use block;

#[derive(PartialEq, Eq)]
pub struct World {
    pub blocks: Vec<Vec<block::Block>>,
    pub player_pos: (u16, u16), // x, y
}

impl World {
    pub fn new(blocks: Vec<Vec<block::Block>>, player_pos: (u16, u16)) -> World {
        World {
            blocks: blocks,
            player_pos: player_pos,
        }
    }

    pub fn tick(&mut self) {
    }

    pub fn do_action(&mut self, action: &Action) {
        let move_dir: Option<(i8, i8)> = match *action {
                Action::MoveDown  => { Some((0, 1)) }
                Action::MoveUp    => { Some((0, -1)) }
                Action::MoveLeft  => { Some((-1, 0)) }
                Action::MoveRight => { Some((1, 0)) }
                _                 => { None }
            };
        if let Some((dx, dy)) = move_dir {
            self.player_pos.0 += dx as u16;
            self.player_pos.1 += dy as u16;
            if !(self.blocks[self.player_pos.0 as usize][self.player_pos.1 as usize].on_walk)(self) {
                self.player_pos.0 -= dx as u16;
                self.player_pos.1 -= dy as u16;
            }
        }

    }

    pub fn draw(&self) {
        // Draw world
        log(&format!("RGB: {:?}", self.blocks[0][0].get_col()));
        for (x, col) in self.blocks.iter().enumerate() {
            for (y, block) in col.iter().enumerate() {
                put_char((x as u16, y as u16), block.get_ch(), block.get_col());
            }
        }

        // Draw player
        put_char(self.player_pos, '@', (0, 255, 0));
    }

    pub fn generate(&mut self, width: usize, height: usize) {
        for x in 0..width {
            self.blocks.push(vec![]);
            for _ in 0..height {
                self.blocks[x].push(block::GROUND);
            }
        }
        for _ in 0..20 {
            let x = (rand() * width as f64) as usize;
            let y = (rand() * height as f64) as usize;
            self.blocks[x][y] = block::WALL;
        }
        let x = (rand() * width as f64) as usize;
        let y = (rand() * height as f64) as usize;
        self.blocks[x][y] = block::TELEPORTER;
    }
}
