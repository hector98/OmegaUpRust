// https://omegaup.com/arena/problem/Empresa-de-Seguridad/

use std::io;

fn main() {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: Vec<&str> = num.trim().split("").collect();

    println!("{}{}{}{}", num[4], num[3], num[2], num[1]);
}
