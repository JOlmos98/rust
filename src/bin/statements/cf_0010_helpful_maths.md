## Índice

- [Statement](#statement)
  - [A. Helpful Maths](#a-helpful-maths)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Matemáticas útiles](#a-matemáticas-útiles)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Helpful Maths

**time limit per test:** 2 seconds  
**memory limit per test:** 256 megabytes

Xenia the beginner mathematician is a third year student at elementary school. She is now learning the addition operation.

The teacher has written down the sum of multiple numbers. Pupils should calculate the sum. To make the calculation easier, the sum only contains numbers `1`, `2` and `3`. Still, that isn't enough for Xenia. She is only beginning to count, so she can calculate a sum only if the summands follow in non-decreasing order. For example, she can't calculate sum `1+3+2+1` but she can calculate sums `1+1+2` and `3+3`.

You've got the sum that was written on the board. Rearrange the summands and print the sum in such a way that Xenia can calculate the sum.

## Input

The first line contains a non-empty string `s` — the sum Xenia needs to count. String `s` contains no spaces. It only contains digits and characters `"+"`. Besides, string `s` is a correct sum of numbers `1`, `2` and `3`. String `s` is at most `100` characters long.

## Output

Print the new sum that Xenia can count.

## Examples

Input
```bash
3+2+1
```

Output
```bash
1+2+3
```

Input
```bash
1+1+3+1+3
```

Output
```bash
1+1+1+3+3
```

Input
```bash
2
```

Output
```bash
2
```

---

# Enunciado (español)

## A. Matemáticas útiles

**límite de tiempo por test:** 2 segundos  
**límite de memoria por test:** 256 megabytes

Xenia, matemática principiante, es estudiante de tercer año en la escuela primaria. Ahora está aprendiendo la operación de la suma.

La profesora ha escrito la suma de varios números. Los alumnos deben calcularla. Para facilitar el cálculo, la suma solo contiene los números `1`, `2` y `3`. Aun así, eso no le basta a Xenia. Como apenas empieza a contar, solo puede calcular una suma si los sumandos van en orden no decreciente. Por ejemplo, no puede calcular la suma `1+3+2+1`, pero sí puede calcular las sumas `1+1+2` y `3+3`.

Se te da la suma que estaba escrita en la pizarra. Reordena los sumandos e imprime la suma de forma que Xenia pueda calcularla.

## Entrada

La primera línea contiene una cadena no vacía `s` — la suma que Xenia necesita calcular. La cadena `s` no contiene espacios. Solo contiene dígitos y el carácter `"+"`. Además, la cadena `s` es una suma correcta de números `1`, `2` y `3`. La cadena `s` tiene como máximo `100` caracteres.

## Salida

Imprime la nueva suma que Xenia pueda calcular.

## Ejemplos

Input
```bash
3+2+1
```

Output
```bash
1+2+3
```

Input
```bash
1+1+3+1+3
```

Output
```bash
1+1+1+3+3
```

Input
```bash
2
```

Output
```bash
2
```

# Resumen

Dada una cadena `s` (suma correcta de números `1`, `2` y `3` unidos por `+`, sin espacios, ≤ 100 caracteres), reordena los sumandos en orden no decreciente y vuelve a imprimir la suma con el mismo formato.
