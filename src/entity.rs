use world::World;
use shape::Shape;

pub trait Entity {

    fn tick(&mut self, world: &mut World) {
    }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    fn get_shape(&self) -> Shape;

    fn on_collision<E: Entity>(&mut self, other: &mut E, world: &mut World) {}
}


#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16),
    pub shape: Shape
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }

    fn get_shape(&self) -> Shape { self.shape }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub pos: (u16, u16),
    pub shape: Shape
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
    fn get_shape(&self) -> Shape { self.shape }
}

#[derive(PartialEq, Eq, Clone)]
pub enum EntityWrapper {
    WPlayer(Player),
    WJosef(Josef)
}

impl EntityWrapper {

    pub fn tick(&mut self, mut world: &mut World) {
        use self::EntityWrapper::*;

        match self {
            &mut WPlayer(ref mut x) => x.tick(&mut world),
            &mut WJosef(ref mut x) => x.tick(&mut world),
        }
    }

    pub fn get_pos(&self) -> (u16, u16) {
        use self::EntityWrapper::*;

        match self {
            &WPlayer(ref x) => x.get_pos(),
            &WJosef(ref x) => x.get_pos(),
        }
    }

    pub fn get_shape(&self) -> Shape {
        use self::EntityWrapper::*;

        match self {
            &WPlayer(ref x) => x.get_shape(),
            &WJosef(ref x) => x.get_shape(),
        }
    }

    pub fn on_collision<E: Entity>(&mut self, mut other: &mut E, mut world: &mut World) {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref mut x) => x.on_collision(other, &mut world),
            WJosef(ref mut x) => x.on_collision(other, &mut world),
        }
    }
}

