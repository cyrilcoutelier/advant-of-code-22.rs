use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::{Rc, Weak};

const UPDATE_REQUIRED_SPACE: usize = 30000000;
const DISK_SIZE: usize = 70000000;

enum DirectoryMove {
    Out,
    In(String),
    Root,
}

impl DirectoryMove {
    fn from_str(word: &str) -> Self {
        match word {
            ".." => DirectoryMove::Out,
            "/" => DirectoryMove::Root,
            "" => panic!("Invalid destination for `cd`: `{}`", word),
            _ => DirectoryMove::In(word.to_string()),
        }
    }
}

enum Command {
    CD(DirectoryMove),
    LS,
}

impl Command {
    fn from_str(line_str: &str) -> Self {
        let mut words = line_str.split(' ');
        words.next().unwrap();
        let word = words.next().unwrap();
        if word == "ls" {
            return Command::LS;
        }
        if word != "cd" {
            panic!("Invalid command: `{}`", line_str);
        }
        let word = words.next().expect("Missing destination of `cd` command");
        Command::CD(DirectoryMove::from_str(word))
    }
}

struct FileContent {
    name: String,
    size: usize,
}

enum Entry {
    DIRECTORY(String),
    FILE(FileContent),
}

impl Entry {
    fn from_str(line_str: &str) -> Self {
        let mut words = line_str.split(' ');
        let first_word = words.next().unwrap();
        let second_word = words.next().unwrap();
        match first_word {
            "dir" => Entry::DIRECTORY(second_word.to_string()),
            _ => {
                let name = second_word.to_string();
                let size = first_word.parse().unwrap();
                Entry::FILE(FileContent { name, size })
            }
        }
    }
}

enum Line {
    COMMAND(Command),
    ENTRY(Entry),
}

impl Line {
    fn from_str(line_str: &str) -> Self {
        match line_str.starts_with('$') {
            true => Line::COMMAND(Command::from_str(line_str)),
            false => Line::ENTRY(Entry::from_str(line_str)),
        }
    }
}

struct Directory {
    name: String,
    parent: Option<Weak<RefCell<Directory>>>,
    children: HashMap<String, DirectoryEntry>,
    total_size: Cell<Option<usize>>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        let parent = parent.map(|p| Rc::downgrade(&p));
        Directory {
            name,
            parent,
            children: HashMap::new(),
            total_size: Cell::new(None),
        }
    }

    fn get_size(&self) -> usize {
        match self.total_size.get() {
            Some(total_size) => total_size,
            None => {
                let total_size = self.compute_size();
                self.total_size.replace(Some(total_size));
                total_size
            }
        }
    }

    fn compute_size(&self) -> usize {
        self.children
            .values()
            .map(|child| match child {
                DirectoryEntry::FILE(file) => file.size,
                DirectoryEntry::DIRECTORY(directory) => directory.borrow().get_size(),
            })
            .sum()
    }
}

enum DirectoryEntry {
    FILE(FileContent),
    DIRECTORY(Rc<RefCell<Directory>>),
}

