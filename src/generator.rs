use std::collections::HashMap;

use block;
use block::Block;
use difficulty::Difficulty;
use level::Level;
use entity::{EntityWrapper, Player, Josef};
use ext::*;
use move_dir::{random_dir, MoveDir};


pub struct LevelGenerator {
    pub block_probs: HashMap<block::Block, f64>,
    pub entities: Vec<EntityWrapper>,
    pub space: SpaceGenerator,
}

pub enum SpaceGenerator {
    Paths(PathsGenerator),
    Cellular(CellularGenerator),
}

pub struct PathsGenerator {
    pub amount_of_walls: f64,
    pub new_pos_prob: f64,
    pub new_dir_prob: f64,
}

pub struct CellularGenerator {
    pub remove_prec: f64,
    pub wall_block: Block,
    pub s2_iters: usize,
    pub s3_iters: usize,
}

pub trait Generator {
    fn generate(&self, _width: usize, _height: usize, _level: &mut Level);
}

impl LevelGenerator {
    pub fn default_for_difficulty(diff: Difficulty, include_player: bool, include_josef: bool) -> LevelGenerator {
        let mut entities = vec![];
        if include_player {
            entities.push(
                EntityWrapper::WPlayer(
                    Player::new((0, 0), diff.get_start_health())
                )
            );
        }
        if include_josef {
            entities.push(
                EntityWrapper::WJosef(
                    Josef::new(
                        (0, 0),
                        diff.get_josef_police_rate(),
                        diff.get_josef_speed(),
                        diff.get_josef_health()
                    )
                )
            );
        }
        LevelGenerator {
            block_probs: hashmap!{
                block::WALL.clone()   => 0.895,
                block::STONE.clone()  => 0.1,
                block::STAIRS.clone() => 0.005,
            },
            entities: entities,
            space: SpaceGenerator::Paths(PathsGenerator {
                amount_of_walls: 10.0,
                new_pos_prob: 0.01,
                new_dir_prob: 0.05,
            }),
        }
    }
}

impl Generator for LevelGenerator {
    fn generate(&self, width: usize, height: usize, lvl: &mut Level) {
        log("Generating!");

        lvl.entities = HashMap::new();
        lvl.blocks = vec![];

        let total: f64 = self.block_probs.values().sum();

        for x in 0..width {
            lvl.blocks.push(vec![]);
            for _ in 0..height {
                let block_num = rand() * total;

                let mut upto = 0.;

                for (blk, prob) in self.block_probs.iter() {
                    upto += *prob;
                    if upto >= block_num {
                        lvl.blocks[x].push(blk.clone());
                        break;
                    }
                }

            }
        }

        self.space.generate(width, height, lvl);

        let mut space: Vec<(u16, u16)> = vec![];
        for x in 0..width {
            for y in 0..height {
                if lvl.blocks[x][y] == *block::GROUND {
                    space.push((x as u16, y as u16));
                }
            }
        }

        for en in self.entities.iter() {
            let mut en = en.clone();
            let idx = (rand() * space.len() as f64) as usize;
            let (x, y) = space.remove(idx);
            *en.get_pos_mut() = (x as u16, y as u16);
            lvl.add_entity(
                en
                );
        }


        log("Done!");
    }
}

impl Generator for SpaceGenerator {
    fn generate(&self, width: usize, height: usize, lvl: &mut Level) {
        match self {
            SpaceGenerator::Paths(gen) => gen.generate(width, height, lvl),
            SpaceGenerator::Cellular(gen) => gen.generate(width, height, lvl),
        }
    }
}

impl Generator for PathsGenerator {
    fn generate(&self, _width: usize, _height: usize, lvl: &mut Level) {
        let mut placed: Vec<(u16, u16, MoveDir)> = vec![];

        let width = lvl.blocks.len();
        let height = lvl.blocks.get(0).map(|x| x.len()).unwrap_or(0);

        for _ in 0..(self.amount_of_walls * width as f64 * height as f64) as usize {
            if rand() < self.new_pos_prob || placed.is_empty() {
                let x = (rand() * width as f64) as u16;
                let y = (rand() * height as f64) as u16;
                lvl.blocks[x as usize][y as usize] = block::GROUND.clone();
                placed.push((x, y, random_dir()));
            } else {
                let idx = (rand() * placed.len() as f64) as usize;
                let (x, y, mut dir) = placed[idx];

                if rand() < self.new_dir_prob {
                    dir = random_dir();
                }

                let dirv = dir.to_vec();

                let (nx, ny) = (x.wrapping_add(dirv.0 as u16), y.wrapping_add(dirv.1 as u16));

                let block_at = lvl.get_at((nx, ny));
                if block_at.map(|x| x.breakable == block::Breakability::Breakable).unwrap_or(false) {
                    lvl.blocks[nx as usize][ny as usize] = block::GROUND.clone();
                    placed.push((nx, ny, dir));
                }
            }
        }

    }

}
impl Generator for CellularGenerator {
    fn generate(&self, _width: usize, _height: usize, lvl: &mut Level) {
        let width = lvl.blocks.len();
        let height = lvl.blocks.get(0).map(|x| x.len()).unwrap_or(0);

        // Stage 1: Remove a precentage of the existing blocks
        for x in 0..width {
            for y in 0..height {
                if rand() < self.remove_prec {
                    lvl.blocks[x][y] = block::GROUND.clone();
                }
            }
        }

        // Stage 2: Do iterations of the B5678/S45678 and B5678/S5678

        for i in 0..self.s2_iters + self.s3_iters {
            let mut new_blocks = lvl.blocks.clone();

            let is_s3 = i >= self.s2_iters;

            for x in 0..width {
                for y in 0..height {
                    let mut neighbors = 0;
                    let alive = lvl.blocks[x][y] != block::GROUND.clone();
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            let rx = (x as isize) + dx;
                            let ry = (y as isize) + dy;

                            let rx = if rx < 0 { 0 } else { rx as usize };
                            let ry = if ry < 0 { 0 } else { ry as usize };

                            let rx = if rx > width  - 1 { width  - 1 } else { rx as usize };
                            let ry = if ry > height - 1 { height - 1 } else { ry as usize };

                            if lvl.blocks[rx][ry] != block::GROUND.clone() {
                                neighbors += 1;
                            }
                        }
                    }

                    if !alive && neighbors > 4 {
                        new_blocks[x][y] = self.wall_block.clone();
                    }
                    if is_s3 {
                        if alive && neighbors < 4 {
                            new_blocks[x][y] = block::GROUND.clone();
                        }
                    } else {
                        if alive && neighbors < 5 {
                            new_blocks[x][y] = block::GROUND.clone();
                        }
                    }
                }
            }
            lvl.blocks = new_blocks;
        }

    }
}
