// https://omegaup.com/arena/problem/Escalera-de-asteriscos/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u8 = n.trim().parse().expect("Not a number");

    let mut str_line = String::from("");
    let mut str_final = String::from("");

    for _ in 0..n {
        str_line += "#";
        str_final += format!("{}\n", str_line).as_str();
    }

    println!("{}", str_final);
}
