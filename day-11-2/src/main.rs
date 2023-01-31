#![feature(map_first_last)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Split;

type WorryLevel = u32;
type MonkeyId = usize;

const NB_ROUNDS: usize = 10000;
const NB_ACTIVE_MONKEYS: usize = 2;

struct Item {
    initial_level: WorryLevel,
    congruences_map: BTreeMap<WorryLevel, WorryLevel>,
}

impl Item {
    fn new(initial_level: WorryLevel) -> Self {
        let congruences_map = BTreeMap::new();
        Item {
            initial_level,
            congruences_map,
        }
    }

    fn fill_congruences_map(&mut self, congruences_list: &[WorryLevel]) {
        congruences_list.iter().for_each(|test_value| {
            self.congruences_map
                .entry(*test_value)
                .or_insert_with(|| self.initial_level % test_value);
        })
    }

    // IF a ≡ b (n) THEN a+c ≡ b+c (n)
    fn add(&mut self, right: Option<WorryLevel>) {
        let right = right.unwrap();
        self.congruences_map
            .iter_mut()
            .for_each(|(test_value, current_value)| {
                *current_value = (*current_value + right) % test_value;
            })
    }

    // IF a ≡ b (n) THEN ac ≡ bc (n)
    // IF a ≡ b (n) THEN a^2 ≡ b^2 (n)
    fn multiply(&mut self, right: Option<WorryLevel>) {
        self.congruences_map
            .iter_mut()
            .for_each(|(test_value, current_value)| {
                let local_right = match right {
                    Some(value) => value,
                    None => *current_value,
                };

                *current_value = (*current_value * local_right) % test_value;
            })
    }
}

struct Operation {
    function: fn(&mut Item, Option<WorryLevel>),
    right: Option<WorryLevel>,
}

impl Operation {
    fn manipulate(&self, item: &mut Item) {
        (self.function)(item, self.right);
    }
}

struct Flyingitem {
    target_monkey_id: MonkeyId,
    item: Item,
}

struct Monkey {
    id: MonkeyId,
    operation: Operation,
    test_value: WorryLevel,
    target_true: MonkeyId,
    target_false: MonkeyId,
    items: VecDeque<Item>,
    manipulated_item: Option<Item>,
    manipulations_count: usize,
}

impl Monkey {
    fn new(
        id: MonkeyId,
        operation: Operation,
        test_value: WorryLevel,
        target_true: MonkeyId,
        target_false: MonkeyId,
        items: VecDeque<Item>,
    ) -> Self {
        Monkey {
            id,
            operation,
            test_value,
            target_true,
            target_false,
            items,
            manipulated_item: None,
            manipulations_count: 0,
        }
    }

    fn has_items_left(&self) -> bool {
        !self.items.is_empty()
    }

    fn take_item(&mut self) {
        let item = self.items.pop_front();
        self.manipulated_item = item;
    }

    fn manipulate_item(&mut self) {
        let item = self.manipulated_item.as_mut().unwrap();
        self.operation.manipulate(item);
        self.manipulations_count += 1;
    }

    fn throw_item(&mut self) -> Flyingitem {
        let target_monkey_id = self.get_target_monkey_id();
        let item = self.manipulated_item.take().unwrap();
        Flyingitem {
            target_monkey_id,
            item,
        }
    }

    fn get_target_monkey_id(&self) -> MonkeyId {
        let item = self.manipulated_item.as_ref().unwrap();
        let congruence = item.congruences_map.get(&self.test_value).unwrap();

        match *congruence == 0 {
            true => self.target_true,
            false => self.target_false,
        }
    }

