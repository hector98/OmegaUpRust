// https://omegaup.com/arena/problem/Tabla-de-factoriales/

//use num_integer::Integer;

fn main() {
    for i in 0..100 {
        if i == 0 {
            println!("{}! = {}", i, 1);
        } else {
            println!("{}! = {}", i, factorial(i));
        }
    }
}

fn factorial(n: u32) -> u128 {
    let mut result: u128 = 1;

    for i in 1..n {
        result *= i as u128;
    }

    return result;
}
