use world::World;

pub struct Block {
    ch: char,
    col: (u8, u8, u8),
    bg: (u8, u8, u8),
    pub on_walk: fn(&mut World) -> bool, // When the player moves to this tile, what should happen? Returns if the block is passable
    id: u64
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.id == other.id
    }
}
impl Eq for Block {}

impl Block {
    #[inline]
    pub fn get_ch(&self) -> char { self.ch }
    #[inline]
    pub fn get_col(&self) -> (u8, u8, u8) { self.col }
    #[inline]
    pub fn get_bg(&self) -> (u8, u8, u8) { self.bg }
}

pub const GROUND:     Block = Block { id: 0, ch: '.', col: (128, 128, 128), bg: (0, 0, 0), on_walk: |_| { true } };
pub const WALL:       Block = Block { id: 1, ch: '#', col: (202, 195, 210), bg: (0, 0, 0), on_walk: |_| { false } };
pub const TELEPORTER: Block = Block { id: 2, ch: '%', col: (255, 30, 255), bg: (0, 100, 0), on_walk: |world| { world.get_player_mut().unwrap().pos = (0,0); true } };
