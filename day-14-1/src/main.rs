#![feature(entry_insert)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq, Clone)]
enum Tile {
    Rock,
    Sand,
    Air,
}

struct PosDiff {
    x: isize,
    y: isize,
}

impl PosDiff {
    fn to_normal(&self) -> Self {
        let x = {
            if self.x != 0 {
                self.x / self.x.abs()
            } else {
                0
            }
        };
        let y = {
            if self.y != 0 {
                self.y / self.y.abs()
            } else {
                0
            }
        };
        PosDiff { x, y }
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new_sand() -> Self {
        Pos { x: 500, y: 0 }
    }

    fn add(&self, other: &PosDiff) -> Self {
        Pos {
            x: (self.x as isize + other.x) as usize,
            y: (self.y as isize + other.y) as usize,
        }
    }

    fn sub(&self, other: &Self) -> PosDiff {
        PosDiff {
            x: self.x as isize - other.x as isize,
            y: self.y as isize - other.y as isize,
        }
    }
}

struct Scan {
    depth: usize,
    content: HashMap<Pos, Tile>,
}

impl Scan {
    fn new() -> Self {
        Scan {
            depth: 1,
            content: HashMap::new(),
        }
    }

    fn add_rocks_lines(&mut self, rocks_lines: &[Pos]) {
        rocks_lines
            .iter()
            .skip(1)
            .enumerate()
            .for_each(|(previous_idx, rock)| {
                let previous_rock = rocks_lines.get(previous_idx).unwrap();
                self.add_rocks_line(previous_rock, rock);
            })
    }

    fn add_rocks_line(&mut self, from_pos: &Pos, to_pos: &Pos) {
        let mut pos = from_pos.clone();
        let vector = to_pos.sub(from_pos).to_normal();
        while &pos != to_pos {
            self.add_rock(&pos);
            pos = pos.add(&vector);
        }
        self.add_rock(&pos);
    }

    fn add_rock(&mut self, pos: &Pos) {
        self.content.insert(pos.clone(), Tile::Rock);
        self.depth = self.depth.max(pos.y + 1);
    }

    fn try_generate_sand(&mut self) -> bool {
        let mut pos = Pos::new_sand();
        loop {
            let new_pos = self.try_move_sand(&pos);
            match new_pos {
                Some(some_new_pos) => pos = some_new_pos,
                None => return self.try_add_sand(&pos),
            }
            if pos.y > self.depth {
                return false;
            }
        }
    }

    fn try_move_sand(&self, incoming_sand_pos: &Pos) -> Option<Pos> {
        let y = incoming_sand_pos.y + 1;
        let shift: [isize; 3] = [0, -1, 1];
        for delta_x in shift {
            if delta_x == -1 && incoming_sand_pos.x == 0 {
                continue;
            }
            let x = (incoming_sand_pos.x as isize + delta_x) as usize;
            let pos = Pos { x, y };
            if self.is_air(&pos) {
                return Some(pos);
            }
        }
        None
    }

    fn try_add_sand(&mut self, pos: &Pos) -> bool {
        let entry = self.content.entry(pos.clone());
        match &entry {
            Entry::Occupied(val) => match val.get() {
                Tile::Sand => false,
                Tile::Air => {
                    entry.insert_entry(Tile::Sand);
                    true
                }
                Tile::Rock => panic!("Cannot add a sand at a rock"),
            },
            Entry::Vacant(_) => {
                entry.insert_entry(Tile::Sand);
                true
            }
        }
    }

    fn is_air(&self, pos: &Pos) -> bool {
        self.get_tile_content(pos) == Tile::Air
    }

    fn get_tile_content(&self, pos: &Pos) -> Tile {
        match self.content.get(pos) {
            Some(tile) => tile.clone(),
            None => Tile::Air,
        }
    }
}

fn parse_line(line: &str) -> Vec<Pos> {
    let words = line.split(" -> ");
    words.map(parse_word).collect()
}

fn parse_word(word: &str) -> Pos {
    let mut words = word.split(',');
    let x = words.next().unwrap().parse().unwrap();
    let y = words.next().unwrap().parse().unwrap();
    Pos { x, y }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut scan = Scan::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            let rocks_lines = parse_line(&line);
            scan.add_rocks_lines(&rocks_lines);
        });

    while scan.try_generate_sand() {
        //
    }
    let result = scan
        .content
        .values()
        .filter(|tile| **tile == Tile::Sand)
        .count();
    println!("Result is `{:?}`", result);
}
