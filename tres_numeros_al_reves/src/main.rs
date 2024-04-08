// https://omegaup.com/arena/problem/Tres-Numeros-Al-Reves/
use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

    println!("{}{}{}", c, b, a);
}
