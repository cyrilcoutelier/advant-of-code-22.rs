

pub struct CaloriesList {
  pub max_elf_calories: i32,
  pub current_elf_calories: i32,
}

impl CaloriesList {
  pub fn new() -> Self {
    CaloriesList { max_elf_calories: 0, current_elf_calories: 0 }
  }

  pub fn add_elf_item(&mut self, calories: i32) {
    self.current_elf_calories += calories;
  }

  pub fn get_max_elf_calories(&self) -> i32 {
    self.max_elf_calories
  }

  pub fn complete_elf(&mut self) {
    if self.current_elf_calories > self.max_elf_calories {
      self.max_elf_calories = self.current_elf_calories;
    }
    self.current_elf_calories = 0;
  }
}
