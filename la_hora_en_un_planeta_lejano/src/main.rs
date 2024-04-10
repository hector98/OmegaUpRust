// https://omegaup.com/arena/problem/La-hora-en-un-planeta-lejado/
use std::io;

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let mut s: u32 = s.trim().parse().unwrap();

    let days: u32 = s / 42000;
    s %= 42000;
    let hours: u32 = s / 3500;
    s %= 3500;
    let minutes: u32 = s / 50;
    s %= 50;

    println!("{} {} {} {}", days, hours, minutes, s);
}
