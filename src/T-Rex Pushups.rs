use std::io;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.read_line(&mut buffer).unwrap();
    let spots: usize = buffer.trim().parse().unwrap();

    buffer.clear();
    stdin.read_line(&mut buffer).unwrap();
    let elevations: Vec<i64> = buffer
        .trim()
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut result = 0;
    for i in 1..spots {
        if elevations[i - 1] - elevations[i] >= 4 {
            result += 1;
        }
    }
    println!("{result}");
}
