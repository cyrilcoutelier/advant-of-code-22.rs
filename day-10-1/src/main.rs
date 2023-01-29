use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type RegisterUnit = i32;
const STARTING_VALUE: RegisterUnit = 1;
const FIRST_INTERESTING_CYCLE: i32 = 20;
const INTERESTING_CYCLE_DISTANCE: i32 = 40;

struct Instruction {
    nb_cycles: usize,
    payload: Payload,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        let payload = Payload::from_str(s);
        let nb_cycles = payload.get_nb_cycles();
        Instruction { nb_cycles, payload }
    }
}

enum Payload {
    Noop,
    AddX(RegisterUnit),
}

impl Payload {
    fn from_str(s: &str) -> Self {
        let mut words = s.split(' ');
        let instruction_word = words.next().unwrap();
        match instruction_word {
            "noop" => Payload::Noop,
            "addx" => {
                let payload_word = words.next().unwrap();
                let payload_number: RegisterUnit = payload_word.parse().unwrap();
                Payload::AddX(payload_number)
            }
            _ => panic!("Invalid instruction word: {}", instruction_word),
        }
    }

    fn get_nb_cycles(&self) -> usize {
        match self {
            Payload::Noop => 1,
            Payload::AddX(_) => 2,
        }
    }
}

struct Processor {
    instructions_list: Vec<Instruction>,
    current_instruction: Option<Instruction>,
    cycle_number: usize,
    register_x: RegisterUnit,
    current_instruction_cycle: usize,
    loading_instructions: bool,
    interesting_signals: Vec<RegisterUnit>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            instructions_list: Vec::new(),
            current_instruction: None,
            cycle_number: 0,
            register_x: STARTING_VALUE,
            current_instruction_cycle: 0,
            loading_instructions: true,
            interesting_signals: Vec::new(),
        }
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        if !self.loading_instructions {
            panic!("Cannot load more instructions when processor is executing");
        }
        self.instructions_list.push(instruction);
    }

    fn do_cycle(&mut self) {
        if self.is_finished() {
            return;
        }
        self.stop_loading_if_needed();
        self.start_cycle();
        self.count_cycle();
        if is_cycle_interesting(self.cycle_number) {
            let signal_strength = self.get_signal_strength();
            self.interesting_signals.push(signal_strength);
        }
        self.end_cycle();
    }

    fn is_finished(&self) -> bool {
        self.instructions_list.is_empty() && self.current_instruction.is_none()
    }

    fn stop_loading_if_needed(&mut self) {
        if !self.loading_instructions {
            return;
        }
        self.instructions_list.reverse();
        self.loading_instructions = false;
    }

    fn start_cycle(&mut self) {
        if self.current_instruction.is_none() {
            let instruction = self.instructions_list.pop().unwrap();
            self.current_instruction_cycle = instruction.nb_cycles;
            self.current_instruction = Some(instruction);
        }
    }

    fn count_cycle(&mut self) {
        self.cycle_number += 1;
        self.current_instruction_cycle -= 1;
    }

    fn end_cycle(&mut self) {
        if self.current_instruction_cycle == 0 {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        let instruction = self.current_instruction.as_ref().unwrap();
        match instruction.payload {
            Payload::Noop => (),
            Payload::AddX(add_value) => self.perform_add_x(add_value),
        }
        self.current_instruction = None;
    }

    fn perform_add_x(&mut self, add_value: RegisterUnit) {
        self.register_x += add_value;
    }

    fn get_signal_strength(&self) -> RegisterUnit {
        self.register_x * self.cycle_number as RegisterUnit
    }
}

fn is_cycle_interesting(cycle_number: usize) -> bool {
    let cycle_number = cycle_number as i32 - FIRST_INTERESTING_CYCLE;
    if cycle_number < 0 {
        return false;
    }
    cycle_number % INTERESTING_CYCLE_DISTANCE == 0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut processor = Processor::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            let instruction = Instruction::from_str(&line);
            processor.add_instruction(instruction);
        });

    while !processor.is_finished() {
        processor.do_cycle();
    }

    let result: RegisterUnit = processor.interesting_signals.iter().sum();
    println!("The result is `{}`", result);
}
