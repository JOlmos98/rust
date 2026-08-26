use std::io;

fn main() {
    println!("Introduce un número entero (será el número de palabras que introducirás después):");

    // while hasta que tengamos el entero correcto

    let mut input = String::new();

    // primero ofrecemos input del entero
    let weight: u32 = loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Error al leer el input, introduce un número entero.");

        match input.trim().parse() {
            Ok(num) => {
                break num;
            }
            Err(_) => {
                println!("Introduce un número entero válido.")
            }
        }
    };
    // despues evaluamos que ha metido el user,
    // si es entero, perfecto, seguimos
    // si no (else), volvemos a imprimir que meta un entero y reofrecemos input en bucle (while)

    let mut is_correct_input = false;
    let mut palabras: Vec<String> = Vec::new();


    while is_correct_input == false {

    io::stdin()
            .read_line(&mut input)
            .expect("Error al leer el input, introduce una palabra sin números.");

        match input.trim().parse()


    }

    // cuando salimos del while del entero inicial entramos en otro while,
    // ahora queremos strings de caracteres sin números

    // ofrecemos input, evaluamos si es palabra sin numeros,
    // si sí, seguimos ofreciendo tantos inputs nuevos segun el entero inicial
    // si no, no avanza el counter de inputs ofrecidos para palabraas y volvemos a reofrecer el "mismo input" en bucle (while)

    // cuando tengamos todas las palabras (segun el entero), evaluamos las palabras con un for y vemos si length > 10
    // si no, imprimimos palabra como tal,
    // si si, abreviamos y escribimos inicial, número de letras totales -2 (la inicial y la última restadas) y escribimos última letra

    // finaliza el programa
}

fn es_palabra_valida(palabra: &str) -> bool {

    // En Rust no hay `for (int i=0; ...)`. Recorres un rango o un iterador.
    // `.chars()` recorre cada carácter (el equivalente a `charAt(i)`).
    for c in palabra.chars() {
        if es_numero(&c.to_string()) {
            return false;
        }
    };

    return true;
}

fn es_numero(posible_num: &str) -> bool {
    match posible_num {
        "0" => true,
        "1" => true,
        "2" => true,
        "3" => true,
        "4" => true,
        "5" => true,
        "6" => true,
        "7" => true,
        "8" => true,
        "9" => true,
        _ => false, // esto es el default
    }
}
