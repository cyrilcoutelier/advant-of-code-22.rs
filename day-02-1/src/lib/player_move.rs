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
  let diff = my_score - their_score;

  if diff == 0 {
    return 3;
  }
  if diff == 1 || diff == -2 {
    return 6;
  }
  if diff == -1 || diff == 2 {
    return 0;
  }
  panic!("Should be unreachable");
}
