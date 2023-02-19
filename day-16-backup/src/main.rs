#![feature(entry_insert)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use day_16_1::{play, Board, ValveDeclaration};

fn parse_line(line: &str) -> ValveDeclaration {
    let mut words = line.split(' ');

    let name = words.nth(1).unwrap().to_string();
    let rate = words.nth(2).unwrap();
    let rate = rate
        .split('=')
        .nth(1)
        .unwrap()
        .trim_end_matches(';')
        .parse()
        .unwrap();

    let words = words.skip(4);

    let connections = words
        .map(|word| word.trim_end_matches(',').to_string())
        .collect();

    ValveDeclaration {
        name,
        rate,
        connections,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = "./input.txt".to_string();
    let path = args.get(1).unwrap_or(&default);
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut board = Board::new();
    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            let valve_declaration = parse_line(&line);
            board.add_valve(valve_declaration);
        });
    board.add_indirect_paths();
    board.remove_empty_valves();

    let final_states = play(&board);
    let result = final_states
        .iter()
        .map(|state| {
            println!("Candidate: {}", state.cumulated_pressure);
            state.cumulated_pressure
        })
        .max()
        .unwrap();
    println!("The resut is {}", result);
}
