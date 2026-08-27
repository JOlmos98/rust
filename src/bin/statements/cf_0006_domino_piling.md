## Índice

- [Statement](#statement)
  - [A. Domino piling](#a-domino-piling)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Colocación de dominós](#a-colocación-de-dominós)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Domino piling

**time limit per test:** 2 seconds  
**memory limit per test:** 256 megabytes

You are given a rectangular board of `M × N` squares. Also you are given an unlimited number of standard domino pieces of `2 × 1` squares. You are allowed to rotate the pieces. You are asked to place as many dominoes as possible on the board so as to meet the following conditions:

1. Each domino completely covers two squares.
2. No two dominoes overlap.
3. Each domino lies entirely inside the board. It is allowed to touch the edges of the board.

Find the maximum number of dominoes, which can be placed under these restrictions.

## Input

In a single line you are given two integers `M` and `N` — board sizes in squares (`1 ≤ M ≤ N ≤ 16`).

## Output

Output one number — the maximal number of dominoes, which can be placed.

## Examples

Input
```bash
2 4
```

Output
```bash
4
```

Input
```bash
3 3
```

Output
```bash
4
```

---

# Enunciado (español)

## A. Colocación de dominós

**límite de tiempo por test:** 2 segundos  
**límite de memoria por test:** 256 megabytes

Se te da un tablero rectangular de `M × N` casillas. También se te da un número ilimitado de fichas de dominó estándar de `2 × 1` casillas. Se permite rotar las fichas. Debes colocar la mayor cantidad posible de dominós en el tablero cumpliendo las siguientes condiciones:

1. Cada dominó cubre completamente dos casillas.
2. Ningún par de dominós se solapa.
3. Cada dominó queda enteramente dentro del tablero. Se permite tocar los bordes del tablero.

Encuentra el número máximo de dominós que se pueden colocar bajo estas restricciones.

## Entrada

En una sola línea se te dan dos enteros `M` y `N` — los tamaños del tablero en casillas (`1 ≤ M ≤ N ≤ 16`).

## Salida

Imprime un número — el número máximo de dominós que se pueden colocar.

## Ejemplos

Input
```bash
2 4
```

Output
```bash
4
```

Input
```bash
3 3
```

Output
```bash
4
```

# Resumen

Dados dos enteros `M` y `N` (dimensiones del tablero, con `1 ≤ M ≤ N ≤ 16`), determina cuántas fichas de dominó de tamaño `2 × 1` (rotables) caben como máximo en el tablero sin solaparse y sin salir de él. Imprime ese número.
