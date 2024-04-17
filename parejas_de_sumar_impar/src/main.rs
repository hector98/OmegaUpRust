// https://omegaup.com/arena/problem/Parejas-de-sumar-impar/
use std::io;
use std::convert::TryInto;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Please type a number!");

    let mut array = String::new();
    io::stdin()
        .read_line(&mut array)
        .expect("Failed to read line");
    let array: Vec<&str> = array.split_whitespace().collect();

    let mut i: usize = 0;
    let mut pares: u32 = 0;

    if n >= 1000 {
        let mut j: usize = (n / 2).try_into().unwrap();
        let mut k: usize = if n % 2 == 0 { j } else { (n / 2 + 1).try_into().unwrap() };
        let mut l: usize = (n - 1).try_into().unwrap();
        loop {
            let (a, b, c, d): (u32, u32, u32, u32) = (
                array[i].parse().unwrap(),
                array[j].parse().unwrap(),
                array[k].parse().unwrap(),
                array[l].parse().unwrap(),
                );

            if a % 2 == 0 {
                pares += 1;
            }

            if j == k {
                if b % 2 == 0 {
                    pares += 1;
                }
            } else {
                if b % 2 == 0 {
                    pares += 1;
                }

                if c % 2 == 0 {
                    pares += 1;
                }
            }

            if d % 2 == 0 {
                pares += 1;
            }

            i += 1;
            j -= 1;
            k += 1;
            l -= 1;

            if i == j || k == l || j == 0 {
                break;
            }

        }
    } else {
        loop {
            let num: u32 = array[i].parse().unwrap();
            if num % 2 == 0 {
                pares += 1;
            }

            if i == (n - 1).try_into().unwrap() {
                break;
            }

            i += 1;
        }
    }

    let sumas: u32 = (n - pares) * pares;
    println!("{}", sumas);
}
