use std::io;

fn main(){
    loop{
        println!("Comandos:\n hola \n adios \n numero \n\n Escribe algo:");

        /* Creamos una variable llamada entrada, es mutable => mut, podremos cambiar su contenido
        * String::new(); => Crea un string vacío donde vamos a guardar lo que el usuario escriba
        */
        let mut entrada = String::new();
        io::stdin() /* Entrada estandar (Teclado) */
            .read_line(&mut entrada) /* Lee una línea completa y lo guarda en la variable entrada */
            /* &mut entrad => nos envía a la variable entrada sin craar una nueva */
            .expect("No he podido leer la línea"); /* si algo falla, imprime eso */

        /* Elimina espacios en blanco y saltos de línea*/
        /* La anterior variable ya no es accesible y la remplazamos con esta */
        let entrada = entrada.trim(); /* Con esto creamos una nueva variable con el valor de entrada pero límpia (sin salto de línea y más) */
        match entrada { /* estructura de control que compara un valor con varias opciones */
            /* Si el usuario escribe hola, responde lo siguiente */
            "hola" => {
                println!("Hola, como te llamas?");
                let mut nombre = String::new();
                io::stdin()
                    .read_line(&mut nombre)
                    .expect("No he podido leer la línea");
                let nombre = nombre.trim();
                /* Vec<&str> crea un array con el mensaje */
                let palabras: Vec<&str> = nombre.split_whitespace().collect();
                // "me llamo juan" => "me" "llamo" "juan"
                let ult = palabras.last().unwrap();
                /* .last => selecciona el ultimo string dentro de la lista */
                /* .unwrap => lo saca de la lista y lo pasa a la varaible, ahora el ultimo valor es juan */

                println!("Encantado de conocerte, {}", ult);
            }
            "numero" => {
                println!("Que tipo de operación quieres hacer?\n\n Operaciones disponibles:\n\n +\n -\n *\n /\n")
                let mut op = String::new();
                io::stdin()
                    .read_line(&mut op)
                    .expect("No he podido leer la línea");
                let op = op.trim();
                /* op.chars convierte el input en ejemplo => "+" = ["+"]  Otro ejemplo =>  "Hola" = ["h", "o", "l", "a"]
                 * chars => recorrer las letras de una palabra y meterlas en unarray
                 *
                 * .next => recoge el primer cáracter ejemplos => ["+"] = + ejempo2 => ["h", "o", "l", "a"] = "h"
                 *
                 * .unwrap => saca el cáracter seleccionado para ser usado
                 */
                let operador = op.chars().next().unwrap();

                // Primer número
                println!("Introduce el primer número:");
                let mut n1 = String::new();
                io::stdin()
                    .read_line(&mut n1)
                    .except("Ha habido un problema");
                let n1 = n1.trim()
                    .parse::<f32>() // float
                    .expect("Numero inválido")

                // Segundo número
                println!("Introduce el segundo número:");
                let mut n2 = String::new();
                io::stdin()
                    .read_line(&mut n2)
                    .expect("Número inválido");
                let n2 = n2.trim()
                    .parse::<f32>() /* string a número */
                    .expect("Número inválido");

                let resultado = match operador {
                    '+' => n1 + n2,
                    '-' => n1 - n2,
                    '*' => n1 * n2,
                    '/' => n1 / n2,
                    /* Si no es ninguno de los anteriores: */
                    _=> {
                        println!(operador inválido);
                        continue;
                    }
                };

                println!("El resultado del cálculo es: {}", resultado)
                break;
            }
            "adios" => {
                println!("Que vaya bien");
                break; // Cerramos bucle
            }
            // _=> cualquier valor que no coincida con los demás, tendrá esta respuesta
            _=> println!("No entiendo lo que has escrito")
        }
    }
}
