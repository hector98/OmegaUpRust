// https://omegaup.com/arena/problem/003_DevTequila/#problems
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: Vec<&str> = input.trim().split(' ').collect();

    let (a, b) = (input[0].parse::<i32>().unwrap(), input[1].parse::<i32>().unwrap());
    
    let sum = a + b;
    let sub = a - b;
    let div = a / b;
    let mul = a * b;

    println!("{sum}\n{sub}\n{div}\n{mul}");
}
