// Raíz de la *biblioteca* del crate (`rust-notes`).
// .Todo lo que declares aquí con `pub` puede usarse desde los binarios
// de `src/bin/` con: `use rust::...`

// Módulo `utils`: agrupa utilidades reutilizables (helpers de input, etc.).
// Es solo un namespace; no es un fichero suelto fuera del proyecto.
pub mod utils {
    // Incluye el fichero `src/utils/input_two_integers.rs` como submódulo.
    // `pub` hace que sus funciones `pub` sean visibles fuera de `utils`.
    pub mod input_two_integers;
}
