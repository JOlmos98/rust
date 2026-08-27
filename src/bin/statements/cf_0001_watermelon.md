## Índice

- [Statement](#statement)
  - [A. Watermelon](#a-watermelon)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Sandía](#a-sandía)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Watermelon

**time limit per test:** 1 second  
**memory limit per test:** 64 megabytes

One hot summer day Pete and his friend Billy decided to buy a watermelon. They chose the biggest and the ripest one, in their opinion. After that the watermelon was weighed, and the scales showed `w` kilos. They rushed home, dying of thirst, and decided to divide the berry, however they faced a hard problem.

Pete and Billy are great fans of even numbers, that's why they want to divide the watermelon in such a way that each of the two parts weighs even number of kilos, at the same time it is not obligatory that the parts are equal. The boys are extremely tired and want to start their meal as soon as possible, that's why you should help them and find out, if they can divide the watermelon in the way they want. For sure, each of them should get a part of positive weight.

## Input

The first (and the only) input line contains integer number `w` (`1 ≤ w ≤ 100`) — the weight of the watermelon bought by the boys.

## Output

Print `YES`, if the boys can divide the watermelon into two parts, each of them weighing even number of kilos; and `NO` in the opposite case.

## Examples

InputCopy
```bash
8
```

OutputCopy
```bash
YES
```

> [!Note]
> For example, the boys can divide the watermelon into two parts of 2 and 6 kilos respectively (another variant — two parts of 4 and 4 kilos).

---

# Enunciado (español)

## A. Sandía

**límite de tiempo por test:** 1 segundo  
**límite de memoria por test:** 64 megabytes

Un día caluroso de verano, Pete y su amigo Billy decidieron comprar una sandía. Eligieron la más grande y madura, según su opinión. Después la sandía fue pesada y la báscula mostró `w` kilos. Corrieron a casa, muriendo de sed, y decidieron dividir la fruta; sin embargo, se enfrentaron a un problema difícil.

Pete y Billy son grandes fans de los números pares, por eso quieren dividir la sandía de tal forma que cada una de las dos partes pese un número par de kilos; al mismo tiempo, no es obligatorio que las partes sean iguales. Los chicos están extremadamente cansados y quieren empezar a comer lo antes posible, por eso debes ayudarles y averiguar si pueden dividir la sandía de la forma que quieren. Por supuesto, cada uno debe recibir una parte de peso positivo.

## Entrada

La primera (y única) línea de entrada contiene el número entero `w` (`1 ≤ w ≤ 100`) — el peso de la sandía comprada por los chicos.

## Salida

Imprime `YES` si los chicos pueden dividir la sandía en dos partes, cada una con un peso par de kilos; e imprime `NO` en el caso contrario.

## Ejemplos

Input
```bash
8
```

Output
```bash
YES
```

> [!Note]
> Por ejemplo, los chicos pueden dividir la sandía en dos partes de 2 y 6 kilos respectivamente (otra variante — dos partes de 4 y 4 kilos).

# Resumen

Dado un peso entero `w` (entre 1 y 100, input por consola de ese peso), decide si es posible partir la sandía en exactamente dos trozos, ambos con peso positivo y ambos con peso par (no hace falta que sean iguales). Imprime `YES` si se puede y `NO` si no.
