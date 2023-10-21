use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let mut scores = HashMap::new();
    for line in stdin.lock().lines() {
        for name in line.unwrap().split(",") {
            let value = scores.get(name).unwrap_or(&0);
            scores.insert(name.to_string(), value + 1);
        }
    }
    if let Some((name, votes)) = scores.iter().max_by_key(|(_, v)| *v) {
        println!("{name}");
        println!("{votes}");
    }
}
