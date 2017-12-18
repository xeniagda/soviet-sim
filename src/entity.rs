use world::World;

pub trait Entity {

    fn tick(&mut self, world: &mut World) {
    }

    fn get_pos(&self) -> (u16, u16);
    fn get_pos_mut(&mut self) -> &mut (u16, u16);

    fn on_collision<E: Entity>(&mut self, other: &mut E, world: &mut World) {}

    fn move_dir(&mut self, dir: (i8, i8), world: &mut World) {
        let pos = self.get_pos_mut();
        pos.0 += dir.0 as u16;
        pos.1 += dir.1 as u16;
    }

}


#[derive(PartialEq, Eq, Clone)]
pub struct Player {
    pub pos: (u16, u16)
}

impl Entity for Player {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Josef {
    pub pos: (u16, u16)
}

impl Entity for Josef {
    fn get_pos(&self) -> (u16, u16) { self.pos }
    fn get_pos_mut(&mut self) -> &mut (u16, u16) { &mut self.pos }
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

    pub fn on_collision<E: Entity>(&mut self, mut other: &mut E, mut world: &mut World) {
        use self::EntityWrapper::*;

        match *self {
            WPlayer(ref mut x) => x.on_collision(other, &mut world),
            WJosef(ref mut x) => x.on_collision(other, &mut world),
        }
    }
}