    fn receive_item(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

struct MonkeysCrew {
    monkeys: Vec<RefCell<Monkey>>,
}

impl MonkeysCrew {
    fn new() -> Self {
        MonkeysCrew {
            monkeys: Vec::new(),
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(RefCell::new(monkey));
    }

    fn fill_monkeys_congruences(&self) {
        let list: Vec<WorryLevel> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.borrow().test_value)
            .collect();
        self.monkeys.iter().for_each(|monkey| {
            monkey
                .borrow_mut()
                .items
                .iter_mut()
                .for_each(|item| item.fill_congruences_map(&list))
        });
    }

    fn do_round(&self) {
        self.monkeys.iter().for_each(|monkey_ref| {
            while monkey_ref.borrow().has_items_left() {
                let Flyingitem {
                    item,
                    target_monkey_id,
                } = {
                    let mut monkey = monkey_ref.borrow_mut();
                    monkey.take_item();
                    monkey.manipulate_item();
                    monkey.throw_item()
                };
                let receiver_monkey = self.monkeys.get(target_monkey_id).unwrap();
                receiver_monkey.borrow_mut().receive_item(item);
            }
        });
    }
}

struct MonkeyFactory {
    id: Option<MonkeyId>,
    operation: Option<Operation>,
    test_value: Option<WorryLevel>,
    target_true: Option<MonkeyId>,
    target_false: Option<MonkeyId>,
    items: Option<VecDeque<Item>>,
    ready_count: usize,
}

impl MonkeyFactory {
    fn new() -> Self {
        MonkeyFactory {
            id: None,
            operation: None,
            test_value: None,
            target_true: None,
            target_false: None,
            items: None,
            ready_count: 6,
        }
    }

    fn parse_line(&mut self, line: &str) {
        let mut words = line.trim().split(' ');
        let first_word = words.next().unwrap();
        match first_word {
            "Monkey" => self.parse_id(&mut words),
            "Starting" => self.parse_items(&mut words),
            "Operation:" => self.parse_operation(&mut words),
            "Test:" => self.parse_test(&mut words),
            "If" => self.parse_throw(&mut words),
            _ => (),
        }
    }

    fn parse_id(&mut self, words: &mut Split<char>) {
        if self.id.is_some() {
            panic!("Trying to set an id where there is already one");
        }
        self.ready_count -= 1;

        let word = words.next().unwrap();
        let mut words = word.split(':');
        let word = words.next().unwrap();

        let id = word.parse().unwrap();
        self.id = Some(id);
    }

    fn parse_items(&mut self, words: &mut Split<char>) {
        if self.items.is_some() {
            panic!("Trying to set items where there is already one");
        }
        self.ready_count -= 1;

        words.next();
        let items = words
            .map(|word| {
                let mut words = word.split(',');
                let word = words.next().unwrap();
                let initial_level = word.parse::<WorryLevel>().unwrap();
                Item::new(initial_level)
            })
            .collect();
        self.items = Some(items);
    }

    fn parse_operation(&mut self, words: &mut Split<char>) {
        if self.operation.is_some() {
            panic!("Trying to set an operation where there is already one");
        }
        self.ready_count -= 1;

        let symbol = words.nth(3).unwrap();
        let value = words.next().unwrap();

        let value = match value {
            "old" => None,
            _ => Some(value.parse().unwrap()),
        };
        let operation = match symbol {
            "*" => Operation {
                function: Item::multiply,
                right: value,
            },
            "+" => Operation {
                function: Item::add,
                right: value,
            },
            _ => panic!("Invalid operator {}", symbol),
        };

        self.operation = Some(operation);
    }

    fn parse_test(&mut self, words: &mut Split<char>) {
        if self.test_value.is_some() {
            panic!("Trying to set a test value where there is already one");
        }
        self.ready_count -= 1;

        let word = words.nth(2).unwrap();
        let test_value = word.parse().unwrap();
        self.test_value = Some(test_value);
    }

    fn parse_throw(&mut self, words: &mut Split<char>) {
        let word = words.next().unwrap();
        match word {
            "true:" => self.parse_true(words),
            "false:" => self.parse_false(words),
            _ => panic!("Invalid value for If: {}", word),
        }
    }

    fn parse_true(&mut self, words: &mut Split<char>) {
        if self.target_true.is_some() {
            panic!("Trying to set a target_true value where there is already one");
        }
        self.ready_count -= 1;

        let word = words.nth(3).unwrap();
        let target_true = word.parse().unwrap();
        self.target_true = Some(target_true);
    }

    fn parse_false(&mut self, words: &mut Split<char>) {
        if self.target_false.is_some() {
            panic!("Trying to set a target_false value where there is already one");
        }
        self.ready_count -= 1;

        let word = words.nth(3).unwrap();
        let target_false = word.parse().unwrap();
        self.target_false = Some(target_false);
    }

    fn is_monkey_ready(&self) -> bool {
        self.ready_count == 0
    }

    fn get_monkey(&mut self) -> Monkey {
        let operation = self.operation.take().unwrap();
        let items = self.items.take().unwrap();

        let id = self.id.unwrap();
        let test_value = self.test_value.unwrap();
        let target_true = self.target_true.unwrap();
        let target_false = self.target_false.unwrap();
        self.id = None;
        self.test_value = None;
        self.target_true = None;
        self.target_false = None;

        self.ready_count = 6;

        Monkey::new(id, operation, test_value, target_true, target_false, items)
    }
}

struct TopMonkey {
    set: BTreeSet<usize>,
}

impl TopMonkey {
    fn new() -> Self {
        TopMonkey {
            set: BTreeSet::new(),
        }
    }

    fn add_monkey(&mut self, manipulations_count: usize) {
        self.set.insert(manipulations_count);
        while self.set.len() > NB_ACTIVE_MONKEYS {
            self.set.pop_first();
        }
    }

    fn get_monkey_business(&self) -> usize {
        self.set.iter().product()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut monkey_factory = MonkeyFactory::new();
    let mut monkeys_crew = MonkeysCrew::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            monkey_factory.parse_line(&line);
            if monkey_factory.is_monkey_ready() {
                let monkey = monkey_factory.get_monkey();
                monkeys_crew.add_monkey(monkey);
            }
        });

    monkeys_crew.fill_monkeys_congruences();

    for _ in 0..NB_ROUNDS {
        monkeys_crew.do_round();
    }

    let mut top_monkeys = TopMonkey::new();
    monkeys_crew
        .monkeys
        .iter()
        .for_each(|monkey| top_monkeys.add_monkey(monkey.borrow().manipulations_count));
    let result = top_monkeys.get_monkey_business();
    println!("The result is `{}`", result);
}
