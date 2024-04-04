// https://omegaup.com/arena/problem/pb-Cuadrado/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i16 = n.trim().parse().unwrap();
    let mut m_original = vec![vec![]; n as usize];

    for i in 0..n {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        line = line.replace("\n", "");
        //let line: Vec<String> = line.split(' ').map(|s| s.to_string()).collect();

        m_original[i as usize] = line.split(' ').map(|s| s.to_string()).collect();
        drop(line);
    }

    //println!("{:?}", m_original);

    let mut m_giro = String::from("");
    let mut i: i16 = n - 1;
    let mut j: i16 = 0;
    loop {
        //println!("{}", m_original[i as usize][j as usize]);
        let aux_str = &m_original[i as usize][j as usize].as_str();
        m_giro += format!("{} ", &aux_str).as_str();
        
        if i == 0 {
            j += 1;
            m_giro.push_str("\n");
            if j == n {
                //i = 0;
                drop(aux_str);
                break;
            } else {
                i = n - 1;
            }
        } else {
            i -= 1;   
        }

    }

    i = 0;
    j = 0;
    drop(m_original);
    println!("{}", m_giro);
    drop(m_giro);
}
