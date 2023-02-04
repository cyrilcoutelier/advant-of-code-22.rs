#![feature(entry_insert)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use day_14_1::{get_intersection_disk_row, Couple, Pos, Segments};

const SEARCHED_ROW: isize = 2000000;

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

    let mut segments = Segments::new();
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

    couples
        .iter()
        .filter_map(|couple| {
            get_intersection_disk_row(&couple.sensor, couple.distance, SEARCHED_ROW)
        })
        .for_each(|segment| {
            segments.add_segment(segment);
        });

    couples.iter().for_each(|couple| {
        if couple.beacon.y == SEARCHED_ROW {
            segments.remove_dot(couple.beacon.x);
        }
        if couple.sensor.y == SEARCHED_ROW {
            segments.remove_dot(couple.sensor.x);
        }
    });

    let result: usize = segments.map.values().sum();

    println!("Result is `{}`", result);
}
