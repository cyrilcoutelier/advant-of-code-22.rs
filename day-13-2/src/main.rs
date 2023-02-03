#![feature(map_first_last)]

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Value = usize;

type NodeResult = Result<(), bool>;

struct Pair {
    left: Vec<Node>,
    right: Vec<Node>,
}

impl Pair {
    fn is_right_order(&self) -> bool {
        match compare_list(&self.left, &self.right) {
            Ok(()) => panic!("No decision had be made"),
            Err(result) => result,
        }
    }
}

struct PairsFactory {
    pairs: Vec<Pair>,
    tmp_left: Option<Vec<Node>>,
}

impl PairsFactory {
    fn new() -> Self {
        PairsFactory {
            pairs: Vec::new(),
            tmp_left: None,
        }
    }

    fn add_list(&mut self, list: Vec<Node>) {
        match self.tmp_left.take() {
            Some(left_list) => self.pairs.push(Pair {
                left: left_list,
                right: list,
            }),
            None => self.tmp_left = Some(list),
        }
    }

    fn get_pairs(self) -> Vec<Pair> {
        assert!(
            self.tmp_left.is_none(),
            "There should not be en orphan list remaining"
        );
        self.pairs
    }
}

enum Node {
    Value(Value),
    List(Vec<Node>),
}

fn compare_list(left: &[Node], right: &[Node]) -> NodeResult {
    let mut right_iter = right.iter();
    for left_item in left.iter() {
        match right_iter.next() {
            None => return Err(false),
            Some(right_item) => compare_nodes(left_item, right_item)?,
        }
    }

    match right.len() > left.len() {
        true => Err(true),
        false => Ok(()),
    }
}

fn wrap_value(value: Value) -> Vec<Node> {
    vec![Node::Value(value)]
}

fn compare_nodes(left_node: &Node, right_node: &Node) -> NodeResult {
    match left_node {
        Node::Value(left_value) => match right_node {
            Node::Value(right_value) => compare_values(*left_value, *right_value)?,
            Node::List(right_list) => compare_list(&wrap_value(*left_value), right_list)?,
        },
        Node::List(left_list) => match right_node {
            Node::List(right_list) => compare_list(left_list, right_list)?,
            Node::Value(right_value) => compare_list(left_list, &wrap_value(*right_value))?,
        },
    }

    Ok(())
}

fn compare_values(left_value: Value, right_value: Value) -> NodeResult {
    if left_value < right_value {
        return Err(true);
    }
    if left_value > right_value {
        return Err(false);
    }
    Ok(())
}

struct LineParser {
    lists_stack: Vec<Vec<Node>>,
    current_list: Option<Vec<Node>>,
    current_value: Option<Value>,
}

impl LineParser {
    fn new() -> Self {
        LineParser {
            lists_stack: Vec::new(),
            current_list: None,
            current_value: None,
        }
    }

    fn get_result(&mut self) -> Vec<Node> {
        assert_eq!(self.lists_stack.len(), 0, "The list_stack should be empty");
        assert!(
            self.current_value.is_none(),
            "There should not be ongoing value"
        );
        self.current_list.take().unwrap()
    }

    fn parse_char(&mut self, c: char) {
        match c {
            '[' => self.start_list(),
            ']' => self.stop_list(),
            ',' => self.try_finish_number(),
            '0'..='9' => self.add_digit(c),
            _ => panic!("Invalid char in line: `{}`", c),
        }
    }

    fn start_list(&mut self) {
        if let Some(current_list) = self.current_list.take() {
            self.lists_stack.push(current_list);
        }
        self.current_list = Some(Vec::new());
    }

    fn stop_list(&mut self) {
        self.try_finish_number();

        if let Some(mut parent_list) = self.lists_stack.pop() {
            let child_list = self.current_list.take().unwrap();
            parent_list.push(Node::List(child_list));
            self.current_list = Some(parent_list);
        }
    }

    fn try_finish_number(&mut self) {
        if self.current_value.is_some() {
            self.finish_number();
        }
    }

    fn finish_number(&mut self) {
        let value = self.current_value.take().unwrap();
        let current_list = self.current_list.as_mut().unwrap();
        current_list.push(Node::Value(value));
    }

    fn add_digit(&mut self, digit: char) {
        let digit = digit as Value - '0' as Value;
        match self.current_value.as_mut() {
            Some(current_value) => {
                *current_value *= 10;
                *current_value += digit;
            }
            None => self.current_value = Some(digit),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut pairs_factory = PairsFactory::new();
    let mut line_parser = LineParser::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            if line != "" {
                line.chars().for_each(|c| line_parser.parse_char(c));
                let list = line_parser.get_result();
                pairs_factory.add_list(list);
            }
        });

    let pairs = pairs_factory.get_pairs();
    let result: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.is_right_order())
        .map(|(idx, _)| idx + 1)
        .sum();
    println!("Result is `{:?}`", result);
}
