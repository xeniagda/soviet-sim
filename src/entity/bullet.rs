use world::World;
use shape::Shape;
use move_dir::MoveDir;
use inventory::InventoryItem;

use super::{Entity, EntityWrapper};


#[derive(PartialEq, Eq, Clone)]
pub struct Bullet {
    pub pos: (u16, u16),
    pub dir: MoveDir,
}

impl Bullet {
    pub fn new(pos: (u16, u16), dir: MoveDir) -> Bullet {
        Bullet {
            pos: pos,
            dir: dir
        }
    }

}

impl Entity for Bullet {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape {
        let ch = self.dir.to_ch();

        Shape::new(ch, (255, 255, 255), (0, 0, 0))
    }

    fn get_name(&self) -> String { "Bullet".into() }

    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        let dir =
            if let Some(EntityWrapper::WBullet(ref mut this)) = world.entities.get_mut(&en_id) {
                this.dir
            } else {
                return;
            };

        let move_fn =
            if let Some(enw) = world.entities.get_mut(&en_id) {
                enw.get_move_fn()
            } else {
                return;
            };

        move_fn(world, en_id, dir);
    }

    fn on_collision(world: &mut World, me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(enw) = world.entities.get_mut(&other_id) {
            match enw {
                EntityWrapper::WPlayer(ref mut pl) => {
                    pl.pick_up(InventoryItem::Bullet);
                }
                _ => {
                    enw.get_hurt_fn()(world, other_id, 1);
                }
            }
        }

        world.entities.remove(&me_id);

        false
    }

}

