use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().to_string();
    let mut pieces = buffer.split(" ");
    let target: usize = pieces.next().unwrap().parse().unwrap();
    let g_1: f64 = pieces.next().unwrap().parse().unwrap();
    let p_0: f64 = pieces.next().unwrap().parse().unwrap();
    let p_1: f64 = pieces.next().unwrap().parse().unwrap();
    let p_2: f64 = pieces.next().unwrap().parse().unwrap();

    let mut g = g_1;
    for _ in 0..target {
        g = (1.0 - g) * (1.0 - g) * p_0 + 2.0 * g * (1.0 - g) * p_1 + g * g * p_2;
    }
    println!("{g}");
}
