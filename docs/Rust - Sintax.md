[[Rust]].
[[Rust - Sintax]].

```rust
/// Representa el estado de un usuario en el sistema.
/// Los enums en Rust son muy potentes, pero aquí tienes su forma más básica.
enum Estado {
    Activo,
    Inactivo,
    Baneado,
}

/// Suma dos números enteros y devuelve el resultado.
/// 
/// # Argumentos
/// * `a` - El primer número
/// * `b` - El segundo número
fn sumar(a: i32, b: i32) -> i32 {
    // En Rust, la última línea sin punto y coma es el `return` implícito. 
    a + b
    // También se puede "return a + b;"  pero es menos común.
}

fn main() {
    // 1. CONSTANTES
    // Siempre son inmutables, el tipo es 100% obligatorio y usan mayúsculas.
    const LIMITE_MAXIMO: u32 = 100;

    // 2. VARIABLES PRIMITIVAS, TIPADAS Y MUTABLES
    // Usamos `mut` para poder cambiarlas. El tipado (i32, f64, bool) suele ser
    // inferido por el compilador, pero aquí está explícito como pediste.
    let mut contador: i32 = 0;
    let mut precio: f64 = 19.99;
    let mut es_valido: bool = true;
    let mut letra: char = 'A'; // Las comillas simples son para char, las dobles para String

    // Modificando una variable mutable
    contador += 1;

    // 3. BUCLE WHILE
    while contador < 3 {
        contador += 1;
    }

    // 4. BUCLE FOR
    // Iterando un rango. `1..5` es de 1 a 4. `1..=3` es de 1 a 3 (inclusivo).
    for i in 1..=3 {
        println!("Bucle for iteración: {}", i);
    }

    // Usando la función
    let resultado_suma = sumar(10, 5);

    // 5. SWITCH CASE (MATCH)
    // En Rust no existe `switch`, se usa `match`. 
    // Es mucho más seguro porque te OBLIGA a cubrir todos los casos posibles.
    let estado_actual = Estado::Activo;

    match estado_actual {
        Estado::Activo => println!("El usuario puede entrar."),
        Estado::Inactivo => println!("Falta confirmar el email."),
        Estado::Baneado => {
            // Si necesitas más de una línea, usas llaves
            println!("Usuario bloqueado.");
            println!("Contactar soporte.");
        }
    }
}
```