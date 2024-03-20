// https://omegaup.com/arena/problem/sacos-de-macetas/
use std::io;

fn main() {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let mut num: i32 = num.trim().parse().unwrap();
    let mut result: i32;

    if num < 0 {
        num *= -1;
    }

    if num <= 3 {
        result = 1;
    }else if num < 7 {
        result = if num - 3 > 0 { 2 } else { 1 };
    }else if num == 7 {
        result = 1;
    }
    else {
        result = num / 7;
        if num % 7 > 3 {
            result += 2;
        }else{
            result += 1;
        }
    }

    println!("{}", result);
}
