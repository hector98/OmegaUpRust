// https://omegaup.com/arena/problem/Concatenacion-de-dos-numeros/
use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");
    a = a.replace("\n", "");

    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");
    b = b.replace("\n", "");

    println!("{}{}", a.to_string(), b.to_string());
}
