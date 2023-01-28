#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum PlayerMove {
    Rock,
    Paper,
    Scissor,
}

impl PlayerMove {
    pub fn get_move_score(&self) -> i32 {
        match self {
            PlayerMove::Rock => 1,
            PlayerMove::Paper => 2,
            PlayerMove::Scissor => 3,
        }
    }
    pub fn from_points(points: i32) -> Self {
        match points {
            1 => PlayerMove::Rock,
            2 => PlayerMove::Paper,
            3 => PlayerMove::Scissor,
            _ => panic!("Cannot build PlayerMove from points: {}", points),
        }
    }
}

pub fn create_from_char(c: char) -> PlayerMove {
    match c {
        'A' | 'X' => PlayerMove::Rock,
        'B' | 'Y' => PlayerMove::Paper,
        'C' | 'Z' => PlayerMove::Scissor,
        _ => panic!("Unsupported letter: {}", c),
    }
}

pub fn get_fight_score(my_move: &PlayerMove, their_move: &PlayerMove) -> i32 {
    let my_score = my_move.get_move_score();
    let their_score = their_move.get_move_score();
    let mut diff = my_score - their_score;
    if diff < 0 {
        diff += 3;
    }
    match diff {
        0 => 3,
        1 => 6,
        2 => 0,
        _ => panic!("Should be unreachable"),
    }
}
