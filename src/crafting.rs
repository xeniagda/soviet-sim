use block;
use block::Block;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub out: Block,
    pub needed: Vec<(Block, usize)>,
}

lazy_static! {
    pub static ref RECIPES: Vec<Recipe> = vec![
        Recipe {
            out: block::MOVER.clone(),
            needed: vec![(block::WALL.clone(), 20), (block::TELEPORTER.clone(), 1)]
        }
    ];
}
