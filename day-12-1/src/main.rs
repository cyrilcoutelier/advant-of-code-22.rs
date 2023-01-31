#![feature(map_first_last)]

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_ADJACENTS: usize = 4;
type Height = u16;
type Id = usize;
type Distance = usize;

struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn get_id(&self, width: usize) -> Id {
        self.x + self.y * width
    }
}

struct PathFinding<'a> {
    graph: &'a Graph,
    initial_pos: Id,
    mapping: HashMap<Id, Distance>,
}

impl<'a> PathFinding<'a> {
    fn new(graph: &'a Graph, initial_pos: Id) -> Self {
        let mapping = HashMap::new();
        // mapping.insert(initial_pos, 0);
        PathFinding {
            graph,
            initial_pos,
            mapping,
        }
    }

    fn find_paths(&mut self) {
        self.find_paths_rec(self.initial_pos, 0);
    }

    fn find_paths_rec(&mut self, explored_id: Id, distance: Distance) {
        let current_distance = self.mapping.entry(explored_id).or_insert(distance);
        if *current_distance > distance {
            *current_distance = distance;
        }
        self.graph
            .nodes
            .get(explored_id)
            .unwrap()
            .adjacents
            .iter()
            .for_each(|adjacent_id| match self.mapping.get(adjacent_id) {
                Some(adjacent_distance) => {
                    if *adjacent_distance > distance + 1 {
                        self.find_paths_rec(*adjacent_id, distance + 1);
                    }
                }
                None => self.find_paths_rec(*adjacent_id, distance + 1),
            });
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn from_map(map: &Map) -> Self {
        let width = map.get_width();
        let height = map.get_height();
        let mut nodes = Vec::with_capacity(width * height);

        map.rows.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, _)| {
                let pos = Pos { x, y };

                let adjacent_positions = map.get_accessible_tiles_from_pos(&pos);
                let adjacents = adjacent_positions
                    .iter()
                    .map(|adjacent_pos| adjacent_pos.get_id(width))
                    .collect();

                nodes.push(Node { adjacents });
            });
        });
        Graph { nodes }
    }
}

struct Node {
    adjacents: Vec<Id>,
}

fn can_go(from: Height, to: Height) -> bool {
    from + 1 >= to
}

struct Map {
    start_pos: Pos,
    end_pos: Pos,
    rows: Vec<Vec<Height>>,
}

impl Map {
    fn get_accessible_tiles_from_pos(&self, pos: &Pos) -> Vec<Pos> {
        let vec = self.get_adjacent_tiles(pos);
        let current_height = self.get_tile_height(pos);
        vec.into_iter()
            .filter(|adjacent_pos| {
                let adjacent_height = self.get_tile_height(adjacent_pos);
                can_go(current_height, adjacent_height)
            })
            .collect()
    }

    fn get_adjacent_tiles(&self, pos: &Pos) -> Vec<Pos> {
        let mut vec = Vec::with_capacity(MAX_ADJACENTS);
        if pos.x > 0 {
            vec.push(Pos {
                x: pos.x - 1,
                y: pos.y,
            });
        }
        let width = self.get_width();
        if pos.x + 1 < width {
            vec.push(Pos {
                x: pos.x + 1,
                y: pos.y,
            });
        }

        if pos.y > 0 {
            vec.push(Pos {
                x: pos.x,
                y: pos.y - 1,
            });
        }
        let height = self.get_height();
        if pos.y + 1 < height {
            vec.push(Pos {
                x: pos.x,
                y: pos.y + 1,
            });
        }
        vec
    }

    fn get_width(&self) -> usize {
        self.rows.first().unwrap().len()
    }

    fn get_height(&self) -> usize {
        self.rows.len()
    }

    fn get_tile_height(&self, pos: &Pos) -> Height {
        *self.rows.get(pos.y).unwrap().get(pos.x).unwrap()
    }
}

struct MapFactory {
    start_pos: Option<Pos>,
    end_pos: Option<Pos>,
    rows: Vec<Vec<Height>>,
    width: Option<usize>,
}

fn char_to_height(c: char) -> Height {
    c as Height - 'a' as Height
}

impl MapFactory {
    fn new() -> Self {
        MapFactory {
            start_pos: None,
            end_pos: None,
            rows: Vec::new(),
            width: None,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let row: Vec<Height> = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                let letter = match c {
                    'S' => {
                        if self.start_pos.is_some() {
                            panic!("There is already a start position");
                        }
                        let y = self.rows.len();
                        self.start_pos = Some(Pos { x, y });
                        'a'
                    }
                    'E' => {
                        if self.end_pos.is_some() {
                            panic!("There is already a end position");
                        }
                        let y = self.rows.len();
                        self.end_pos = Some(Pos { x, y });
                        'z'
                    }
                    _ => c,
                };
                char_to_height(letter)
            })
            .collect();
        let width = row.len();
        match self.width {
            Some(current_width) => assert_eq!(
                width, current_width,
                "The provided line width {} is different than the map width {}",
                width, current_width
            ),
            None => self.width = Some(width),
        }
        self.rows.push(row);
    }

    fn create_map(self) -> Map {
        let MapFactory {
            start_pos,
            end_pos,
            rows,
            width: _width,
        } = self;
        let start_pos = start_pos.unwrap();
        let end_pos = end_pos.unwrap();
        Map {
            start_pos,
            end_pos,
            rows,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut map_factory = MapFactory::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            map_factory.parse_line(&line);
        });

    let map = map_factory.create_map();
    println!("Create graph");
    let graph = Graph::from_map(&map);

    let width = map.get_width();
    let start_id = map.start_pos.get_id(width);
    let end_id = map.end_pos.get_id(width);

    println!("Find Paths");
    let mut path_finding = PathFinding::new(&graph, start_id);
    path_finding.find_paths();
    let result = path_finding.mapping.get(&end_id).unwrap();
    println!("Result is `{}`", result);
}
