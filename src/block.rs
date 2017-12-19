use world::World;
use std::sync::Mutex;
use ext::*;
use shape::Shape;

lazy_static! {
    pub static ref BLOCK_FUNCS: Mutex<Vec<fn(&mut World) -> bool>>
        = Mutex::new(vec![|_| { false }]);
}

pub struct Block {
    shape: Shape,
    id: usize,
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.id == other.id
    }
}
impl Eq for Block {}

impl Block {
    fn new(shape: Shape, on_walk: fn(&mut World) -> bool) -> Block {
        let mut blkf = BLOCK_FUNCS.lock().unwrap();
        blkf.push(on_walk);

        Block {
            id: blkf.len() - 1,
            shape: shape
        }
    }


    #[inline]
    pub fn get_id(&self) -> usize { self.id }

    #[inline]
    pub fn get_shape(&self) -> Shape { self.shape }
}

lazy_static! {
    pub static ref GROUND:     Block = Block::new(Shape::new('.', (128, 128, 128), (0, 0, 0)), |_| { true } );
    pub static ref WALL:       Block = Block::new(Shape::new('#', (202, 195, 210), (0, 0, 0)), |_| { false } );
    pub static ref TELEPORTER: Block = Block::new(Shape::new('%', (255, 30, 255), (0, 100, 0)), |world| { let (w, h) = (world.blocks.len(), world.blocks[0].len()); world.generate(w, h); true } );

}
