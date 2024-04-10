// https://omegaup.com/arena/problem/comi-Fraccion-mixta/
use std::io;

fn main() {
    let mut m_n = String::new();
    io::stdin()
        .read_line(&mut m_n)
        .expect("Failed to read line");
    let (m, n) = m_n.trim().split_once(" ").unwrap();
    let n: u64 = n.trim().parse().expect("Not a number");
    let m: u64 = m.trim().parse().expect("Not a number");

    if m % n != 0 {
        println!("{} {}/{}", m/n, m%n, n);
    } else {
        println!("{}", m/n);
    }
}
