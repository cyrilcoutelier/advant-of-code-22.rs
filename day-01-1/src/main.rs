use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use day010::calories_list::CaloriesList;

fn main() {
  let args: Vec<String> = env::args().collect();
  let path = &args[1];
  let path = Path::new(path);
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines();
  let mut calories_list = CaloriesList::new();

  lines.for_each(|line| {
    match line {
      Ok(item_calories) => {
        match item_calories.parse() {
          Ok(item_calories) => calories_list.add_elf_item(item_calories),
          Err(_) => calories_list.complete_elf(),
        };  
      },
      Err(e) => println!("Could not parse line: {}", e),
    }
  });
  calories_list.complete_elf();
  println!("Elf carying the max calories result is {}", calories_list.get_max_elf_calories());
}
