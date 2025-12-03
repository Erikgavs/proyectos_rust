use rand::Rng;
use std::io;

fn main(){
    let mut rng = rand::thread_rng();
    let numero = rng.gen_range(1..=10);

    loop {
        println!("Adivina el número secreto (1-10)");
        let mut respuesta = String::new();
        io::stdin()
            .read_line(&mut respuesta)
            .expect("Respuesta no válida");
        let respuesta: i32 = respuesta.trim().parse().expect("Número no válido");

        if respuesta == numero {
        println!("Has adivinado el número secreto!: {}", numero);
        break;
        } else {
            println!("No has adivinado el número, vuelve a intentarlo");
        }
    }
}
