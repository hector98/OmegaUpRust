// https://omegaup.com/arena/problem/Tabla-de-factoriales/

//use num_integer::Integer;

fn main() {
    let mut valor = String::from("1");
    for i in 0..101 {
        if i == 0 {
            println!("{}! = {}", i, 1);
        } else {
            valor = factorial(i, &valor);
            println!("{}! = {}", i, valor);
        }
    }
}

fn factorial<'a>(n: u32, valor_ant: &str) -> String {
    let valor_ant = valor_ant.to_string();

    let result = multiply(&valor_ant, n).to_string();

    return result.to_string().chars().rev().collect();
}

fn multiply<'a>(x: &str, y: u32) -> String {
    let mut llevo: u32 = 0;
    let mut result = String::from("");
    let x = x.chars().rev().collect::<String>();
    let num: Vec<&str> = x.trim().split("").collect();

    for i in num {
        if i != "" {
            let aux_i: u32 = i.trim().parse().unwrap();
            //let aux_i = 8;
            let aux_result: u32 = (aux_i * y) + llevo;

            if aux_result >= 10 {
                //let aux_result = aux_result.to_string();
                //let aux: Vec<&str> = aux_result.trim().split("").collect();

                llevo = aux_result / 10;
                //result.push_str(llevo.to_string().as_str());
                //result.chars().rev().collect::<String>();
                result.push_str(&(aux_result % 10).to_string());
            } else {
                llevo = 0;
                result.push_str(&aux_result.to_string());
            }
        }
    }

    if llevo != 0 {
        result.push_str(&llevo.to_string().chars().rev().collect::<String>());
    }

    return result.to_string();
}
