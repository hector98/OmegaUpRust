// https://omegaup.com/arena/problem/SSS/
use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u128 = n.trim().parse().expect("Not a number");

    let mut sum: u128 = 0;
    let mut sum_total: u128 = 0;

    for i in 1..=n {
        let aux_sum: u128 = i * (i + 1) / 2;

        sum_total += aux_sum + sum;
        sum += aux_sum;
    }

    println!("{}", sum_total);
}
