const MAX_IN_LIST: usize = 3;

pub struct CaloriesList {
    min_top_3: i32,
    list: Vec<i32>,
    current_elf_calories: i32,
}

impl CaloriesList {
    pub fn new() -> Self {
        CaloriesList {
            min_top_3: 0,
            list: Vec::with_capacity(4),
            current_elf_calories: 0,
        }
    }

    pub fn add_elf_item(&mut self, calories: i32) {
        self.current_elf_calories += calories;
    }

    pub fn get_max_top_elf_calories(&self) -> i32 {
        self.list.iter().fold(0, |acc, prev| acc + prev)
    }

    pub fn complete_elf(&mut self) {
        let current_elf_calories = self.current_elf_calories;
        self.current_elf_calories = 0;

        if self.list.len() < MAX_IN_LIST || current_elf_calories > self.min_top_3 {
            self.list.push(current_elf_calories);
            self.list.sort();
            self.list.reverse();
        }
        if self.list.len() > MAX_IN_LIST {
            self.list.pop();
        }
        self.min_top_3 = *self.list.last().unwrap();
    }
}
