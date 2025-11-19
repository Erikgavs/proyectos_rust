use rand::Rng;
use std::io;

fn main(){
    // mutable porque tiene que generar distintos números, no va a ser fijo
    let mut rng = rand::thread_rng();
    // No mutable porque quiero que sea fijo mientras el usuario lo adivina
    let s_num = rng.gen_range(1..=10);

    // loop mientras el usuario prueba a encontrar el número
    loop {
        println!("intenta adivinar el número (1-10)");
        let mut respuesta = String::new();
        io::stdin()
            .read_line(&mut respuesta)
            .expect("Error");
        let respuesta: i32 = respuesta.trim().parse().expect("introduce un número válido");

        if respuesta == s_num {
            println!("Has adivinado el número!: {}", s_num);
            break;
        } else {
            println!("No has adivinado el número, sigue intentando!");
        }
    }
}
