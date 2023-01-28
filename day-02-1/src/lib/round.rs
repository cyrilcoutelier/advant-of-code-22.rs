use crate::player_move::{create_from_char, get_fight_score, PlayerMove};

pub struct Round {
    my_move: PlayerMove,
    their_move: PlayerMove,
}

impl Round {
    pub fn new(my_move: PlayerMove, their_move: PlayerMove) -> Self {
        Round {
            my_move,
            their_move,
        }
    }

    pub fn get_score(&self) -> i32 {
        self.my_move.get_move_score() + get_fight_score(&self.my_move, &self.their_move)
    }
}

pub fn parse_str(str: &str) -> Round {
    let my_char = str.chars().nth(2).unwrap();
    let their_char = str.chars().nth(0).unwrap();
    let my_move = create_from_char(my_char);
    let their_move = create_from_char(their_char);
    Round::new(my_move, their_move)
}
