use world::{MoveDir};
use shape::Shape;

use std::ops::{Deref, DerefMut};

pub trait Entity {

    fn tick(&mut self) {
    }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    fn move_dir(&mut self, dir: MoveDir/*, world: &mut World */) /* -> bool */ {
        let (dx, dy) = dir.to_vec();
        self.get_pos_mut().0 += dx as u16;
        self.get_pos_mut().1 += dy as u16;

        // let id = world.blocks.get(self.get_pos().0 as usize).and_then(|x| x.get(self.get_pos().1 as usize)).map(|x| x.get_id()).unwrap_or(0);
        // let blkf = block::BLOCK_FUNCS.lock().unwrap();

        // match blkf.get(id) {
        //     Some(f) => {
        //         if !f(world) {
        //             self.get_pos_mut().0 -= dx as u16;
        //             self.get_pos_mut().1 -= dy as u16;
        //             true
        //         } else {
        //             false
        //         }
        //     }
        //     None => { false }
        // }
    }

    fn get_shape(&self) -> Shape;

}


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Player {
    pub pos: (u16, u16),
    pub shape: Shape
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { self.shape }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Josef {
    pub pos: (u16, u16),
    pub shape: Shape
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
    fn get_shape(&self) -> Shape { self.shape }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum EntityWrapper {
    WPlayer(Player),
    WJosef(Josef)
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
