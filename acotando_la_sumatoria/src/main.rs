// https://omegaup.com/arena/problem/Acotando-la-sumatoria/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u16 = n.trim().parse().unwrap();
    let mut k = 1;
    let mut sum = 0;

    loop {
        sum += k + 7;
        if sum >= n {
            break;
        }
        k += 1;
    }

    println!("{}", k);
}
