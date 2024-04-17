// https://omegaup.com/arena/problem/Dibujando-una-piramide/
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.split_whitespace().collect::<Vec<&str>>();
    let n = input[0].parse::<u32>().unwrap();
    let letter = input[1];

    let mut str_space = vec![" ".to_string(); n as usize];
    let mut str_line = String::from("");
    let mut str_final = String::from("");

    for i in 0..n {
        str_space.pop();

        str_line = str_space.join("") +  str_line.trim();
        if i != 0 {
            str_line += format!(" {}", letter.trim()).as_str();
        } else {
            str_line += format!("{}", letter.trim()).as_str();
        }

        str_final += format!("{}\n", str_line).as_str();

    }

    println!("{}", str_final);
}
