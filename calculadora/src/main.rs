use std::io::{self, Write};
use colored::Colorize;

fn main() {
    
    print!("{}", "Ingrese una cuenta: ".bold());
    io::stdout().flush().expect("Error al vaciar el buffer");

    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Error leyendo la linea");

    let string = string.trim().replace(" ", ""); 

    let mut chars: Vec<char> = string.chars().collect();
    chars.push(' ');
    
    println!("{} {}", "Resultado:".bold(), calc_ter(chars).to_string().cyan().bold());
        
}

fn calc_ter(chars: Vec<char>) -> f64 {
    let mut acum1 = vec![];
    let mut acum2 = vec![];
    let mut estado = false;
    let mut calculo = ' ';
    let mut resultado = 0.0;

    for char in chars.iter() {
        if *char == '*' || *char == ' ' || *char == '/' || *char == '+' || *char == '-' {
            
            if acum2.is_empty() {
                estado = true;
            } else {
                // Convertir los acumuladores en strings
                let acum3 = acum1.iter().collect::<String>();
                let acum4 = acum2.iter().collect::<String>();

                // Intentar convertir los strings en enteros
                let num1: f64 = acum3.parse().expect("Error al convertir número");
                let num2: f64 = acum4.parse().expect("Error al convertir número");

                // Realizar la multiplicación y mostrar el resultado
            
                if calculo == '*'  {
                    resultado = num1 * num2;
                } else if calculo == '/' {
                    resultado = num1 / num2;
                } else if calculo == '-' {
                    resultado = num1 - num2;
                } else if calculo == '+' {
                    resultado = num1 + num2;
                }
                acum1 = resultado.to_string().chars().collect();
                acum2 = vec![];
                
            }
            calculo = *char;
            continue;
        }

        if estado {acum2.push(*char);}
        else {acum1.push(*char);}
    }
    return resultado.into();
}
