use level::Level;
use shape::Shape;
use move_dir::MoveDir;
use inventory::InventoryItem;
use block;

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

    fn tick(level: &mut Level, en_id: u64) where Self: Sized {
        let dir =
            if let Some(EntityWrapper::WBullet(ref mut this)) = level.entities.get_mut(&en_id) {
                this.dir
            } else {
                return;
            };

        let move_fn =
            if let Some(enw) = level.entities.get_mut(&en_id) {
                enw.get_move_fn()
            } else {
                return;
            };

        move_fn(level, en_id, dir);
    }

    fn move_dir(level: &mut Level, en_id: u64, dir: MoveDir) -> bool
        where Self: Sized {

        let (dx, dy) = dir.to_vec();

        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(en) = level.entities.get_mut(&en_id) {
            en.get_pos_mut().0 += dx as u16;
            en.get_pos_mut().1 += dy as u16;

            new_pos_and_dir = Some((en.get_pos().clone(), (dx, dy)));
        }


        if let Some((pos, dir)) = new_pos_and_dir {
            let passable = level.blocks.get(pos.0 as usize)
                        .and_then(|x| x.get(pos.1 as usize))
                        .map(|x| x.is_passable())
                        .unwrap_or(true);

            if !passable {
                level.blocks[pos.0 as usize][pos.1 as usize] = block::GROUND.clone();
                level.entities.remove(&en_id);
                return true;
            } else {
                let id = level.blocks.get(pos.0 as usize)
                    .and_then(|x| x.get(pos.1 as usize))
                    .map(|x| x.get_id())
                    .unwrap_or(0);

                let blkf = block::BLOCK_FUNCS.lock().unwrap();

                match blkf.get(id) {
                    Some(f) => {
                        f(level, en_id);
                    }
                    None => {}
                }
            }
            for k in level.entities.clone().keys() {
                if k != &en_id && level.entities.get(k).map(|x| x.get_pos()) == Some(pos) {
                    let mut collided = false;

                    let f = level.entities.get(k).unwrap().get_collision_fn();

                    if !f(level, *k, en_id) {
                        if let Some(en) = level.entities.get_mut(&en_id) {
                            en.get_pos_mut().0 -= dir.0 as u16;
                            en.get_pos_mut().1 -= dir.1 as u16;
                            collided = true;
                        }
                    }


                    if let Some(f) = level.entities.get(&en_id).map(|x| x.get_collision_fn()) {
                        if !f(level, en_id, *k) {
                            if let Some(en) = level.entities.get_mut(&en_id) {
                                if !collided {
                                    en.get_pos_mut().0 -= dir.0 as u16;
                                    en.get_pos_mut().1 -= dir.1 as u16;
                                }
                                collided = true;
                            }
                        }
                    }
                    if collided {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn on_collision(level: &mut Level, me_id: u64, other_id: u64) -> bool
        where Self: Sized {

        if let Some(enw) = level.entities.get_mut(&other_id) {
            match enw {
                EntityWrapper::WPlayer(ref mut pl) => {
                    pl.pick_up(InventoryItem::Bullet);
                }
                EntityWrapper::WBomb(_) => {
                }
                _ => {
                    enw.get_hurt_fn()(level, other_id, 1);
                }
            }
        }

        level.entities.remove(&me_id);

        false
    }

}

