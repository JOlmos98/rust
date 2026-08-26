use std::io;

fn main() {


    println!("Introduce un número entero (será el número de palabras que introducirás después):");

    // while hasta que tengamos el entero correcto

    let mut input = String::new();
    let mut is_correct_input = false;

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer el input, introduce un número entero.");



    // primero ofrecemos input del entero

    // despues evaluamos que ha metido el user, 
    // si es entero, perfecto, seguimos
    // si no (else), volvemos a imprimir que meta un entero y reofrecemos input en bucle (while)



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