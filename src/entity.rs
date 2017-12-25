use world::{MoveDir, World};
use shape::Shape;

use std::ops::{Deref, DerefMut};

pub trait Entity {

    fn tick(world: &mut World, en_id: u64) where Self: Sized { }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    fn move_dir(&mut self, dir: MoveDir) {
        let (dx, dy) = dir.to_vec();
        self.get_pos_mut().0 += dx as u16;
        self.get_pos_mut().1 += dy as u16;
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

impl EntityWrapper {
    pub fn get_tick_fn(&self) -> impl Fn(&mut World, u64) {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(_) => Player::tick,
            WJosef(_) => Josef::tick,
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
