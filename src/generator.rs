use std::collections::HashMap;

use block;
use difficulty::Difficulty;
use level::Level;
use entity::{EntityWrapper, Player, Josef};
use ext::*;
use move_dir::{random_dir, MoveDir};

pub struct Generator {
    pub width: usize,
    pub height: usize,
    pub block_probs: HashMap<block::Block, f64>,
    pub amount_of_walls: f64,
    pub new_pos_prob: f64,
    pub new_dir_prob: f64,
    pub entities: Vec<EntityWrapper>,
}

impl Generator {
    pub fn default_for_difficulty(diff: Difficulty, include_player: bool, include_josef: bool) -> Generator {
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
        Generator {
            width: 180,
            height: 111,
            block_probs: hashmap!{
                block::WALL.clone()   => 0.895,
                block::STONE.clone()  => 0.1,
                block::STAIRS.clone() => 0.005,
            },
            amount_of_walls: 10.0,
            new_pos_prob: 0.01,
            new_dir_prob: 0.05,
            entities: entities
        }
    }

    pub fn generate(&self, lvl: &mut Level) {
        log("Generating!");

        lvl.entities = HashMap::new();
        lvl.blocks = vec![];

        let total: f64 = self.block_probs.values().sum();

        for x in 0..self.width {
            lvl.blocks.push(vec![]);
            for _ in 0..self.height {
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

        let mut placed: Vec<(u16, u16, MoveDir)> = vec![];
        for _ in 0..(self.amount_of_walls * self.width as f64 * self.height as f64) as usize {
            if rand() < self.new_pos_prob || placed.is_empty() {
                let x = (rand() * self.width as f64) as u16;
                let y = (rand() * self.height as f64) as u16;
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

        for en in self.entities.iter() {
            let mut en = en.clone();
            let idx = (rand() * placed.len() as f64) as usize;
            let (x, y, _) = placed[idx];
            placed.remove(idx);
            *en.get_pos_mut() = (x as u16, y as u16);
            lvl.add_entity(
                en
                );
        }
        log("Done!");
    }
}
