use block::{Block, GROUND};
use world::World;
use entity::{EntityWrapper, Bomb, Bullet};
use shape::Shape;
use move_dir::MoveDir;

pub const SUPER_BOOTS: InventoryItem = InventoryItem::SuperBoots(1000, 1001);
pub const PICKAXE: InventoryItem = InventoryItem::Pickaxe(1000, 1001);

#[derive(PartialEq, Eq, Clone)]
pub enum InventoryItem {
    Block(Block),
    Bomb,
    Bullet,
    // (durability_left, max_durability)
    SuperBoots(u16, u16),
    Pickaxe(u16, u16),
}

impl InventoryItem {
    pub fn place_pos(&self, world: &mut World, pos: (u16, u16), dir: MoveDir) -> bool {
        match self {
            InventoryItem::Block(ref block) => {
                if let Some(last) =
                    world.blocks.get_mut(pos.0 as usize).and_then(|x| x.get_mut(pos.1 as usize)) {

                    if *last == GROUND.clone() {
                        *last = block.clone();
                        return true;
                    }
                }
            }
            InventoryItem::Bomb => {
                world.add_entity(EntityWrapper::WBomb(Bomb::new(pos, 300)));
                return true;
            }
            InventoryItem::Bullet => {
                world.add_entity(EntityWrapper::WBullet(Bullet::new(pos, dir)));
                return true;
            }
            InventoryItem::SuperBoots(_, _) | InventoryItem::Pickaxe(_, _) => {}
        }
        false
    }

    pub fn get_shape(&self) -> Shape {
        match self {
            InventoryItem::Block(ref block) => block.get_shape(),
            InventoryItem::Bomb => Shape::new('B', (255, 30, 255), (0, 100, 0)),
            InventoryItem::Bullet => Shape::new('^', (255, 255, 255), (0, 0, 0)),
            InventoryItem::SuperBoots(durability, max) => {
                let d: u8 = ((255 * *durability as u32) / *max as u32) as u8;
                Shape::new('b', (255, 0, 255), (d, d, d))
            },
            InventoryItem::Pickaxe(durability, max) => {
                let d: u8 = ((255 * *durability as u32) / *max as u32) as u8;
                Shape::new('Y', (255, 0, 255), (d, d, d))
            },
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            InventoryItem::Block(ref block) => block.name.clone(),
            InventoryItem::Bomb => "Bomb".into(),
            InventoryItem::Bullet => "Bullet".into(),
            InventoryItem::SuperBoots(_, _) => "Super Boots".into(),
            InventoryItem::Pickaxe(_, _) => "Pickaxe".into(),
        }
    }
    pub fn get_desc(&self) -> String {
        match self {
            InventoryItem::Block(ref block) => block.desc.clone(),
            InventoryItem::Bomb => "Blows up enemies (and you)".into(),
            InventoryItem::Bullet => "Shoots things".into(),
            InventoryItem::SuperBoots(_, _) => "Makes you able to run very fast. Ctrl+Alt+Arrow key to use".into(),
            InventoryItem::Pickaxe(_, _) => "Makes you able to mine very fast. Alt+M+Arrow key to use".into(),
        }
    }
}
