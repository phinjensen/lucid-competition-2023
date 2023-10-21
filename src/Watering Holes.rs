use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut all_holes = HashSet::new();
    let mut first = true;
    let mut dinosaurs = HashMap::new();
    for line in stdin.lock().lines() {
        if first {
            first = false;
            continue;
        }
        let line = line.unwrap();
        let (name, hole) = line.split_once(' ').unwrap();
        let hole: usize = hole.parse().unwrap();
        if hole > 0 {
            all_holes.insert(hole);
        }
        dinosaurs.insert(name.to_string(), hole);
    }
    let mut holes: HashMap<usize, Vec<String>> = HashMap::new();
    for (name, hole) in dinosaurs {
        if let Some(names) = holes.get_mut(&hole) {
            names.push(name);
        } else {
            holes.insert(hole, vec![name]);
        }
    }
    let mut keys = all_holes.iter().collect::<Vec<_>>();
    keys.sort();
    for hole in keys {
        print!("{hole} ");
        let names = holes.get_mut(hole);
        if let Some(names) = names {
            names.sort();
            println!("{}", names.join(" "));
        } else {
            println!("n/a");
        }
    }
}
