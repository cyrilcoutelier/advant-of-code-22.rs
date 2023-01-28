use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MARKER_SIZE: usize = 14;

fn predicate(line: &str, pos: usize) -> Option<usize> {
    if pos <= MARKER_SIZE {
        return None;
    }
    if line.len() <= MARKER_SIZE {
        return None;
    }
    let slice = &line[pos - MARKER_SIZE..pos];
    let mut char_set = HashSet::new();
    match slice.bytes().all(|c| char_set.insert(c)) {
        true => Some(pos),
        false => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.next().unwrap();
    let line = line.ok().unwrap();

    let result = line
        .chars()
        .enumerate()
        .find_map(|(pos, _)| predicate(&line, pos));

    match result {
        Some(pos) => println!("Marker position is: {}", pos),
        None => println!("Could not find a marker"),
    }
}
