use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let score: i32 = lines
        .filter(|line| {
            if let Err(e) = line {
                println!("Could not parse line: {}", e);
                return false;
            }
            true
        })
        .map(|line| {
            let line = line.unwrap();
            let (left_compartment, right_compartment) = split_line(&line);
            let left_compartment = str_to_set(left_compartment);
            let duplicate = find_duplicate(&left_compartment, right_compartment);
            get_item_priority(duplicate)
        })
        .sum();
    println!("The score is {}", score);
}

fn split_line(line: &str) -> (&str, &str) {
    let len = line.len();
    let half = len / 2;
    let first_slice = &line[0..half];
    let second_slice = &line[half..len];
    (first_slice, second_slice)
}

fn str_to_set(line: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    line.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

fn find_duplicate(left_compartment: &HashSet<char>, right_compartment: &str) -> char {
    let result = right_compartment
        .chars()
        .find(|c| left_compartment.contains(c));
    match result {
        Some(c) => c,
        None => panic!("No duplicate found in a bag"),
    }
}

fn get_item_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        _ => panic!("Item {} is invalid", item),
    }
}
