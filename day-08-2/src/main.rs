use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type TreeSize = u8;

struct Forest {
    rows: Vec<Vec<TreeSize>>,
}

#[derive(Clone)]
struct Position {
    column_idx: usize,
    row_idx: usize,
}

impl Forest {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }

    fn add_line(&mut self, line: &str) {
        let line: Vec<TreeSize> = line
            .chars()
            .map(|char| u8::try_from(char.to_digit(10).unwrap()).unwrap())
            .collect();
        if !self.rows.is_empty() {
            let expected_size = self.get_width();
            let actual_size = line.len();
            if actual_size != expected_size {
                panic!(
                    "The line should be {}, while it is {}",
                    expected_size, actual_size
                );
            }
        }
        self.rows.push(line);
    }

    fn get_height(&self) -> usize {
        self.rows.len()
    }

    fn get_width(&self) -> usize {
        self.rows.get(0).unwrap().len()
    }

    fn get_left_trees<'a>(&'a self, pos: &Position) -> Box<dyn Iterator<Item = &'a TreeSize> + 'a> {
        let vec = self.rows.get(pos.row_idx).unwrap();
        let slice = &vec[0..pos.column_idx];
        let iter = slice.iter();
        Box::new(iter)
    }

    fn get_right_trees<'a>(
        &'a self,
        pos: &Position,
    ) -> Box<dyn Iterator<Item = &'a TreeSize> + 'a> {
        let vec = self.rows.get(pos.row_idx).unwrap();
        let slice = &vec[pos.column_idx + 1..];
        let iter = slice.iter();
        Box::new(iter)
    }

    fn get_top_trees<'a>(&'a self, pos: &Position) -> Box<dyn Iterator<Item = &'a TreeSize> + 'a> {
        let column_idx = pos.column_idx;
        let slice = &self.rows[0..pos.row_idx];
        Box::new(slice.iter().map(move |row| row.get(column_idx).unwrap()))
    }

    fn get_bottom_trees<'a>(
        &'a self,
        pos: &Position,
    ) -> Box<dyn Iterator<Item = &'a TreeSize> + 'a> {
        let column_idx = pos.column_idx;
        let slice = &self.rows[pos.row_idx + 1..];
        Box::new(slice.iter().map(move |row| row.get(column_idx).unwrap()))
    }

    fn is_tree_visible(&self, pos: &Position) -> bool {
        if pos.row_idx == 0
            || pos.row_idx == self.get_height() - 1
            || pos.column_idx == 0
            || pos.column_idx == self.get_width() - 1
        {
            return true;
        }
        let tree_size = self.get_tree_size(pos);
        Forest::DIRECTIONS
            .iter()
            .any(|method| method(self, pos).all(|adjacent_size| *adjacent_size < tree_size))
    }

    fn get_tree_size(&self, pos: &Position) -> TreeSize {
        *self
            .rows
            .get(pos.row_idx)
            .unwrap()
            .get(pos.column_idx)
            .unwrap()
    }

    const DIRECTIONS: [for<'a> fn(
        &'a Forest,
        &Position,
    ) -> Box<dyn Iterator<Item = &'a TreeSize> + 'a>; 4] = [
        Forest::get_top_trees,
        Forest::get_bottom_trees,
        Forest::get_left_trees,
        Forest::get_right_trees,
    ];

    fn get_trees_iter(&self) -> PositionsIterator {
        PositionsIterator::new(self)
    }
}

struct PositionsIterator<'a> {
    forest: &'a Forest,
    pos: Position,
}

impl<'a> PositionsIterator<'a> {
    fn new(forest: &'a Forest) -> Self {
        let pos = Position {
            column_idx: 0,
            row_idx: 0,
        };
        Self { pos, forest }
    }
}

impl<'a> Iterator for PositionsIterator<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.column_idx >= self.forest.get_width()
            || self.pos.row_idx >= self.forest.get_height()
        {
            return None;
        }
        let item = self.pos.clone();

        self.pos.row_idx += 1;
        if self.pos.row_idx >= self.forest.get_width() {
            self.pos.row_idx = 0;
            self.pos.column_idx += 1;
        }

        Some(item)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut forest = Forest::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            forest.add_line(&line);
        });

    let result = forest
        .get_trees_iter()
        .filter(|pos| forest.is_tree_visible(pos))
        .count();
    println!("The result is `{}`", result);
}
