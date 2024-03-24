// https://omegaup.com/arena/problem/Buscando-cadenas-con-poca-memori/
use std::io;

fn main() {
    let mut j: u32;
    let mut k: u32;
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: u32 = n.trim().parse().expect("Not a number");

    let mut words: Vec<String> = Vec::new();

    for _ in 0..n {
        let mut w = String::new();
        io::stdin()
            .read_line(&mut w)
            .expect("Failed to read line");
        w = w.replace("\n", "");
        words.push((&w.trim()).to_string());
    }

    let mut result = String::from("");
    let mut r: u8 = 0;
    let mut m = String::new();
    io::stdin()
        .read_line(&mut m)
        .expect("Failed to read line");
    let m: u32 = m.trim().parse().expect("Not a number");
    let mut search: Vec<String> = Vec::new();

    for _ in 0..m {
        let mut word = String::new();
        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");
        word = word.replace("\n", "");
        search.push((&word.trim()).to_string());

        let mut i: u32 = 0;
        if n % 2 == 0 {
            j = (n / 2) - 1;
            k = n / 2;
        }
        else {
            j = n / 2;
            k = n / 2;
        }
        let mut l: u32 = n - 1;

        loop {
            if word == words[i as usize] || word == words[j as usize] || word == words[k as usize] || word == words[l as usize] {
                r = 1;
                break;
            }
            
            if i == j || k == l || j == 0 || k == n-1 {
                break;
            }

            i += 1;
            j -= 1;
            k += 1;
            l -= 1;
        }
        result.push_str(&r.to_string());
        result.push_str("\n");
        r = 0;
    }
    
    println!("{}", result);
}
