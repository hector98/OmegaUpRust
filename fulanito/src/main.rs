// https://omegaup.com/arena/problem/Fulanito/
use std::io;

fn main() {
    let mut q_n = String::new();
    io::stdin()
        .read_line(&mut q_n)
        .expect("Failed to read line");
    let q_n: Vec<u32> = q_n
        .split_whitespace()
        .map(|x| x.trim().parse().expect("Not a number"))
        .collect();

    let mut autos = String::new();
    io::stdin()
        .read_line(&mut autos)
        .expect("Failed to read line");
    
    let autos: Vec<u32> = autos
        .split_whitespace()
        .map(|x| x.trim().parse().expect("Not a number"))
        .collect();

    let mut resultados = String::from("");

    for _ in 0..q_n[0] {
        let mut gas = String::new();
        io::stdin()
            .read_line(&mut gas)
            .expect("Failed to read line");
        let mut gas: u32 = gas.trim().parse().expect("Not a number");
        let mut r: u8 = 0;

        for i in &autos {
            if *i <= gas {
                r += 1;
                gas -= i;
            }else {
                break;
            }
        }
        resultados.push_str(&r.to_string());
        resultados.push(' ');
        resultados.push_str(&gas.to_string());
        resultados.push('\n');
    }

    println!("{}", resultados);

}
