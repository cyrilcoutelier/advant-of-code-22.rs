use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let re = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();

    let score = lines
        .filter(|line| {
            if let Err(e) = line {
                println!("Could not parse line: {}", e);
                return false;
            }
            true
        })
        .filter(|line| {
            let line = line.as_ref().unwrap();
            let cap = re.captures(line).unwrap();

            let first_start = cap.get(1).unwrap().as_str();
            let first_start: i32 = first_start.parse().unwrap();

            let first_stop = cap.get(2).unwrap();
            let first_stop: i32 = first_stop.as_str().parse().unwrap();

            let second_start = cap.get(3).unwrap();
            let second_start: i32 = second_start.as_str().parse().unwrap();

            let second_stop = cap.get(4).unwrap();
            let second_stop: i32 = second_stop.as_str().parse().unwrap();

            (first_start <= second_start && first_stop >= second_start) ||
            (second_start <= first_start && second_stop >= first_start)
        })
        .count();
    println!("The score is {}", score);
}
