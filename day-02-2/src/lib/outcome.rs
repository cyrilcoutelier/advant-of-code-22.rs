#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Outcome {
    DRAW,
    WIN,
    LOSE,
}

impl Outcome {
    pub fn from_char(c: char) -> Self {
        match c {
            'X' => Outcome::LOSE,
            'Y' => Outcome::DRAW,
            'Z' => Outcome::WIN,
            _ => panic!("No outcome possible for char {}", c),
        }
    }

    pub fn to_point(&self) -> i32 {
        match self {
            Outcome::DRAW => 3,
            Outcome::WIN => 6,
            Outcome::LOSE => 0,
        }
    }
}
