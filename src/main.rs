use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let mut ancestors: HashMap<String, String> = HashMap::new();
    let mut targets = Vec::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some((ancestor, child)) = line.split_once(" -- ") {
            ancestors.insert(child.to_string(), ancestor.to_string());
        } else {
            targets.push(line);
        }
    }

    let mut parents_0: HashSet<String> = HashSet::new();
    let mut parents_1: HashSet<String> = HashSet::new();
    loop {
        if parents_1.contains(&targets[0]) {
            println!("{}", targets[0]);
            break;
        } else if parents_0.contains(&targets[1]) {
            println!("{}", targets[1]);
            break;
        }
        if let Some(parent) = ancestors.get(&targets[0]) {
            targets[0] = parent.to_string();
        }
        if let Some(parent) = ancestors.get(&targets[1]) {
            targets[1] = parent.to_string();
        }
        parents_0.insert(targets[0].to_string());
        parents_1.insert(targets[1].to_string());
    }
}
