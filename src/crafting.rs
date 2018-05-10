use block;
use inventory::InventoryItem;

#[derive(Clone)]
pub struct Recipe {
    pub out: InventoryItem,
    pub needed: Vec<(InventoryItem, u64)>,
}

lazy_static! {
    pub static ref RECIPES: Vec<Recipe> = vec![
        Recipe {
            out: InventoryItem::Block(block::MOVER.clone()),
            needed: vec![
                (InventoryItem::Block(block::WALL.clone()), 20),
                (InventoryItem::Bomb, 1),
            ]
        },
        Recipe {
            out: InventoryItem::Bomb,
            needed: vec![
                (InventoryItem::Block(block::WALL.clone()), 50),
                (InventoryItem::Block(block::STONE.clone()), 10),
            ]
        },
        Recipe {
            out: InventoryItem::Bullet,
            needed: vec![
                (InventoryItem::Block(block::WALL.clone()), 5),
            ]
        },
    ];
}
