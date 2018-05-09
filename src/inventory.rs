use block::{Block, GROUND};
use world::World;
use entity::{EntityWrapper, Bomb};
use shape::Shape;

#[derive(PartialEq, Eq, Clone)]
pub enum InventoryItem {
    Block(Block),
    Bomb,
}

impl InventoryItem {
    pub fn place_pos(&self, world: &mut World, pos: (u16, u16)) -> bool {
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
        }
        false
    }

    pub fn get_shape(&self) -> Shape {
        match self {
            InventoryItem::Block(ref block) => block.shape,
            InventoryItem::Bomb => Shape::new('B', (255, 30, 255), (0, 100, 0)),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            InventoryItem::Block(ref block) => block.name.clone(),
            InventoryItem::Bomb => "Bomb".into(),
        }
    }
}
