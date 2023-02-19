use std::cell::RefCell;
use std::collections::{hash_map::Entry::Occupied, hash_map::Entry::Vacant, HashMap};

const ORIGIN: &str = "AA";
const AVAILABLE_TIME: usize = 30;

pub struct ValveDeclaration {
    pub name: String,
    pub rate: usize,
    pub connections: Vec<String>,
}

struct Valve {
    name: String,
    rate: usize,
    connections: HashMap<String, usize>,
}

pub struct Board {
    valves: HashMap<String, Valve>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            valves: HashMap::new(),
        }
    }

    pub fn add_valve(&mut self, declaration: ValveDeclaration) {
        let ValveDeclaration {
            name,
            rate,
            connections,
        } = declaration;
        let connections = connections.into_iter().map(|name| (name, 1)).collect();
        let valve = Valve {
            name,
            rate,
            connections,
        };
        self.valves.insert(valve.name.clone(), valve);
    }

    #[allow(clippy::needless_collect)]
    pub fn add_indirect_paths(&mut self) {
        let names: Vec<String> = self
            .valves
            .values()
            .filter(|valve| valve.rate > 0)
            .map(|valve| valve.name.clone())
            .collect();

        names.into_iter().for_each(|name| {
            let mut path = Path::new(self, name);
            path.map_paths();
            path.update_map_distances();
        });
    }

    pub fn update_distance(&mut self, from: &str, to: &str, distance: usize) {
        match self
            .valves
            .get_mut(from)
            .unwrap()
            .connections
            .entry(to.to_string())
        {
            Occupied(mut entry) => {
                *entry.get_mut() = distance;
            }
            Vacant(entry) => {
                entry.insert(distance);
            }
        };
        match self
            .valves
            .get_mut(to)
            .unwrap()
            .connections
            .entry(from.to_string())
        {
            Occupied(mut entry) => {
                *entry.get_mut() = distance;
            }
            Vacant(entry) => {
                entry.insert(distance);
            }
        };
    }

    pub fn remove_empty_valves(&mut self) {
        let names: Vec<String> = self
            .valves
            .values()
            .filter(|valve| valve.rate == 0 && valve.name != ORIGIN)
            .map(|valve| valve.name.clone())
            .collect();

        names.iter().for_each(|name| {
            self.remove_valve(name);
        });
    }

    fn remove_valve(&mut self, name: &str) {
        self.valves.remove(name);
        self.valves.values_mut().for_each(|valve| {
            valve.connections.remove(name);
        });
    }

    pub fn print(&self) {
        self.valves.values().for_each(|valve| {
            println!("valve {} rate={}. Connections: ", valve.name, valve.rate);
            print!("\t");
            valve.connections.iter().for_each(|(name, distance)| {
                print!("valve {}, distance={} ; ", name, distance);
            });
            println!();
        });
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

struct Path<'a> {
    board: &'a mut Board,
    origin: String,
    connections: RefCell<HashMap<String, usize>>,
}

impl<'a> Path<'a> {
    pub fn new(board: &'a mut Board, origin: String) -> Self {
        let mut connections = HashMap::new();
        connections.insert(origin.clone(), 0);
        Path {
            board,
            origin,
            connections: RefCell::new(HashMap::new()),
        }
    }

    pub fn map_paths(&mut self) {
        let origin = self.origin.clone();
        self.map_path_rec(&origin, 0);
    }

    fn map_path_rec(&self, valve_name: &str, distance: usize) {
        let connections = self
            .board
            .valves
            .get(valve_name)
            .unwrap()
            .connections
            .iter();

        connections.for_each(|(connection_name, connection_distance)| {
            let mut connections = self.connections.borrow_mut();
            match connections.entry(connection_name.clone()) {
                Occupied(mut entry) => {
                    let val = entry.get_mut();
                    if *val > distance + connection_distance {
                        *val = distance + connection_distance;
                        drop(connections);
                        self.map_path_rec(connection_name, distance + connection_distance);
                    }
                }
                Vacant(entry) => {
                    entry.insert(distance + connection_distance);
                    drop(connections);
                    self.map_path_rec(connection_name, distance + connection_distance);
                }
            }
        });
    }

