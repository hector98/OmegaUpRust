// https://omegaup.com/arena/problem/Contando-Divisores/
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: Vec<&str> = input.trim().split_whitespace().collect();
    let (k, mut a, b): (i32, i32, i32) = (input[0].parse().unwrap(), input[1].parse().unwrap(), input[2].parse().unwrap());

    let mut count: i32 = 0;
    if a % k != 0 {
        a = a + (k - (a % k));
    }
    
    loop {
        a += k;
        count += 1;

        if a > b {
            break;
        }
    }

    println!("{}", count);
}
