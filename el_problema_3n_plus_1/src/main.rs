// https://omegaup.com/arena/problem/El-problema-3n1/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let mut n: i16 = n.trim().parse::<i16>().unwrap();
    let mut max: i16 = n;
    let mut steps: i16 = 0;

    loop {
        if n == 1 {
            break;
        } else {
            steps += 1;
        }

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (n * 3) + 1;
        }

        if n > max {
             max = n;
        }

    }

    println!("{} {}", steps, max);
}
