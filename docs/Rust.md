[[Rust]].
[[Rust - Sintax]].

[[Rust - Competitive programming 1 - Watermelon]].

[[competitive-enunciado-rs]].

[[Ejercicios Java - Rust]].
[Herramientas relacionadas con Rust y la web](https://www.youtube.com/watch?v=4d4Uqyfzja4&ab_channel=Fazt).
[Cheatsheet de Rust](https://quickref.me/rust.html).
[Otro cheatsheet en Github de Rust](https://github.com/donbright/rust-lang-cheat-sheet).

Cosas: 
- Interesante crate para la utenticación web segura: [webauthn_rs](https://docs.rs/webauthn-rs/latest/webauthn_rs/).

## · Crear un ejecutable .exe y ejecutarlo en CMD.

1 - Creamos un programa a poder ser que funcione por línea de comandos, como este ejercicio de CodeForces:

```java
use std::io::{self, BufRead};

fn watermelon(n: i32) {
    if n > 1 && n <= 100 {
        if n % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    } else {
        println!("WEIGHT GREATER THAN 100 OR LESS THAN 1");
    }
}

fn main() {
    println!("Enter weight:");
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).expect("Failed to read line");
    let weight: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("INPUT ERROR");
            return;
        }
    };
    watermelon(weight);
}
```

2 - En la terminal escribimos `cargo build --release`.
3 - Ya tendríamos en la carpeta del proyecto en `proyecto\target\release` un archivo ejecutable .exe que podemos ejecutar por línea de comandos simplemente yendo al directorio y escribiendo su nombre.

# · APUNTES Rust
## Crates

Un Crate es una unidad de compilación, son similares a los paquetes de otros lenguajes. Un ejemplo de un Crate sería (compuesto por un archivo rs, un toml y un directorio de proyecto):

Archivo rs:
```rust
fn main() { 
	println!("Hello, world!"); 
	}
```

Archivo toml:
```toml
[package] 
name = "my_project" 
version = "0.1.0" 
authors = ["Tu Nombre <tuemail@example.com>"] 
edition = "2021" 

[dependencies] 
serde = "1.0"
```

Directorio:
```css
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

Podemos encontrar distintos crates en [crates.io](https://crates.io/).

### Esto dice Microsoft de los Crates

Un crate de Rust es una unidad de compilación. Es el fragmento de código más pequeño que puede ejecutar el compilador de Rust. El código de un crate se compila en conjunto para crear un archivo ejecutable binario o una biblioteca. En Rust, solo los crates se compilan como unidades reutilizables. Un crate contiene una jerarquía de módulos de Rust con un módulo implícito de nivel superior sin nombre.

### Esto dice ChatGPT de los Crates

En Rust, los "crates" son equivalentes a los paquetes en otros lenguajes de programación. Los crates son unidades de compilación y distribución de código en Rust, similares a cómo funcionan los paquetes en lenguajes como JavaScript (npm), Python (pip), o Java (Maven).

Detalles sobre los Crates en Rust

1. **Crate de Biblioteca y Crate de Binario**:
    
    - **Biblioteca (Library Crate)**: Contiene código que se puede reutilizar en otros programas. No tiene un punto de entrada como `main`.
    - **Binario (Binary Crate)**: Contiene un punto de entrada `main` y puede ser ejecutado como un programa.
2. **Cargo y Crates.io**:
    
    - **Cargo**: Es la herramienta de gestión de paquetes y compilación de Rust. Cargo gestiona las dependencias del proyecto, compila el código y crea ejecutables.
    - **Crates.io**: Es el registro oficial de crates de Rust, similar a cómo npm es para JavaScript y PyPI es para Python.
## Cargo

Aunque se puede usar el compilador de Rust (`rustc`) directamente para crear crates, en la mayoría de los proyectos se usa la herramienta de compilación de Rust y un administrador de dependencias llamado **Cargo**.

Cargo hace gran cantidad de cosas, entre las que se incluyen las siguientes:

- Crear nuevas plantillas de proyecto con el comando `cargo new`.
- Compilar un proyecto con el comando `cargo build`.
- Compilar y ejecutar un proyecto con el comando `cargo run`.
- Probar un proyecto con el comando `cargo test`.
- Comprobar los tipos de proyecto con el comando `cargo check`.
- Compilar la documentación de un proyecto con el comando `cargo doc`.
- Publique una biblioteca para `crates.io` con el comando `cargo publish`.
- Para agregar crates dependientes a un proyecto, agregue el nombre del crate al archivo Cargo.toml.

## Comandos básicos de Cargo para crear un proyecto

Para crear un proyecto y hacer lo básico con él se usan los siguientes comandos:

- `cargo new <nombre_proyecto>`: Crea un nuevo proyecto de Rust en un nuevo directorio con el nombre especificado.
- `cargo init` (en caso de no haber ejecutado ``cargo new``): Una vez en el directorio donde queremos generar el proyecto, escribimos en la terminal este comando y se generan los archivos.
- `cargo build`: Compila el proyecto de Rust en el directorio actual. Los archivos de compilación se colocan en el directorio `target`.
- `cargo run`: Compila y ejecuta el proyecto de Rust. Es útil para pruebas rápidas y desarrollo continuo.
- `cargo test`: Ejecuta las pruebas del proyecto de Rust. Esto compila y corre todas las funciones de prueba dentro del proyecto.
- `cargo check`: Verifica el proyecto sin compilarlo por completo. Es más rápido que `cargo build` y útil para comprobar errores rápidamente.
- `cargo clean`: Elimina los archivos de compilación en el directorio `target`, limpiando el proyecto.
- `cargo doc --open`: Genera documentación para el proyecto basado en comentarios y abre la documentación en un navegador web.
- `cargo update`: Actualiza las dependencias en el archivo `Cargo.lock` a sus versiones más recientes.
- `cargo publish`: Publica el proyecto en el registro de crates.io para compartirlo con la comunidad de Rust.
- `cargo install <nombre_crate>`: Instala un crate binario desde crates.io en tu sistema local.

## 1. Ownership y borrowing, propiedad y préstamo.

```rust
let s = String::from("Hola");

let t = s; // s ya no es válido aquí

println!("{}", t);
```

## 2. Símbolo &.

```rust
let s = String::from("hola");
let t = &s; // t es una referencia a s
println!("{}", t);
```

Es decir, & señala a la variable s, es como un puntero, como un `*s`.

## 3. Option y Result.

```rust
let some_number = Some(5);
match some_number {
    Some(n) => println!("El número es {}", n),
    None => println!("No hay número"),
}
```

El matching: si una variable encaja, hacemos esto, si no, hacemos otra cosa, como una especie de Switch.

Otro ejemplo de match:

```rust
let num = 5;
match num {
1 => println!("Uno"),
2 => println!("Dos"),
_ => println!("Otro número"),
}

```
## 4. 


## 5. 
## 
## 
## 

# Esto dice Gemini sobre Rust:

Rust se considera un lenguaje de programación superior a otros por varias razones:

**Seguridad en la memoria:** Rust está diseñado para evitar errores de memoria comunes como punteros nulos y fugas de memoria. Esto lo hace un lenguaje ideal para desarrollar sistemas confiables y de alto rendimiento.

**Rendimiento:** Rust es un lenguaje compilado que se traduce en código máquina eficiente. Esto lo hace comparable en rendimiento a lenguajes como C y C++, mientras que ofrece características de seguridad de memoria superiores.

**Concurrencia:** Rust proporciona un sistema de tipos seguro para la concurrencia, lo que facilita la escritura de código que se ejecuta en paralelo de manera segura y eficiente.

**Productividad:** Rust tiene un sistema de propiedad que ayuda a los programadores a evitar errores comunes y a escribir código más limpio y mantenible.

**Comunidad:** Rust tiene una comunidad vibrante y activa que está dispuesta a ayudar a los nuevos usuarios a aprender el lenguaje.

**Aplicaciones:** Rust es un lenguaje versátil que se puede utilizar para desarrollar una amplia gama de aplicaciones, desde sistemas operativos y embebidos hasta aplicaciones web y juegos.

Sin embargo, es importante tener en cuenta que Rust también tiene algunas desventajas:

**Curva de aprendizaje:** Rust tiene una curva de aprendizaje más pronunciada que otros lenguajes de programación. Esto se debe a su sistema de tipos complejo y a su enfoque en la seguridad en la memoria.

**Ecosistema:** El ecosistema de Rust es más pequeño que el de otros lenguajes de programación populares como Python o JavaScript. Esto significa que puede haber menos bibliotecas y herramientas disponibles para Rust.

**En general, Rust es un lenguaje de programación poderoso y versátil que ofrece muchas ventajas sobre otros lenguajes. Sin embargo, es importante tener en cuenta su curva de aprendizaje y su ecosistema más pequeño antes de decidir si es el lenguaje adecuado para ti.**

- **Web para Crates de la comunidad de Rust:** [crates.io](https://crates.io/)
- **Sitio web oficial de Rust:** [https://www.rust-lang.org/](https://www.rust-lang.org/)
- **Libro de Rust:** [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
- **Documentación de Rust:** [https://docs.rs/](https://docs.rs/)
# Sobre el SO

Sí, puedes crear tu propio sistema operativo desde cero utilizando **Rust** y una **Raspberry Pi** como hardware base. Sin embargo, es un proyecto ambicioso que requiere conocimientos avanzados en programación de bajo nivel, arquitectura de sistemas y hardware. A continuación, te detallo los aspectos clave y pasos generales para lograrlo:

---

### 1. **¿Por qué Rust?**

Rust es un excelente lenguaje para este tipo de proyectos porque ofrece:

- **Control de bajo nivel:** Acceso directo a la memoria y al hardware.
- **Seguridad en la memoria:** Minimiza errores comunes como desbordamientos de buffer o punteros nulos.
- **Buen rendimiento:** Comparable a C o C++.
- **Comunidad activa:** Hay recursos, bibliotecas y ejemplos para crear sistemas operativos en Rust.

---

### 2. **Hardware: Raspberry Pi**

La Raspberry Pi es una plataforma adecuada porque:

- Es compacta y accesible.
- Tiene una amplia documentación técnica.
- Puedes encontrar kits de pantallas compatibles para la interfaz visual.

Las versiones más recientes (como la Raspberry Pi 4) ofrecen hardware más potente, ideal para experimentar.

---

### 3. **Pasos principales para crear un SO con Rust**

#### A. **Preparación inicial**

1. **Configurar el entorno:**
    
    - Instala Rust (`rustup`).
    - Asegúrate de tener el compilador adecuado para el hardware ARM de Raspberry Pi (usa `rustc` con el objetivo `arm-none-eabi` o `aarch64-none-elf`).
    - Configura `QEMU` (un emulador) para probar tu sistema operativo antes de cargarlo en la Raspberry Pi.
2. **Obtén la documentación técnica:**
    
    - Descarga el manual de referencia del hardware (BCM2835 o BCM2837, según tu modelo). Esto es crucial para entender cómo interactuar con los periféricos (GPU, GPIO, etc.).

#### B. **Crear el núcleo (Kernel)**

1. **Escribe un cargador de arranque:**
    
    - La Raspberry Pi espera un archivo llamado `kernel.img` en la tarjeta SD para arrancar. Este archivo es tu sistema operativo compilado.
2. **Inicializa el hardware:**
    
    - Configura el controlador de interrupciones, temporizadores y UART (para depuración).
    - Configura la pantalla si estás usando una interfaz gráfica.
3. **Crea un bucle principal:**
    
    - Este bucle manejará las tareas del sistema operativo, como eventos de hardware o programación de procesos.
4. **Implemente controladores básicos:**
    
    - **GPIO:** Para interactuar con botones o LEDs.
    - **Pantalla:** Para mostrar texto o gráficos.
    - **Teclado:** Si necesitas entrada.

#### C. **Desarrolla funciones avanzadas**

1. **Sistema de tareas:**
    
    - Implementa un planificador básico para multitarea.
    - Maneja interrupciones para cambiar entre tareas.
2. **Gestión de memoria:**
    
    - Diseña un administrador de memoria para manejar el acceso y la asignación de RAM.
3. **Sistema de archivos (opcional):**
    
    - Implementa un controlador para leer y escribir archivos en la tarjeta SD.

#### D. **Interfaz gráfica**

1. Usa el controlador de GPU de la Raspberry Pi para mostrar gráficos en la pantalla.
2. Empieza con una interfaz simple que pueda mostrar texto e imágenes.

#### E. **Pruebas y depuración**

1. Prueba tu sistema operativo en un emulador como QEMU.
2. Graba el archivo `kernel.img` en una tarjeta SD y prueba en la Raspberry Pi.

---

### 4. **Herramientas y recursos útiles**

- **Tutoriales y frameworks:**
    
    - [BlogOS](https://os.phil-opp.com/) (Guía paso a paso para escribir un sistema operativo en Rust).
    - Proyectos en GitHub como [RustOS](https://github.com/thepowersgang/rust_os).
- **Emulador QEMU:** Para probar el SO sin necesidad de cargarlo en hardware real constantemente.
    
- **Libros recomendados:**
    
    - _"Operating Systems: Three Easy Pieces"_ para entender conceptos fundamentales.
    - _"The Rust Programming Language"_ para dominar Rust.

---

### 5. **Desafíos comunes**

- **Interacción con hardware:** Comprender y manejar los registros y controladores de la Raspberry Pi puede ser complicado.
- **Depuración:** Sin un sistema operativo base, depurar en bajo nivel requiere herramientas y técnicas avanzadas.
- **Plazo largo:** Este tipo de proyectos puede tomar meses o años, dependiendo de su alcance.

---

### 6. **Consejo final**

Empieza con un sistema operativo básico que pueda encender la pantalla, mostrar un mensaje y manejar alguna interacción sencilla (como un botón o un LED). Luego, expande gradualmente tu proyecto, añadiendo características como multitarea, gráficos avanzados o un sistema de archivos.