    pub fn update_map_distances(&mut self) {
        self.connections
            .borrow()
            .iter()
            .filter(|(name, _)| **name != self.origin)
            .for_each(|(name, distance)| self.board.update_distance(&self.origin, name, *distance));
    }
}

pub fn play(board: &Board) -> Vec<GameState> {
    let mut final_states = Vec::new();

    let state = GameState::new_initial(board);
    play_rec(state, &mut final_states);

    final_states
}

fn play_rec<'a>(state: GameState<'a>, final_states: &mut Vec<GameState<'a>>) {
    let valve = state.board.valves.get(state.pos).unwrap();
    let remaining_time = state.get_remaining_time();

    let possible_valves = state.remaining_valves.iter().filter(|&&valve_name| {
        let action_time = valve.connections.get(valve_name).unwrap() + 1;
        action_time <= remaining_time
    });

    let can_open = possible_valves
        .map(|&valve_name| {
            let new_state = state.open_valve(valve_name);
            play_rec(new_state, final_states);
        })
        .next()
        .is_some();

    if !can_open {
        let new_state = state.wait();
        final_states.push(new_state);
    }
}

pub struct GameState<'a> {
    pub cumulated_pressure: usize,
    cumulated_rate: usize,
    minutes_elapsed: usize,
    pos: &'a str,
    remaining_valves: Vec<&'a str>,
    board: &'a Board,
}

impl<'a> GameState<'a> {
    pub fn new_initial(board: &'a Board) -> Self {
        let initial_valve = board.valves.get(ORIGIN).unwrap();
        let pos = initial_valve.name.as_ref();
        let cumulated_rate = 0;
        let cumulated_pressure = 0;
        let minutes_elapsed = 0;
        let remaining_valves = board
            .valves
            .values()
            .filter(|valve| valve.name != ORIGIN)
            .map(|valve| valve.name.as_ref())
            .collect();
        GameState {
            cumulated_pressure,
            cumulated_rate,
            minutes_elapsed,
            pos,
            remaining_valves,
            board,
        }
    }

    pub fn open_valve(&self, name: &'a str) -> Self {
        let action_time = self
            .board
            .valves
            .get(self.pos)
            .unwrap()
            .connections
            .get(name)
            .unwrap()
            + 1;
        if action_time > self.get_remaining_time() {
            panic!("Opening valve {} from position {} would take too much time. Elapsed minutes: {}, Time Cost: {}", name, self.pos, self.minutes_elapsed, action_time);
        }

        let valve = self.board.valves.get(name).unwrap();

        let cumulated_rate = self.cumulated_rate + valve.rate;
        let cumulated_pressure = self.cumulated_pressure + self.cumulated_rate * action_time;
        let minutes_elapsed = self.minutes_elapsed + action_time;
        let pos = name;
        let remaining_valves = self
            .remaining_valves
            .iter()
            .copied()
            .filter(|&valve| valve != name)
            .collect();

        let buf = vec![' '; minutes_elapsed];
        let buf: String = buf.iter().collect();
        println!("{}Open {}", buf, name);

        GameState {
            cumulated_pressure,
            cumulated_rate,
            minutes_elapsed,
            pos,
            remaining_valves,
            board: self.board,
        }
    }

    pub fn wait(&self) -> Self {
        let action_time = self.get_remaining_time();
        let cumulated_pressure = self.cumulated_pressure + self.cumulated_rate * action_time;
        let minutes_elapsed = self.minutes_elapsed + action_time;
        let remaining_valves = self.remaining_valves.clone();

        let buf = vec![' '; self.minutes_elapsed];
        let buf: String = buf.iter().collect();
        println!("{}Wait", buf);

        GameState {
            cumulated_pressure,
            cumulated_rate: self.cumulated_rate,
            minutes_elapsed,
            pos: self.pos,
            remaining_valves,
            board: self.board,
        }
    }

    pub fn get_remaining_time(&self) -> usize {
        AVAILABLE_TIME - self.minutes_elapsed
    }
}
