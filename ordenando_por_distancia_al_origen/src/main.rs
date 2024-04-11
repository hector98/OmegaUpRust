// https://omegaup.com/arena/problem/Ordenando-por-distancia-al-orige/
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number");
    let mut array = vec![vec![::f32; 3]; n as usize];

    for _ in 0..n {
        let mut x_y = String::new();
        io::stdin()
            .read_line(&mut x_y)
            .expect("Failed to read line");
        let (x, y) = x_y.trim().split_once(" ").unwrap();

        let x: f32 = x.trim().parse().expect("Please type a number");
        let y: f32 = y.trim().parse().expect("Please type a number");
        let distancia: f32 = (x.powf(2.0) + y.powf(2.0)).sqrt();

        let mut i = n - 1;
        loop {
            if distancia < array[i][2] {
                let aux = array[i];
                array[i] = [x, y, distancia];
            } else if distancia == array[i][2] {
                if array[i][0] < x {
                } else if array[i][1] < y {
                } else {
                }
            }
        }

        println!("{}", distancia);
    }
}
