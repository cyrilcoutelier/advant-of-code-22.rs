use crate::outcome::Outcome;
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

fn get_complementary_move(their_move: PlayerMove, outcome: Outcome) -> PlayerMove {
    let their_points = their_move.get_move_score();
    let diff = match outcome {
        Outcome::DRAW => 0,
        Outcome::LOSE => -1,
        Outcome::WIN => 1,
    };
    let mut my_points = their_points + diff;
    if my_points < 1 {
        my_points += 3;
    }
    if my_points > 3 {
        my_points -= 3;
    }
    PlayerMove::from_points(my_points)
}

#[cfg(test)]
mod tests {
    use crate::{outcome::Outcome, player_move::PlayerMove, round::get_complementary_move};

    #[test]
    fn paper_lose_rock() {
        // When
        let result = get_complementary_move(PlayerMove::Paper, Outcome::LOSE);

        // Then
        assert_eq!(result, PlayerMove::Rock);
    }

    #[test]
    fn rock_lose_scissor() {
        // When
        let result = get_complementary_move(PlayerMove::Rock, Outcome::LOSE);

        // Then
        assert_eq!(result, PlayerMove::Scissor);
    }

    #[test]
    fn scissor_lose_paper() {
        // When
        let result = get_complementary_move(PlayerMove::Scissor, Outcome::LOSE);

        // Then
        assert_eq!(result, PlayerMove::Paper);
    }

    #[test]
    fn paper_win_scissor() {
        // When
        let result = get_complementary_move(PlayerMove::Paper, Outcome::WIN);

        // Then
        assert_eq!(result, PlayerMove::Scissor);
    }

    #[test]
    fn scissor_win_rock() {
        // When
        let result = get_complementary_move(PlayerMove::Scissor, Outcome::WIN);

        // Then
        assert_eq!(result, PlayerMove::Rock);
    }

    #[test]
    fn rock_win_paper() {
        // When
        let result = get_complementary_move(PlayerMove::Rock, Outcome::WIN);

        // Then
        assert_eq!(result, PlayerMove::Paper);
    }

    #[test]
    fn rock_draw_rock() {
        // When
        let result = get_complementary_move(PlayerMove::Rock, Outcome::DRAW);

        // Then
        assert_eq!(result, PlayerMove::Rock);
    }
}

pub fn parse_str(str: &str) -> Round {
    let their_char = str.chars().nth(0).unwrap();
    let their_move = create_from_char(their_char);

    let outcome_char = str.chars().nth(2).unwrap();
    let outcome = Outcome::from_char(outcome_char);

    let my_move = get_complementary_move(their_move, outcome);

    Round::new(my_move, their_move)
}
