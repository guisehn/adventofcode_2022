//! ```cargo
//! [dependencies]
//! regex = "1"
//! lazy_static = "1.4.0"
//! ```

// This challenge taught me that Rust makes it hard to work with tree structures.
// Decided to use a hash map approach instead.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

const ROOT: &str = "root";

#[derive(Debug)]
#[allow(dead_code)]
enum Node {
    File { name: String, size: i32 },
    Dir { name: String },
}

#[derive(Debug)]
struct FileSystem {
    space: i32,
    dirs: HashMap<String, Vec<Node>>,
}

impl FileSystem {
    fn new(space: i32) -> FileSystem {
        let mut dirs: HashMap<String, Vec<Node>> = HashMap::new();
        dirs.insert(String::from(ROOT), vec![]);
        FileSystem { dirs, space }
    }

    fn add_dir(&mut self, parent_path: &Vec<String>, name: &str) {
        let mut path = parent_path.clone();
        path.push(name.to_string());

        let path_key = FileSystem::path_to_key(&path);
        let parent_path_key = FileSystem::path_to_key(&parent_path);

        if self.dirs.contains_key(&path_key) {
            return;
        }

        self.dirs.insert(path_key, vec![]);
        self.dirs
            .get_mut(&parent_path_key)
            .unwrap()
            .push(Node::Dir {
                name: name.to_string(),
            });
    }

    fn add_file(&mut self, parent_path: &Vec<String>, file: Node) {
        let parent_path_key = FileSystem::path_to_key(&parent_path);
        self.dirs.get_mut(&parent_path_key).unwrap().push(file);
    }

    fn path_to_key(path: &Vec<String>) -> String {
        path.join("/")
    }

    fn get_dir_size(&self, path: &str) -> i32 {
        let mut total = 0;
        let children = self.dirs.get(path).unwrap();

        for node in children {
            let size = match node {
                Node::File { size, .. } => *size,
                Node::Dir { name, .. } => {
                    let sub_path = path.to_string() + "/" + name;
                    self.get_dir_size(&sub_path)
                }
            };

            total += size;
        }

        total
    }

    fn space_occupied(&self) -> i32 {
        self.get_dir_size(ROOT)
    }

    fn space_remaining(&self) -> i32 {
        self.space - self.space_occupied()
    }
}

fn read_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

fn parse_terminal_output(fs: &mut FileSystem, terminal: String) {
    let mut current_path: Vec<String> = vec![String::from(ROOT)];

    for line in terminal.trim().lines() {
        if line[0..4].eq("$ cd") {
            cd(fs, &mut current_path, &line);
        } else if line[0..4].eq("dir ") {
            let dir = &line[4..];
            fs.add_dir(&current_path, dir);
        } else {
            add_file(fs, &current_path, &line);
        }
    }
}

fn cd(fs: &mut FileSystem, current_path: &mut Vec<String>, command: &str) {
    // skip "$ cd "
    match &command[5..] {
        "/" => {
            current_path.drain(1..);
        }

        ".." => {
            if current_path.len() > 1 {
                current_path.pop();
            }
        }

        dir_name => {
            fs.add_dir(&current_path, dir_name);
            current_path.push(String::from(dir_name));
        }
    }
}

fn add_file(fs: &mut FileSystem, current_path: &Vec<String>, line: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    }

    match RE.captures(line) {
        Some(cap) => {
            let size: i32 = cap[1].parse().unwrap();
            let name = cap[2].to_string();
            fs.add_file(current_path, Node::File { name, size });
        }

        None => (),
    }
}

fn main() {
    let input = read_input();

    let mut fs = FileSystem::new(70000000);
    parse_terminal_output(&mut fs, input);

    let space_remaining = fs.space_remaining();

    let result: i32 = fs
        .dirs
        .iter()
        .map(|(path, _)| fs.get_dir_size(path))
        .filter(|size| (space_remaining + *size) > 30000000)
        .min()
        .unwrap();

    println!("{result}");
}
