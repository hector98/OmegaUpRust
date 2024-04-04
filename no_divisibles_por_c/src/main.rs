// https://omegaup.com/arena/problem/No-divisibles-por-C/
use std::io;

fn main() {

    let mut t = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");
    let t: i64 = t.trim().parse().unwrap();
    let mut result = String::from("");

    for i in 0..t {
        let mut a_b_c = String::new();
        io::stdin()
            .read_line(&mut a_b_c)
            .expect("Failed to read line");
        let a_b_c: Vec<&str> = a_b_c.trim().split(' ').collect();

        let (a, b, c): (i64, i64, i64) = (a_b_c[0].parse().unwrap(), a_b_c[1].parse().unwrap(), a_b_c[2].parse().unwrap());

        let mut div: i64 = b - a;
        div = if div % 2 == 0 { div / 2 } else { div / 2 + 1 };
        println!("{} {}", div, c);
        let sum: i64 = (a + b) * (div -2);

        result = result + &sum.to_string();

        if i != t - 1 {
            result = result + "\n";
        }
    }

    println!("{}", result);
}
