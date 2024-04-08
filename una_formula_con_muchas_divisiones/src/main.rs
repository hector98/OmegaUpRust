// https://omegaup.com/arena/problem/Una-formula-con-muchas-divisione/
use std::io;

fn main() {
    let mut x_y = String::new();
    io::stdin()
        .read_line(&mut x_y)
        .expect("Failed to read line");

    let (x, y) = x_y.trim().split_once(' ').unwrap();
    let x: f32 = x.trim().parse().expect("Not a number");
    let y: f32 = y.trim().parse().expect("Not a number");

    let z = ( ((x.powf(3.0) + x.powf(2.0)) / (y.powf(2.0) - y)) - ((x/y) + 5.0) ) / (2.0 * x);

    println!("{:.6}", z);
}

