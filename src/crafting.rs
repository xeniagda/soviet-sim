use block;
use block::Block;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub out: Block,
    pub needed: Vec<(Block, u64)>,
}

lazy_static! {
    pub static ref RECIPES: Vec<Recipe> = vec![
        Recipe {
            out: block::MOVER.clone(),
            needed: vec![(block::WALL.clone(), 20), (block::BOMB.clone(), 1)]
        },
        Recipe {
            out: block::BOMB.clone(),
            needed: vec![(block::WALL.clone(), 50), (block::STONE.clone(), 10)]
        },
    ];
}
