// https://omegaup.com/arena/problem/Minimo-y-Maximo-en-un-Intervalo/
use std::io;

fn main() {
    let mut n_m = String::new();
    io::stdin()
        .read_line(&mut n_m)
        .expect("Failed to read line");
    let n_m: Vec<i32> = n_m
        .split(" ")
        .map( |x| x.trim().parse().expect("Not a number"))
        .collect();

    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    let mut s: Vec<i32> = s
        .split(" ")
        .map( |x| x.trim().parse().expect("Not a number"))
        .collect();

    let mut results = String::from("");

    for _ in 0..n_m[1] {
        let mut interval = String::new();
        io::stdin()
            .read_line(&mut interval)
            .expect("Failed to read line");

        let interval: Vec<&str> = interval
            .trim()
            .split(" ")
            .collect();

        let (mut a, b): (i32, i32) = (interval[1].parse().unwrap(), interval[2].parse().unwrap());
        a -= 1;

        if interval[0] == "C" {
            let mut min = s[0];
            let mut max = s[0];
            //b -= 1;

            while a < b {
                if s[a as usize] < min {
                    min = s[a as usize];
                }

                if s[a as usize] > max {
                    max = s[a as usize];
                }

                a += 1;
            }
            results += &format!("{} {}\n", min, max);
        } else {
            s[a as usize] = b;
        }
    }

    println!("{}", results);
}
