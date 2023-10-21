use std::{
    collections::{HashMap, HashSet},
    io,
};

fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().to_string();
    //eprintln!("Line: {:?}", buffer);
    buffer
}

fn main() {
    // First line, number of species
    let num_species: usize = read_line().parse().unwrap();

    let species: HashMap<String, HashSet<String>> = (0..num_species)
        .map(|_| {
            let name = read_line();
            // Second line of species, number of attributes
            let num_attributes: usize = read_line().parse().unwrap();

            let attributes: HashSet<String> = (0..num_attributes).map(|_| read_line()).collect();
            (name, attributes)
        })
        .collect();

    let num_fossils: usize = read_line().parse().unwrap();
    for _ in 0..num_fossils {
        // Second line of fossil, number of attributes
        let num_attributes: usize = read_line().parse().unwrap();
        let attributes: HashSet<String> = (0..num_attributes).map(|_| read_line()).collect();
        let mut best_score = 0.0;
        let mut best_score_name = None;
        for (s_name, s_attributes) in &species {
            let mut m = 0; // Matching attributes
            let mut n = 0; // Non-matching attributes
            for f_attribute in &attributes {
                if s_attributes.contains(f_attribute) {
                    m += 1;
                } else {
                    n += 1;
                }
            }
            let score = 100.0 * (m - n) as f64 / s_attributes.len() as f64;
            if score == 100.0 {
                best_score_name = Some(s_name);
                break;
            } else if score >= 50.0 && score > best_score {
                best_score = score;
                best_score_name = Some(s_name);
            }
        }
        match best_score_name {
            None => println!("Possible New Discovery"),
            Some(name) => println!("{name}"),
        }
    }
}
