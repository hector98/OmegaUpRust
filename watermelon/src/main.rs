// https://omegaup.com/arena/problem/Watermel/
use std::io;

fn main() {
    let mut w = String::new();

    io::stdin()
        .read_line(&mut w)
        .expect("Failed to read line");

    let w: i32 = w.trim().parse().unwrap();

    if w % 2 == 0 && w > 2 {
        println!("SI");
    } else {
        println!("NO");
    }
}
