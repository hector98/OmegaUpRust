// https://omegaup.com/arena/problem/Palindromos/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse().unwrap();
    let mut output = String::from("");
    
    for _ in 0..n {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        word = word.replace("\n", "").trim().to_string();

        if word == word.chars().rev().collect::<String>() {
            output += "P\n";
        } else {
            output += "NP\n";
        }
    }

    println!("{}", output);
}
