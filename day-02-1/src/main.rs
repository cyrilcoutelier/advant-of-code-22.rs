use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use day020::round::parse_str;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = &args[1];
  let path = Path::new(path);
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines();
  let mut score = 0;

  lines.for_each(|line| {
    match line {
      Ok(line_str) => {
        let round = parse_str(&line_str);
        score += round.get_score();
      },
      Err(e) => println!("Could not parse line: {}", e),
    }
  });
  println!("The score is {}", score);
}
