use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn get_longest(predator: &String, eating: &HashMap<String, Vec<String>>, cache: &mut HashMap<String, i32>) -> i32 {
    if let Some(result) = cache.get(predator) {
        *result
    } else {
        let result = if let Some(prey_list) = eating.get(predator) {
            1 + prey_list
                .iter()
                .map(|prey| get_longest(prey, eating, cache))
                .max().unwrap_or(0)
        } else {
            1
        };
        cache.insert(predator.to_string(), result);
        result
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();

    let mut eating = HashMap::new();
    let mut eaten_by: HashMap<String, Vec<String>> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let (predator, prey) = line.split_once(" <- ").unwrap();
        let predator = predator.to_string();
        let prey: Vec<String> = prey.split(", ").map(|s| s.to_string()).collect();
        eaten_by.insert(predator.to_string(), vec![]);
        for dino in &prey {
            if let Some(eaten_by_list) = eaten_by.get_mut(dino) {
                eaten_by_list.push(predator.to_string());
            } else {
                eaten_by.insert(dino.to_string(), vec![predator.to_string()]);
            }
        }
        eating.insert(predator, prey);
    }

    let mut cache: HashMap<String, i32> = HashMap::new();
    let apex_predators: Vec<String> = eaten_by
        .iter()
        .filter(|(_, predators)| predators.len() == 0)
        .map(|(name, _)| name.to_string())
        .collect();
    println!("{}", apex_predators.iter().map(|p| get_longest(p, &eating, &mut cache)).max().unwrap());
}