struct FileSystem {
    root: Rc<RefCell<Directory>>,
    cd: Rc<RefCell<Directory>>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory::new("/".to_string(), None)));
        let cd = root.clone();
        FileSystem { root, cd }
    }

    fn process_line(&mut self, line: Line) {
        match line {
            Line::ENTRY(entry) => self.add_entry(entry),
            Line::COMMAND(command) => self.process_command(&command),
        }
    }

    fn add_entry(&mut self, entry: Entry) {
        match entry {
            Entry::FILE(file) => self.add_file(file),
            Entry::DIRECTORY(name) => self.add_directory(name),
        }
    }

    fn add_file(&mut self, file: FileContent) {
        let name = file.name.clone();
        self.check_duplicate(&name);
        let mut cd = self.cd.borrow_mut();
        let entry = DirectoryEntry::FILE(file);
        cd.children.insert(name, entry);
    }

    fn add_directory(&mut self, name: String) {
        self.check_duplicate(&name);
        let mut cd = self.cd.borrow_mut();
        let directory = Directory::new(name.clone(), Some(self.cd.clone()));
        let entry = DirectoryEntry::DIRECTORY(Rc::new(RefCell::new(directory)));
        cd.children.insert(name, entry);
    }

    fn check_duplicate(&self, name: &str) {
        let cd = self.cd.borrow();
        if cd.children.contains_key(name) {
            panic!(
                "The current directory `{}` has already an entry with name `{}`",
                cd.name, name
            );
        }
    }

    fn process_command(&mut self, command: &Command) {
        match command {
            Command::LS => (),
            Command::CD(directory_target) => self.process_cd(directory_target),
        }
    }

    fn process_cd(&mut self, directory_target: &DirectoryMove) {
        match directory_target {
            DirectoryMove::In(name) => self.cd_in(name),
            DirectoryMove::Out => self.cd_out(),
            DirectoryMove::Root => self.cd_root(),
        }
    }

    fn cd_in(&mut self, name: &str) {
        let new_cd = {
            let cd = self.cd.borrow();
            let child = cd.children.get(name).unwrap_or_else(|| {
                panic!("The directory {} do not have a child {}", cd.name, name)
            });
            match child {
                DirectoryEntry::FILE(_) => {
                    panic!("cannot change directory to {} as it is a file", name)
                }
                DirectoryEntry::DIRECTORY(directory) => directory.clone(),
            }
        };

        self.cd = new_cd;
    }

    fn cd_root(&mut self) {
        self.cd = self.root.clone();
    }

    fn cd_out(&mut self) {
        let new_cd = {
            let previous_cd = self.cd.borrow();
            previous_cd
                .parent
                .as_ref()
                .expect("Cannot go up when already at the root")
                .upgrade()
                .unwrap()
        };

        self.cd = new_cd;
    }

    fn print(&self) {
        FileSystem::print_directory(&self.root.borrow(), 0);
    }

    fn print_directory(directory: &Directory, depth: usize) {
        let mut precursor = "  ".repeat(depth);
        println!("{}- {} (dir)", precursor, directory.name);
        precursor.push_str("  ");
        directory.children.values().for_each(|child| match child {
            DirectoryEntry::FILE(file) => {
                println!("{}- {} (file, size={})", precursor, file.name, file.size)
            }
            DirectoryEntry::DIRECTORY(sub_directory) => {
                FileSystem::print_directory(&sub_directory.borrow(), depth + 1)
            }
        })
    }

    fn directories(&self) -> DirectoryIterator {
        DirectoryIterator::from_root(self.root.clone())
    }

    fn get_smallest_fitting_directory(&self) -> Rc<RefCell<Directory>> {
        let file_system_size = self.root.borrow().get_size();
        println!("file_system_size: `{}`", file_system_size);
        let available_space = DISK_SIZE - file_system_size;
        println!("available_space: `{}`", available_space);
        let missing_space = UPDATE_REQUIRED_SPACE - available_space;
        println!("missing_space: `{}`", missing_space);

        self.directories()
            .filter(|directory| directory.borrow().get_size() >= missing_space)
            .reduce(|previous_directory, directory| {
                let previous_size = previous_directory.borrow().get_size();
                let new_size = directory.borrow().get_size();
                match previous_size < new_size {
                    true => previous_directory,
                    false => directory,
                }
            })
            .unwrap()
    }
}

struct DirectoryIterator {
    file: Vec<Rc<RefCell<Directory>>>,
}

impl DirectoryIterator {
    fn from_root(root: Rc<RefCell<Directory>>) -> Self {
        let file = vec![root];
        DirectoryIterator { file }
    }
}

impl Iterator for DirectoryIterator {
    type Item = Rc<RefCell<Directory>>;

    fn next(&mut self) -> Option<Self::Item> {
        let directory_opts = self.file.pop();
        match directory_opts {
            None => None,
            Some(directory) => {
                directory
                    .borrow()
                    .children
                    .values()
                    .filter_map(|child| match child {
                        DirectoryEntry::FILE(_) => None,
                        DirectoryEntry::DIRECTORY(child_directory) => Some(child_directory.clone()),
                    })
                    .for_each(|child_directory| self.file.push(child_directory));
                Some(directory)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut file_system = FileSystem::new();

    lines
        .filter_map(|line| match line {
            Ok(line_str) => Some(line_str),
            Err(e) => {
                println!("Could not parse line: {}", e);
                None
            }
        })
        .for_each(|line| {
            let line = Line::from_str(&line);
            file_system.process_line(line);
        });

    file_system.print();

    let result = file_system.get_smallest_fitting_directory();
    println!(
        "The result is `{}, with a size of {}`",
        result.borrow().name,
        result.borrow().get_size()
    );
}
