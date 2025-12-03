use std::io;

fn main() {
    loop {
        println!("Comandos:\n hola \n adios \n numero \n\n Escribe algo:");

        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error detectado");
        let entrada = entrada.trim();
        match entrada {
            "hola" => {
                println!("Hola, como te llamas?");
                let mut nombre = String::new();
                io::stdin().read_line(&mut nombre).expect("Error detectado");
                let nombre = nombre.trim();
                let palabras: Vec<&str> = nombre.split_whitespace().collect();
                let ult = palabras.last().unwrap();

                println!("Encantado de conocerte, {}\n", ult);
            }

            "numero" => {
                println!("Pon un operador \n \n operadores disponibles: \n \n +\n -\n *\n /\n");
                let mut op = String::new();
                io::stdin().read_line(&mut op).expect("No ha funcionado");
                let op = op.trim();
                let operador = op.chars().next().unwrap();

                println!("Específica el primer número");
                let mut n1 = String::new();
                io::stdin().read_line(&mut n1).expect("Error en el código");
                let n1 = n1.trim().parse::<f32>().expect("Error en el parse");

                println!("Específica el primer número");
                let mut n2 = String::new();
                io::stdin().read_line(&mut n2).expect("Error en el código");
                let n2 = n2.trim().parse::<f32>().expect("Error en el parse");

                let resultado = match operador {
                    '+' => n1 + n2,
                    '-' => n1 - n2,
                    '*' => n1 * n2,
                    '/' => n1 / n2,
                    _ => {
                        println!("Operador no válido");
                        continue;
                    }
                };

                println!("El resultado de tu operación es: {}", resultado);
                break;
            }

            "adios" => {
                println!("Nos vemos!");
                break;
            }

            _ => println!("input vacío o incorrecto"),
        }
    }
}
