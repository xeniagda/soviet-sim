use world::{MoveDir, World, random_dir};
use shape::Shape;
use ext::*;
use block;

use std::ops::{Deref, DerefMut};

pub trait Entity {

    fn tick(_world: &mut World, _en_id: u64) where Self: Sized { }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    /// Try to move an entity in a specific direction
    /// Returns if the entity collided and had to move back
    fn move_dir(world: &mut World, en_id: u64, dir: MoveDir) -> bool
        where Self: Sized {

        let (dx, dy) = dir.to_vec();

        let mut new_pos_and_dir: Option<((u16, u16), (i8, i8))> = None;

        if let Some(en) = world.entities.get_mut(&en_id) {
            en.get_pos_mut().0 += dx as u16;
            en.get_pos_mut().1 += dy as u16;

            new_pos_and_dir = Some((en.get_pos().clone(), (dx, dy)));
        }


        if let Some((pos, dir)) = new_pos_and_dir {
            log(&format!("Moved to {:?} in {:?}", pos, dir));

            let id = world.blocks.get(pos.0 as usize)
                        .and_then(|x| x.get(pos.1 as usize))
                        .map(|x| x.get_id())
                        .unwrap_or(0);

            let mut blkf = block::BLOCK_FUNCS.lock().unwrap();

            log(&format!("Id: {}", id));

            match blkf.get(id) {
                Some(f) => {
                    if !f(world, en_id) {
                        log("Moving back");
                        if let Some(en) = world.entities.get_mut(&en_id) {
                            en.get_pos_mut().0 -= dir.0 as u16;
                            en.get_pos_mut().1 -= dir.1 as u16;
                            return true;
                        }
                    }
                }
                None => {}
            }
        }
        false
    }

    fn get_shape(&self) -> Shape;

}


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Player {
    pub pos: (u16, u16),
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { Shape { ch: '@', col: (0, 255, 0), bg: (0, 0, 0) } }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Josef {
    pub pos: (u16, u16),
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
    fn get_shape(&self) -> Shape { Shape { ch: 'J', col: (255, 0, 0), bg: (0, 0, 0) } }

    fn tick(world: &mut World, en_id: u64) where Self: Sized {
        if rand() < 0.1 {
            for _ in 0..10 {
                if Josef::move_dir(world, en_id, random_dir()) {
                    break;
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EntityWrapper {
    WPlayer(Player),
    WJosef(Josef)
}

impl EntityWrapper {
    pub fn get_tick_fn(&self) -> impl Fn(&mut World, u64) {
        use self::EntityWrapper::*;
        match *self {
            WPlayer(_) => Player::tick,
            WJosef(_) => Josef::tick,
        }
    }

    pub fn get_move_fn(&self) -> impl Fn(&mut World, u64, MoveDir) -> bool {
        use self::EntityWrapper::*;
        match *self {
            WPlayer(_) => Player::move_dir,
            WJosef(_) => Josef::move_dir,
        }
    }
}

impl Deref for EntityWrapper {
    type Target = Entity;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref e) => e,
            WJosef(ref e) => e,
        }
    }
}

impl DerefMut for EntityWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref mut e) => e,
            WJosef(ref mut e) => e,
        }
    }
}
