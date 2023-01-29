use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type RegisterUnit = isize;
const STARTING_VALUE: RegisterUnit = 1;
const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;
const SPRITE_WIDTH: usize = 3;
const PIXEL_OFF: char = ' ';
const PIXEL_ON: char = 'X';

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

struct Processor<'a> {
    instructions_list: Vec<Instruction>,
    current_instruction: Option<Instruction>,
    cycle_number: usize,
    current_instruction_cycle: usize,
    loading_instructions: bool,
    screen: &'a mut Screen,
}

impl<'a> Processor<'a> {
    fn new(screen: &'a mut Screen) -> Self {
        Processor {
            instructions_list: Vec::new(),
            current_instruction: None,
            cycle_number: 0,
            current_instruction_cycle: 0,
            loading_instructions: true,
            screen,
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
        self.screen.draw_pixel();
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
        self.screen.move_sprite(add_value);
    }
}

struct Screen {
    rows: Vec<Vec<char>>,
    rtc_x: usize,
    rtc_y: usize,
    sprite_middle_x: RegisterUnit,
}

impl Screen {
    fn new() -> Self {
        let row = vec![PIXEL_OFF; SCREEN_WIDTH];
        let mut rows = Vec::with_capacity(SCREEN_HEIGHT);
        rows.resize_with(SCREEN_HEIGHT, || row.clone());

        Screen {
            rows,
            rtc_x: 0,
            rtc_y: 0,
            sprite_middle_x: STARTING_VALUE,
        }
    }

    fn draw_pixel(&mut self) {
        if self.is_pixel_in_sprite() {
            self.rows[self.rtc_y][self.rtc_x] = PIXEL_ON;
        }
        self.move_rtc();
    }

    fn move_rtc(&mut self) {
        self.rtc_x += 1;
        if self.rtc_x >= SCREEN_WIDTH {
            self.rtc_x = 0;
            self.rtc_y += 1;
        }
    }

    fn is_pixel_in_sprite(&self) -> bool {
        let sprite_start = self.sprite_middle_x - SPRITE_WIDTH as RegisterUnit / 2;
        let rtc_x = self.rtc_x as RegisterUnit;
        rtc_x >= sprite_start && rtc_x < sprite_start + SPRITE_WIDTH as RegisterUnit
    }

    fn move_sprite(&mut self, delta_x: RegisterUnit) {
        self.sprite_middle_x += delta_x;
    }

    fn print(&self) {
        self.rows.iter().for_each(|row| {
            let s: String = row.iter().collect();
            println!("{}", s);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pixel_in_sprite() {
        let mut screen = Screen::new();

        screen.move_sprite(1);
        assert_eq!(screen.sprite_middle_x, 2, "sprite middle position");
        assert_eq!(screen.rtc_x, 0, "rtc_x position");

        assert_eq!(screen.is_pixel_in_sprite(), false, "pixel before sprite");
        screen.move_rtc();
        assert_eq!(
            screen.is_pixel_in_sprite(),
            true,
            "pixel at beginning of sprite"
        );
        screen.move_rtc();
        assert_eq!(
            screen.is_pixel_in_sprite(),
            true,
            "pixel at middle of sprite"
        );
        screen.move_rtc();
        assert_eq!(screen.is_pixel_in_sprite(), true, "pixel at end of sprite");
        screen.move_rtc();
        assert_eq!(screen.is_pixel_in_sprite(), false, "pixel after sprite");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut screen = Screen::new();
    let mut processor = Processor::new(&mut screen);

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

    println!("The result is:");
    screen.print();
}
