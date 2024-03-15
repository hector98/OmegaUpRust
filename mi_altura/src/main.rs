use std::io;

fn main() {
    let mut altura = String::new();
    io::stdin().read_line(&mut altura).expect("Failed to read line");
    let altura: f32 = altura.trim().parse().expect("Failed to parse input");

    println!("Tu altura es de {:.2} metros! WOW\n", altura); 
}
