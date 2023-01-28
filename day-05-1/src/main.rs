use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Command {
    CratesRow(CratesRow),
    Movement(Movement),
    Other,
}

impl Command {
    pub fn from_str(line: &str) -> Self {
        if line.contains("from") {
            return Command::Movement(Movement::from_str(line));
        }
        if line.contains('[') {
            return Command::CratesRow(CratesRow::from_str(line));
        }
        Command::Other
    }
}

struct CratesRow {
    pub list: Vec<Option<char>>,
}

impl CratesRow {
    pub fn from_str(line: &str) -> Self {
        let mut list = Vec::new();
        let nb_chars = line.len();
        let mut i = 1;
        while i < nb_chars {
            let c = line.chars().nth(i).unwrap();
            match c {
                'A'..='Z' => list.push(Some(c)),
                ' ' => list.push(None),
                _ => panic!("The character '{}' is unsupported for a crate", c),
            }
            i += 4;
        }

        CratesRow { list }
    }
}

struct Movement {
    pub from: usize,
    pub to: usize,
    pub quantity: usize,
}

impl Movement {
    pub fn from_str(line: &str) -> Self {
        let mut items = line.split(' ');
        let quantity: usize = items.nth(1).unwrap().parse().unwrap();
        let from = items.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to = items.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        Movement { from, to, quantity }
    }
}

struct CratesStacks {
    stacks_map: BTreeMap<usize, Vec<char>>,
    inserting: bool,
}

impl CratesStacks {
    pub fn new() -> Self {
        CratesStacks {
            stacks_map: BTreeMap::new(),
            inserting: true,
        }
    }

    pub fn add_crates_row(&mut self, crates_row: &CratesRow) {
        if !self.inserting {
            panic!("Cannot inserting crates after starting moving");
        }

        crates_row
            .list
            .iter()
            .enumerate()
            .filter_map(|(stack_idx, crate_opts)| {
                crate_opts
                    .as_ref()
                    .map(|crate_char| (stack_idx, crate_char))
            })
            .for_each(|(stack_idx, crate_char)| {
                let stack = self.stacks_map.entry(stack_idx).or_insert_with(Vec::new);
                stack.push(*crate_char);
            })
    }

    pub fn move_crates(&mut self, movement: &Movement) {
        self.mark_moving();
        let mut tmp_stack = Vec::new();

        let from_stack = self.stacks_map.get_mut(&movement.from).unwrap();
        for _ in 0..movement.quantity {
            let crate_item = from_stack.pop().unwrap();
            tmp_stack.push(crate_item);
        }

        tmp_stack.reverse();

        let to_stack = self.stacks_map.get_mut(&movement.to).unwrap();
        to_stack.extend_from_slice(&tmp_stack);
    }

    pub fn mark_moving(&mut self) {
        if self.inserting {
            self.stacks_map.iter_mut().for_each(|(_, stack)| {
                stack.reverse();
            });
            self.inserting = false;
        }
    }

    pub fn get_result(&self) -> String {
        self.stacks_map
            .values()
            .map(|crates_stack| crates_stack.last().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{CratesRow, CratesStacks, Movement};

    #[test]
    fn basic_setup() {
        // Given
        let mut crates_stacks = CratesStacks::new();

        // When
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('D'), None, None],
        });
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('E'), Some('F'), None],
        });
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('G'), Some('H'), Some('K')],
        });

        crates_stacks.mark_moving();

        // Then
        let result = crates_stacks.get_result();
        assert_eq!(&result, "DFK");
    }

    #[test]
    fn moving_stuff() {
        // Given
        let mut crates_stacks = CratesStacks::new();
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('D'), None, None],
        });
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('E'), Some('F'), None],
        });
        crates_stacks.add_crates_row(&CratesRow {
            list: vec![Some('G'), Some('H'), Some('K')],
        });

        // When
        crates_stacks.move_crates(&Movement { from: 0, to: 2, quantity: 2 });

        // Then
        let result = crates_stacks.get_result();
        assert_eq!(&result, "GFE");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut crates_stacks = CratesStacks::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            match Command::from_str(&line) {
                Command::CratesRow(crates_row) => crates_stacks.add_crates_row(&crates_row),
                Command::Movement(movement) => crates_stacks.move_crates(&movement),
                _ => (),
            };
        });
    println!("The score is {}", crates_stacks.get_result());
}
