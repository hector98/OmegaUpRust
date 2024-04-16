// https://omegaup.com/arena/problem/Rotar-arreglo-derecha-una-vez/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: usize = n.trim().parse().unwrap();
    
    let mut arreglo = String::new();
    io::stdin()
        .read_line(&mut arreglo)
        .expect("Failed to read line");
    let arreglo = arreglo.trim().split_whitespace().collect::<Vec<&str>>();

    //println!("{:?}", arreglo);
    //println!("{}", &arreglo[0..3].join(" "));
    println!("{} {}", arreglo[n - 1..].join(" "), arreglo[0..n - 1].join(" "));
}
