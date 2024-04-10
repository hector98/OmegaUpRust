// https://omegaup.com/arena/problem/par_o_impar/
use std::io;

fn main() {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number: u16 = number.trim().parse().expect("Not a number");

    if number % 2 == 0 {
        println!("par");
    } else {
        println!("impar");
    }
}
