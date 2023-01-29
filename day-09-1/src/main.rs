use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type PosUnit = i16;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => panic!("Invalid character for direction: {}", c),
        }
    }
}

struct Command {
    direction: Direction,
    nb_steps: usize,
}

impl Command {
    fn from_str(text: &str) -> Self {
        let mut letters = text.split(' ');

        let first_str = letters.next().unwrap();
        let first_char = first_str.chars().next().unwrap();
        let direction = Direction::from_char(first_char);

        let second_str = letters.next().unwrap();
        let nb_steps = second_str.parse().unwrap();
        Command {
            direction,
            nb_steps,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    x: PosUnit,
    y: PosUnit,
}

impl Position {
    fn is_too_far(&self) -> bool {
        self.x.abs() > 1 || self.y.abs() > 1
    }

    fn sub(&self, b: &Self) -> Self {
        let x = self.x - b.x;
        let y = self.y - b.y;
        Position { x, y }
    }

    fn get_catchup_move(&self) -> Self {
        let x = if self.x.abs() >= 2 {
            self.x / 2
        } else {
            self.x
        };
        let y = if self.y.abs() >= 2 {
            self.y / 2
        } else {
            self.y
        };
        Position { x, y }
    }

    fn apply_diff(&mut self, diff: &Self) {
        self.x += diff.x;
        self.y += diff.y;
    }
}

struct Map {
    pos_head: Position,
    pos_tail: Position,
    pos_tail_history: HashSet<Position>,
}

impl Map {
    fn new() -> Self {
        let mut pos_tail_history = HashSet::new();
        let pos_tail = Position { x: 0, y: 0 };
        pos_tail_history.insert(pos_tail.clone());
        Map {
            pos_head: pos_tail.clone(),
            pos_tail,
            pos_tail_history,
        }
    }

    fn process_command(&mut self, command: &Command) {
        for _ in 0..command.nb_steps {
            self.process_move(&command.direction);
        }
    }

    fn process_move(&mut self, direction: &Direction) {
        self.move_head(direction);
        self.update_tail();
        self.record_tail_pos();
    }

    fn move_head(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.pos_head.y += 1,
            Direction::Down => self.pos_head.y -= 1,
            Direction::Right => self.pos_head.x += 1,
            Direction::Left => self.pos_head.x -= 1,
        };
    }

    fn update_tail(&mut self) {
        let diff = self.pos_head.sub(&self.pos_tail);
        if !diff.is_too_far() {
            return;
        }
        let catchup_move = diff.get_catchup_move();
        self.pos_tail.apply_diff(&catchup_move);
    }

    fn record_tail_pos(&mut self) {
        self.pos_tail_history.insert(self.pos_tail.clone());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut map = Map::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            let command = Command::from_str(&line);
            map.process_command(&command);
        });

    let result = map.pos_tail_history.len();
    println!("The result is `{}`", result);
}
