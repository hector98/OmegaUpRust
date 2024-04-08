// https://omegaup.com/arena/problem/Serie-17141815/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i8 = n.trim().parse().expect("Not a number");

    if n >= 0 {
        let mut sum = false;
        let mut result = String::from("");
        let mut num: u8 = 17;
        for _ in 0..n {
            result += format!("{} ", num).as_str();

            if sum {
                num += 4;
                sum = !sum;
            } else
            {
                num -= 3;
                sum = !sum;
            }
        }
        println!("{}", result);
    } else {
        println!("El número debe ser positivo");
    }


}
