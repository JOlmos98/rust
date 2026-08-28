use std::io;

pub fn input_two_integers(log: Option<bool>) -> (usize, usize) {
    let log = log.unwrap_or(false);

    if log {println!("Insert two integers separated with white spaces:")};

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("input_two_integers error");

    let mut parts = input.split_whitespace();
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    
    (a, b)
}
