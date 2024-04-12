// https://omegaup.com/arena/problem/Ordenando-por-distancia-al-orige/
use std::io;
use std::convert::TryInto;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");
    let n: i32 = n.trim().parse().expect("Please type a number");
    let mut array = vec![vec![0.0; 3]; n as usize];

    for _ in 0..n {
        let mut x_y = String::new();
        io::stdin()
            .read_line(&mut x_y)
            .expect("Failed to read line");
        let (x, y) = x_y.trim().split_once(" ").unwrap();

        let x: f32 = x.trim().parse().expect("Please type a number");
        let y: f32 = y.trim().parse().expect("Please type a number");
        let distancia: f32 = (x.powf(2.0) + y.powf(2.0)).sqrt();

        let mut i: usize = (n - 1).try_into().unwrap();
        let point = [x, y, distancia].to_vec();
        array[i] = point;

        loop {
            let aux: &Vec<f32> = &array[i].to_vec();

            if array[i][2] < array[i - 1][2] || array[i - 1][2] == 0.0 {
                let aux = &array[i].to_vec();   
                array[i] = array[i - 1].to_vec();
                array[i - 1] = aux.to_vec();
            } else if array[i][2] == array[i - 1][2] {
                if array[i][0] < array[i - 1][0] {
                    let aux = &array[i].to_vec();
                    array[i] = array[i - 1].to_vec();
                    array[i - 1] = aux.to_vec();
                } else if array[i][0] == array[i - 1][0] {
                    if array[i][1] < array[i - 1][1] {
                        let aux = &array[i].to_vec();
                        array[i] = array[i - 1].to_vec();
                        array[i - 1] = aux.to_vec();
                    }
                }
            } else {
                break;
            }

            i -= 1;

            if i == 0 {
                break;
            }

        }
    }

    for i in 0..n {
        println!("{:.0} {:.0}", array[i as usize][0], array[i as usize][1]);
    }
}
