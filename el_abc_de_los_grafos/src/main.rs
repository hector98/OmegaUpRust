// https://omegaup.com/arena/problem/El-ABC-de-los-grafos/
use std::io;

fn main() {
    let mut n_a = String::new();
    io::stdin()
        .read_line(&mut n_a)
        .expect("Failed to read line");

    let (a, b) = n_a.trim().split_once(" ").unwrap();
    let mut output = String::from("");
    output += &format!("Hay {} nodos.\nHay {} aristas.", a.to_string(), b.to_string()).to_string();
    let a: u32 = a.trim().parse().expect("Not a number");

    for _ in 0..a {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read line");
        let (x, y, z) = (data[..1].to_string(), data[1..3].to_string(), data[3..5].to_string());

        output += &format!("\nEl nodo {} conecta con el nodo {} constando {}.", x, y, z).to_string();
    }

    println!("{}", output);
}
