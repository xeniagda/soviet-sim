

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum Difficulty {
    Easy = 0, Medium = 1, Hard = 2, Extreme = 3, Reality = 4
}

impl Difficulty {
    pub fn harder(self) -> Difficulty {
        match self {
            Difficulty::Easy    => Difficulty::Medium,
            Difficulty::Medium  => Difficulty::Hard,
            Difficulty::Hard    => Difficulty::Extreme,
            Difficulty::Extreme => Difficulty::Reality,
            Difficulty::Reality => Difficulty::Reality,
        }
    }

    pub fn easier(self) -> Difficulty {
        match self {
            Difficulty::Reality => Difficulty::Extreme,
            Difficulty::Extreme => Difficulty::Hard,
            Difficulty::Hard    => Difficulty::Medium,
            Difficulty::Medium  => Difficulty::Easy,
            Difficulty::Easy    => Difficulty::Easy,
        }
    }

    #[inline(always)]
    pub fn get_josef_speed(self) -> u16 {
        match self {
            Difficulty::Easy => 1000,
            Difficulty::Medium => 700,
            Difficulty::Hard => 400,
            Difficulty::Extreme => 200,
            Difficulty::Reality => 50
        }
    }

    #[inline(always)]
    pub fn get_police_speed(self) -> u16 {
        match self {
            Difficulty::Easy => 20,
            Difficulty::Medium => 15,
            Difficulty::Hard => 12,
            Difficulty::Extreme => 10,
            Difficulty::Reality => 7
        }
    }
}
