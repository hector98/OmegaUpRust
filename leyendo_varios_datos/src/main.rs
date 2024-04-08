// https://omegaup.com/arena/problem/CR-Leyendo-Varios-datos/#problems
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i16 = n.trim().parse().expect("Not a number");
    let mut result = String::from("");

    for _ in 0..n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        let (a, b) = line.trim().split_once(' ').unwrap();
        let sum = a.parse::<i16>().unwrap() + b.parse::<i16>().unwrap();

        result += format!("{}\n", sum).as_str();
    }

    println!("{}", result);
}
