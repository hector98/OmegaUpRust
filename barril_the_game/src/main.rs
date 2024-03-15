use std::io;

fn main() {
    let mut n_x = String::new();

    io::stdin().read_line(&mut n_x).expect("Failed to read line");
    let n_x: Vec<&str> = n_x.trim().split(' ').collect();
    //let n: i32 = n_x[0].parse().expect("Failed to parse input");
    //let x: i32 = n_x[1].parse().expect("Failed to parse input");

    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("Failed to read line");
    let m: Vec<&str> = m.trim().split(' ').collect();

    let mut i = 0;

    loop {
        //m[i] = m[i].parse().unwrap();
        if m[i] == n_x[1] {
            break;
        }
        i += 1;
    }

    println!("{}", i+1);
}
