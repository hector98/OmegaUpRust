use std::io;

fn main() {
    let mut n = String::new();
    let mut bin = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let mut n: i32 = n.trim().parse().expect("Failed to parse input");
    let mut i = 2;

    let mut aux = String::new();

    loop {
        let mut i_aux: i32 = i;
        let mut j = 0;
        loop {
            if j < n && i_aux != 0 {
                loop {
                    aux += "{i_aux%2}";
                    i_aux /= 2;
                    j += 1;
                    if i_aux == 0 {
                        break;
                    }
                }
            }else{
                aux += "0";
                j += 1;
                if j == n {
                    break;
                }
            }
        }
        i += 1;
        if i < 2**n {
            bin += "{aux}\n";
        } else {
            break;
        }
    }

    println!("{}", bin);
}

