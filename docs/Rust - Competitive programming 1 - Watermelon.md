[[Rust]].
[[Rust - Sintax]].

```rust
use std::io;

fn main() {
    // Iniciamos un bucle infinito y le asignamos el resultado directamente a `weight`
    let weight: u32 = loop {
        println!("Introduce el peso de la sandía en kilos:");

        // La lectura debe estar DENTRO del bucle para volver a preguntar si falla
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer la entrada del usuario");

        // Evaluamos lo introducido
        match input.trim().parse() {
            Ok(num) => {
                // break no solo rompe el bucle, sino que en Rust puede devolver el valor 'num'
                // Ese valor irá directo a la variable `weight`
                break num;
            }
            Err(_) => {
                // Si falla, avisamos. El bucle vuelve al inicio y pide datos otra vez.
                println!("Por favor, introduce un número entero válido.");
            }
        }
    };

    // Aplicamos la lógica (aquí weight ya existe y tiene un número garantizado)
    if weight > 2 && weight % 2 == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}

```