use block;
use inventory::{InventoryItem, PICKAXE, SUPER_BOOTS};

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
                (InventoryItem::Bullet, 7),
                (InventoryItem::Block(block::STONE.clone()), 10),
            ]
        },
        Recipe {
            out: InventoryItem::Bullet,
            needed: vec![
                (InventoryItem::Block(block::WALL.clone()), 5),
            ]
        },
        Recipe {
            out: SUPER_BOOTS,
            needed: vec![
                (InventoryItem::Block(block::MOVER.clone()), 2),
                (InventoryItem::Bullet, 10),
            ]
        },
        Recipe {
            out: PICKAXE,
            needed: vec![
                (InventoryItem::Bomb, 1),
                (InventoryItem::Block(block::MOVER.clone()), 2),
                (InventoryItem::Block(block::WALL.clone()), 8),
                (InventoryItem::Bullet, 7),
                (InventoryItem::Block(block::STONE.clone()), 32),
            ]
        },
    ];
}
