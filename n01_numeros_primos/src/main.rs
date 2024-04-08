// https://omegaup.com/arena/problem/M01-Numeros-Primos/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Not a number");

    let mut i: u32 = 2;
    let mut m_i: u32 = (n + 1) / 2;
    let mut m_f: u32 = if n % 2 == 0 { m_i } else { m_i + 1 };
    let mut f: u32 = n - 1;
    let mut result = String::from("Si");

    loop {
        if i == m_i || f == m_f {
            break;
        }

        if n % i == 0 || n % m_i == 0 || n % m_f == 0 || n % f == 0 {
            result = String::from("No");
            break;
        }

        i += 1;
        m_i -= 1;
        m_f += 1;
        f -= 1;
    }

    println!("{}", result);
}
