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
            out: InventoryItem::SuperBoots(1000, 1001),
            needed: vec![
                (InventoryItem::Block(block::MOVER.clone()), 2),
                (InventoryItem::Bullet, 10),
            ]
        },
        Recipe {
            out: InventoryItem::Pickaxe(500, 501),
            needed: vec![
                (InventoryItem::SuperBoots(1000, 1001), 1),
                (InventoryItem::Block(block::MOVER.clone()), 2),
                (InventoryItem::Bomb, 4),
                (InventoryItem::Block(block::WALL.clone()), 8),
                (InventoryItem::Bullet, 16),
                (InventoryItem::Block(block::STONE.clone()), 32),
            ]
        },
    ];
}
