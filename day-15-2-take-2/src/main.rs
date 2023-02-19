#![feature(entry_insert)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use day_14_1::{Couple, Pos};

const MAX_SIZE: isize = 4000000;

fn parse_line(line: &str) -> Couple {
    let mut words = line.split(' ');

    let sensor_x = words.nth(2).unwrap();
    let sensor_x = sensor_x.trim_start_matches("x=");
    let sensor_x = sensor_x.trim_end_matches(',');
    let sensor_x = sensor_x.parse().unwrap();

    let sensor_y = words.next().unwrap();
    let sensor_y = sensor_y.trim_start_matches("y=");
    let sensor_y = sensor_y.trim_end_matches(':');
    let sensor_y = sensor_y.parse().unwrap();

    let sensor = Pos {
        x: sensor_x,
        y: sensor_y,
    };

    let beacon_x = words.nth(4).unwrap();
    let beacon_x = beacon_x.trim_start_matches("x=");
    let beacon_x = beacon_x.trim_end_matches(',');
    let beacon_x = beacon_x.parse().unwrap();

    let beacon_y = words.next().unwrap();
    let beacon_y = beacon_y.trim_start_matches("y=");
    let beacon_y = beacon_y.parse().unwrap();

    let beacon = Pos {
        x: beacon_x,
        y: beacon_y,
    };

    let distance = sensor.get_distance(&beacon);

    Couple {
        sensor,
        distance,
        beacon,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let couples: Vec<Couple> = lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .map(|line| parse_line(&line))
        .collect();

    let result = couples
        .iter()
        .find_map(|couple| {
            couple
                .sensor
                .get_circle_iter(couple.distance + 1)
                .filter(|pos| pos.x >= 0 && pos.x <= MAX_SIZE && pos.y >= 0 && pos.y <= MAX_SIZE)
                .find(|pos| {
                    couples
                        .iter()
                        .filter(|searched_couple| searched_couple.sensor != couple.sensor)
                        .all(|searched_couple| {
                            searched_couple.sensor.get_distance(pos) > searched_couple.distance
                        })
                })
        })
        .unwrap();

    println!("Result is `{}`", result.y + result.x * MAX_SIZE);
}
