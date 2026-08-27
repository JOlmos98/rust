use std::io;
const EXTRA_LOGS: bool = false;

fn main() {
    let mut input = String::new();

    if EXTRA_LOGS {println!("Inserta dos enteros, el primero será la cantidad de participantes y el segundo el puesto del participante que será la nota de corte:")}
    io::stdin()
        .read_line(&mut input)
        .expect("First input error");

    // leemos el input y SOLO SI son dos enteros separados
    // por un espacio, seguimos, si no, error

    let participants: usize;
    let position: usize;

    if let Some(pos) = input.find(' ') {
        let mut maybe_number = &input[0..pos];

        match maybe_number.parse::<usize>() {
            Ok(num) => {
                participants = num;
                // input.clear();
            }
            Err(_) => {
                input.clear();
                // no era un entero válido
                if EXTRA_LOGS {println!("Error al insertar los dos enteros pedidos.");}
                return; // o println! y volver a pedir, etc.
            }
        }
        // maybe_number = &input[pos..input.len()];
        // Incluye el espacio y el '\n'; hay que hacer trim antes del parse
        maybe_number = input[pos..].trim();

        match maybe_number.parse::<usize>() {
            Ok(num) => {
                position = num.clone();
                input.clear();
            }
            Err(_) => {
                // no era un entero válido
                input.clear();
                if EXTRA_LOGS {println!("Error al insertar los dos enteros pedidos.");}
                return; // o println! y volver a pedir, etc.
            }
        }
    } else {
        // no había espacio
        input.clear();
        return;
    }

    // Interesante forma de coger las partes de un string separando por espacios en blanco.
    // let mut partes = input.split_whitespace();
    // let n: u32 = partes.next().unwrap().parse().unwrap();
    // let k: u32 = partes.next().unwrap().parse().unwrap();
    if EXTRA_LOGS {println!("Inserta las notas, desc, de todos los participantes, enteros separados por un espacio:");}

    // input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Second input error");

    let num_partes = input.split_whitespace().count();
    let mut partes = input.split_whitespace();
    let scores: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    if num_partes != participants {
        if EXTRA_LOGS {println!("Error, la cantidad de notas insertadas son distintas del número de participantes.");}
        return;
    };
    
    let score_participant_selected = &scores[position - 1];
    let mut next_round_participants = 0;

    while let Some(parte) = partes.next() {

        match parte.parse::<usize>() {
            Ok(num) => {
                if num >= *score_participant_selected && num > 0 {next_round_participants+=1}
            }
            Err(_) => {
                if EXTRA_LOGS {println!("Error al insertar los enteros pedidos.");}
                return;
            }
        }
    }

    if EXTRA_LOGS {println!("La nota de corte es {} y pasan {} participantes.", score_participant_selected, next_round_participants);}

    println!("{}", next_round_participants);

    if EXTRA_LOGS {println!("Programa finalizado correctamente.");}

    // depues tendremos que meter tantos enteros en el
    // segundo input (separados por un espacio) segun el
    // primer entero del primer input y validamos

    // el programa, una vez validados los inputs,
    // vemos que nota tiene el user en el puesto
    // del segundo entero del primer input, esa nota
    // será la de corte para avanzar a la siguiente
    // ronda, vemos entonces cuantos usuarios avanzarian
    // (tambien avanzan los que queden por detras de ese
    // usuario SI TIENEN LA MISMA NOTA QUE EL)
}
