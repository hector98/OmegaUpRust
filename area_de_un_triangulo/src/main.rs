// https://omegaup.com/arena/problem/area_de_un_triangulo/
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let (a, b) = input.trim().split_once(' ').unwrap();
    let (a, b): (f32, f32) = (a.parse().unwrap(), b.parse().unwrap());

    println!("{:.2}", (a * b) / 2.0);
}
