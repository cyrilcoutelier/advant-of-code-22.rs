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
    let mut compartments = Vec::with_capacity(3);

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
            let comparment = str_to_set(&line);
            compartments.push(comparment);
            if (compartments.len() < 3) {
                return 0;
            }
            let badge = find_badge(&mut compartments);
            compartments.clear();
            get_item_priority(badge)
        })
        .sum();
    println!("The score is {}", score);
}

fn str_to_set(line: &str) -> HashSet<char> {
    let mut set = HashSet::new();
    line.chars().for_each(|c| {
        set.insert(c);
    });
    set
}

fn get_item_priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        _ => panic!("Item {} is invalid", item),
    }
}

fn find_badge(compartments: &mut Vec<HashSet<char>>) -> char {
    while compartments.len() > 1 {
        let first = compartments.pop().unwrap();
        let badges_candidates: Vec<char> = first
            .into_iter()
            .filter(|candidate_badge| {
                compartments
                    .iter()
                    .all(|compartment| compartment.contains(candidate_badge))
            })
            .collect();
        if badges_candidates.len() == 1 {
            return badges_candidates[0];
        } else if badges_candidates.len() == 0 {
            panic!("a bag has no common with other bags");
        }
    }
    panic!("could not find badge");
}
