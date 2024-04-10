// https://omegaup.com/arena/problem/SumaLions/
use std::io;

fn main() {
    let mut a_b = String::new();
    io::stdin()
        .read_line(&mut a_b)
        .expect("Failed to read line");

    let (a, b) = a_b.trim().split_once(" ").unwrap();
    let a: i16 = a.trim().parse().expect("Not a number");
    let b: i16 = b.trim().parse().expect("Not a number");

    println!("{}", a + b);
}
