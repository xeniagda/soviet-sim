use ext::rand;

pub const DIRECTIONS: [MoveDir; 4] = [ MoveDir::Up, MoveDir::Left, MoveDir::Down, MoveDir::Right ];


#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum MoveDir { Up, Left, Down, Right }

impl MoveDir {
    pub fn to_vec(&self) -> (i8, i8) {
        match self {
            &MoveDir::Up => (0, -1),
            &MoveDir::Down => (0, 1),
            &MoveDir::Left => (-1, 0),
            &MoveDir::Right => (1, 0),
        }
    }

    pub fn move_vec(&self, other: (u16, u16)) -> (u16, u16) {
        let (dx, dy) = self.to_vec();

        (other.0.wrapping_add(dx as u16), other.1.wrapping_add(dy as u16))
    }

    #[allow(unused)]
    pub fn rotate_cw(&self) -> MoveDir {
        match self {
            &MoveDir::Up => MoveDir::Right,
            &MoveDir::Right => MoveDir::Down,
            &MoveDir::Down => MoveDir::Left,
            &MoveDir::Left => MoveDir::Up,
        }
    }

    #[allow(unused)]
    pub fn rotate_180(&self) -> MoveDir {
        match self {
            &MoveDir::Up => MoveDir::Down,
            &MoveDir::Right => MoveDir::Left,
            &MoveDir::Down => MoveDir::Up,
            &MoveDir::Left => MoveDir::Right,
        }
    }

    #[allow(unused)]
    pub fn rotate_ccw(&self) -> MoveDir {
        match self {
            &MoveDir::Up => MoveDir::Left,
            &MoveDir::Right => MoveDir::Up,
            &MoveDir::Down => MoveDir::Right,
            &MoveDir::Left => MoveDir::Down,
        }
    }

    pub fn to_ch(&self) -> char {
        match self {
            &MoveDir::Up => '↑',
            &MoveDir::Down => '↓',
            &MoveDir::Left => '←',
            &MoveDir::Right => '→',
        }
    }
}

pub fn random_dir() -> MoveDir {
    match (rand() * 4.) as usize {
        0 => MoveDir::Left,
        1 => MoveDir::Right,
        2 => MoveDir::Up,
        3 => MoveDir::Down,
        _ => MoveDir::Left
    }
}
