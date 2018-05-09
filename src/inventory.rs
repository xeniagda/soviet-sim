use block::{Block, GROUND};
use world::World;
use entity::EntityWrapper;
use shape::Shape;

#[derive(PartialEq, Eq, Clone)]
pub enum InventoryItem {
    Block(Block),
    Entity(EntityWrapper)
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
            InventoryItem::Entity(ref enw) => {
                world.add_entity(enw.clone());
            }
        }
        false
    }

    pub fn get_shape(&self) -> Shape {
        match self {
            InventoryItem::Block(ref block) => block.shape,
            InventoryItem::Entity(ref en) => en.get_shape(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            InventoryItem::Block(ref block) => block.name.clone(),
            InventoryItem::Entity(ref en) => en.get_name(),
        }
    }
}
