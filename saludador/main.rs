use std::io;

fn main(){
    loop{
        println!("Comandos:\n hola \n adios \n\n Escribe algo:");

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
