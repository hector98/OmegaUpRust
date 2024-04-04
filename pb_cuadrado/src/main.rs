// https://omegaup.com/arena/problem/pb-Cuadrado/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i16 = n.trim().parse().unwrap();
    let mut m_original: Vec<String> = Vec::new();

    for _ in 1..=n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        
        //let line: Vec<i16> = line
        //  .trim()
        //  .split(' ')
        //  .map(|s| s.parse().unwrap())
        //  .collect();
        //println!("{}", line[0..line.len()-1].to_string());

        m_original.push(line[0..line.len()-1].to_string());
    }

    //println!("{:?}", m_original);

    let mut m_giro = String::from("");
    let mut i: i16 = n - 1;
    let mut j: i16 = 0;
    while i >= 0 {
        let aux_str = m_original[i as usize].chars().rev().collect::<String>();
        //m_giro.push_str(aux_str);

        if j != 1 {
            m_giro.push_str("\n");
        }
        
        i -= 1;
        j += 1;
    }

    println!("{}", m_giro);
}
