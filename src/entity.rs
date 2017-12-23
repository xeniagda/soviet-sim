use world::{World, MoveDir};
use shape::Shape;

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

    fn on_collision<E: Entity>(&mut self, other: &mut E, world: &mut World) {}
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

    pub fn tick(&mut self) {
        use self::EntityWrapper::*;

        match self {
            &mut WPlayer(ref mut x) => x.tick(),
            &mut WJosef(ref mut x) => x.tick(),
        }
    }

    pub fn get_pos(&self) -> (u16, u16) {
        use self::EntityWrapper::*;

        match self {
            &WPlayer(ref x) => x.get_pos(),
            &WJosef(ref x) => x.get_pos(),
        }
    }

    pub fn get_pos_mut(&mut self) -> &mut (u16, u16) {
        use self::EntityWrapper::*;

        match self {
            &mut WPlayer(ref mut x) => x.get_pos_mut(),
            &mut WJosef(ref mut x) => x.get_pos_mut(),
        }
    }

    pub fn get_shape(&self) -> Shape {
        use self::EntityWrapper::*;

        match self {
            &WPlayer(ref x) => x.get_shape(),
            &WJosef(ref x) => x.get_shape(),
        }
    }

    pub fn on_collision<E: Entity>(&mut self, other: &mut E, mut world: &mut World) {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref mut x) => x.on_collision(other, &mut world),
            WJosef(ref mut x) => x.on_collision(other, &mut world),
        }
    }
}

