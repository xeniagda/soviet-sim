use ext::put_char;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape {
    pub ch: char,
    pub col: (u8, u8, u8),
    pub bg: (u8, u8, u8)
}

impl Shape {
    pub fn new(ch: char, col: (u8, u8, u8), bg: (u8, u8, u8)) -> Shape {
        Shape { ch: ch, col: col, bg: bg }
    }

    pub fn empty() -> Shape {
        Shape::new(' ', (0, 0, 0), (0, 0, 0))
    }

    pub fn draw(&self, pos: (u16, u16)) {
        put_char(pos, &self);
    }
}


