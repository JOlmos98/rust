## Índice

- [Statement](#statement)
  - [A. Beautiful Matrix](#a-beautiful-matrix)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Matriz hermosa](#a-matriz-hermosa)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Beautiful Matrix

**time limit per test:** 2 seconds  
**memory limit per test:** 256 megabytes

You've got a `5 × 5` matrix, consisting of 24 zeroes and a single number one. Let's index the matrix rows by numbers from `1` to `5` from top to bottom, let's index the matrix columns by numbers from `1` to `5` from left to right. In one move, you are allowed to apply one of the two following transformations to the matrix:

- Swap two neighboring matrix rows, that is, rows with indexes `i` and `i + 1` for some integer `i` (`1 ≤ i < 5`).
- Swap two neighboring matrix columns, that is, columns with indexes `j` and `j + 1` for some integer `j` (`1 ≤ j < 5`).

You think that a matrix looks beautiful, if the single number one of the matrix is located in its middle (in the cell that is on the intersection of the third row and the third column). Count the minimum number of moves needed to make the matrix beautiful.

## Input

The input consists of five lines, each line contains five integers: the `j`-th integer in the `i`-th line of the input represents the element of the matrix that is located on the intersection of the `i`-th row and the `j`-th column. It is guaranteed that the matrix consists of 24 zeroes and a single number one.

## Output

Print a single integer — the minimum number of moves needed to make the matrix beautiful.

## Examples

Input
```bash
0 0 0 0 0
0 0 0 0 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
```

Output
```bash
3
```

Input
```bash
0 0 0 0 0
0 0 0 0 0
0 1 0 0 0
0 0 0 0 0
0 0 0 0 0
```

Output
```bash
1
```

---

# Enunciado (español)

## A. Matriz hermosa

**límite de tiempo por test:** 2 segundos  
**límite de memoria por test:** 256 megabytes

Tienes una matriz de `5 × 5`, formada por 24 ceros y un único número uno. Indexamos las filas de la matriz con números del `1` al `5` de arriba hacia abajo, y las columnas con números del `1` al `5` de izquierda a derecha. En un movimiento, se te permite aplicar una de las dos transformaciones siguientes a la matriz:

- Intercambiar dos filas vecinas de la matriz, es decir, las filas con índices `i` e `i + 1` para algún entero `i` (`1 ≤ i < 5`).
- Intercambiar dos columnas vecinas de la matriz, es decir, las columnas con índices `j` y `j + 1` para algún entero `j` (`1 ≤ j < 5`).

Consideras que una matriz es hermosa si el único número uno de la matriz está situado en el centro (en la celda que está en la intersección de la tercera fila y la tercera columna). Cuenta el número mínimo de movimientos necesarios para hacer la matriz hermosa.

## Entrada

La entrada consiste en cinco líneas; cada línea contiene cinco enteros: el `j`-ésimo entero en la `i`-ésima línea de la entrada representa el elemento de la matriz situado en la intersección de la `i`-ésima fila y la `j`-ésima columna. Se garantiza que la matriz consiste en 24 ceros y un único número uno.

## Salida

Imprime un único entero — el número mínimo de movimientos necesarios para hacer la matriz hermosa.

## Ejemplos

Input
```bash
0 0 0 0 0
0 0 0 0 1
0 0 0 0 0
0 0 0 0 0
0 0 0 0 0
```

Output
```bash
3
```

Input
```bash
0 0 0 0 0
0 0 0 0 0
0 1 0 0 0
0 0 0 0 0
0 0 0 0 0
```

Output
```bash
1
```

# Resumen

Dada una matriz `5 × 5` con 24 ceros y un único `1`, calcula el mínimo número de intercambios de filas o columnas vecinas necesarios para llevar ese `1` a la celda central `(3, 3)`. Imprime ese número.
