// https://omegaup.com/arena/problem/COMI-Triples/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u16 = n.trim().parse().unwrap();
    let mut count = 0;
    let mut result = String::from("");

    for i in 1..=n {
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num: u32 = num.trim().parse().unwrap();

        if num % 3 == 0 || num == 0 {
            count += 1;
            result += &format!("{} ", i);
        }
    }

    if count == 0 {
        println!("No hay triples.");
    } else {
        println!("{}\n{}", count, result);
    }
}
