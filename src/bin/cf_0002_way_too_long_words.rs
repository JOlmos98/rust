use std::io;
const EXTRA_LOGS: bool = false;

fn main() {
    if EXTRA_LOGS {println!("Introduce un número entero (será el número de palabras que introducirás después):");}

    // while hasta que tengamos el entero correcto

    let mut input = String::new();

    // primero ofrecemos input del entero
    let words_number: u32 = loop {
        io::stdin().read_line(&mut input).expect(""); //Error al leer el input, introduce un número entero.

        // despues evaluamos que ha metido el user,
        match input.trim().parse() {
            // si es entero, perfecto, seguimos
            Ok(num) => {
                input.clear();
                break num;
            }
            // si no (else), volvemos a imprimir que meta un entero y reofrecemos input en bucle (while)
            Err(_) => {
                input.clear();
                if EXTRA_LOGS {println!("Introduce un número entero válido.");}
            }
        }
    };

    // cuando salimos del while del entero inicial entramos en otro while,
    // ahora queremos strings de caracteres sin números
    let mut words_counter = 0;
    let mut words: Vec<String> = Vec::new();

    while words_counter < words_number {
        if EXTRA_LOGS {println!("Introduce una palabra:");}

        io::stdin().read_line(&mut input).expect(""); //Error al leer el input, introduce una palabra sin números.

        input = input.trim().to_string();

        if es_palabra_valida(input.trim()) {
            let palabra = input.trim().to_string();
            words.push(palabra);
            words_counter += 1;
        }
        input.clear();
    }
    // ofrecemos input, evaluamos si es palabra sin numeros,
    // si sí, seguimos ofreciendo tantos inputs nuevos segun el entero inicial
    // si no, no avanza el counter de inputs ofrecidos para palabraas y volvemos a reofrecer el "mismo input" en bucle (while)

    let mut i = 0;
    while i < words.len() {
        let word = &words[i]; // &String — paniquea si i está fuera de rango

        if word.len() > 10 {
            let palabra_abreviada = abreviar_palabra(&word);
            println!("{}", palabra_abreviada);
        } else {
            println!("{}", word);
        }

        i += 1;
    }

    // cuando tengamos todas las palabras (segun el entero), evaluamos las palabras con un for y vemos si length > 10
    // si no, imprimimos palabra como tal,
    // si si, abreviamos y escribimos inicial, número de letras totales -2 (la inicial y la última restadas) y escribimos última letra

    // finaliza el programa
}

/// Esta función quiere un PRESTAMO, no una propiedad,
/// por lo que no se puede pasar directamente una variable o
/// constante String, hay que poner el "&" delante del nombre
/// de esa variable indicando que pasamos esa variable prestada y NO la propiedad
fn es_palabra_valida(palabra: &str) -> bool {
    // En Rust no hay `for (int i=0; ...)`. Recorres un rango o un iterador.
    // `.chars()` recorre cada carácter (el equivalente a `charAt(i)`).
    for c in palabra.chars() {
        if es_numero(&c.to_string()) {
            return false;
        }
    }

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

fn abreviar_palabra(palabra: &str) -> String {
    let inicial = palabra.chars().next().unwrap();
    let ultima_letra = palabra.chars().last().unwrap();
    let numero_intermedias = palabra.len() - 2;

    inicial.to_string() + &numero_intermedias.to_string() + &ultima_letra.to_string()

    // format!("{inicial}{numero_intermedias}{ultima_letra}") // También se podría así, de hecho, sería mejor
}

// let inicial = &palabra[0..1];           // &str (solo si es ASCII)
// o más seguro (Unicode):
// let inicial = palabra.chars().next().unwrap(); // char
