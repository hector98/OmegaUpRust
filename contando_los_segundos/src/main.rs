use std::io;

fn main() {
    let mut date = String::new();

    io::stdin()
        .read_line(&mut date)
        .expect("Failed to read line");
    
    let date: Vec<&str> = date.trim().split(" ").collect();

    let (mut d, mut h, mut m, s): (i32, i32, i32, i32) = (date[0].parse().unwrap(), date[1].parse().unwrap(), date[2].parse().unwrap(), date[3].parse().unwrap()); 

    let (d_s, h_s, m_s, s_s): (i32, i32, i32, i32) = (14, 24, 60, 60);

    if d == d_s {
        println!("Feliz San Valentin");
    }else if d > d_s {
        println!("Mas suerte para la siguiente");
    }else {
        m = if s > 0 { m + 1 } else { m };
        h = if m > 0 { h + 1 } else { h };
        d = if h > 0 { d + 1 } else { d };

        println!("{} {} {} {}", d_s - d, h_s - h, m_s - m, s_s - s);
    }

}
