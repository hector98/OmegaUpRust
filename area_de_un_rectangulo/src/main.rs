// https://omegaup.com/arena/problem/area_de_un_rectangulo/
use std::io;

fn main() {
    let mut a_b = String::new();
    io::stdin()
        .read_line(&mut a_b)
        .expect("Failed to read line");

    let (a, b) = a_b.trim().split_once(' ').unwrap();
    let a: u32 = a.trim().parse().expect("Not a number");
    let b: u32 = b.trim().parse().expect("Not a number");

    println!("{}", a * b);
}
