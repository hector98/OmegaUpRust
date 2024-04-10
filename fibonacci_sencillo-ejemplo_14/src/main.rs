// https://omegaup.com/arena/problem/Fibonacci-Sencillo---Ejemplo-14/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u8 = n.trim().parse().unwrap();
    let mut num_actual: u32 = 0;
    let mut num_anterior: u32 = 1;

    for _ in 1..=n {
        let aux: u32 = num_actual;
        num_actual += num_anterior;
        num_anterior = aux;
    }

    println!("{}", num_actual);
}
