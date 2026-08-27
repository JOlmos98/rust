## Índice

- [Enunciado (original)](#enunciado-original)
  - [A. Next Round](#a-next-round)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Siguiente ronda](#a-siguiente-ronda)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Enunciado (original)

## A. Next Round

**time limit per test:** 3 seconds  
**memory limit per test:** 256 megabytes

"Contestant who earns a score equal to or greater than the `k`-th place finisher's score will advance to the next round, as long as the contestant earns a positive score..." — an excerpt from contest rules.

A total of `n` participants took part in the contest (`n ≥ k`), and you already know their scores. Calculate how many participants will advance to the next round.

## Input

The first line of the input contains two integers `n` and `k` (`1 ≤ k ≤ n ≤ 50`) separated by a single space.

The second line contains `n` space-separated integers `a1, a2, ..., an` (`0 ≤ ai ≤ 100`), where `ai` is the score earned by the participant who got the `i`-th place. The given sequence is non-increasing (that is, for all `i` from `1` to `n − 1` the following condition is fulfilled: `ai ≥ ai+1`).

## Output

Output the number of participants who advance to the next round.

## Examples

Input
```bash
8 5
10 9 8 7 7 7 5 5
```

Output
```bash
6
```

Input
```bash
4 2
0 0 0 0
```

Output
```bash
0
```

> [!Note]
> In the first example the participant on the 5th place earned 7 points. As the participant on the 6th place also earned 7 points, there are 6 advancers.
>
> In the second example nobody got a positive score.

---

# Enunciado (español)

## A. Siguiente ronda

**límite de tiempo por test:** 3 segundos  
**límite de memoria por test:** 256 megabytes

"El concursante que obtenga una puntuación igual o mayor que la del finalista en el `k`-ésimo puesto avanzará a la siguiente ronda, siempre que obtenga una puntuación positiva..." — extracto de las reglas del concurso.

Un total de `n` participantes tomaron parte en el concurso (`n ≥ k`), y ya conoces sus puntuaciones. Calcula cuántos participantes avanzarán a la siguiente ronda.

## Entrada

La primera línea de la entrada contiene dos enteros `n` y `k` (`1 ≤ k ≤ n ≤ 50`) separados por un único espacio.

La segunda línea contiene `n` enteros separados por espacios `a1, a2, ..., an` (`0 ≤ ai ≤ 100`), donde `ai` es la puntuación obtenida por el participante que quedó en el `i`-ésimo puesto. La secuencia dada es no creciente (es decir, para todo `i` desde `1` hasta `n − 1` se cumple: `ai ≥ ai+1`).

## Salida

Imprime el número de participantes que avanzan a la siguiente ronda.

## Ejemplos

Input
```bash
8 5
10 9 8 7 7 7 5 5
```

Output
```bash
6
```

Input
```bash
4 2
0 0 0 0
```

Output
```bash
0
```

> [!Note]
> En el primer ejemplo, el participante en el 5.º puesto obtuvo 7 puntos. Como el del 6.º puesto también obtuvo 7 puntos, hay 6 clasificados.
>
> En el segundo ejemplo nadie obtuvo una puntuación positiva.

# Resumen

Dados `n`, `k` y las `n` puntuaciones en orden no creciente, determina cuántos participantes avanzan a la siguiente ronda: deben tener puntuación positiva y al menos tanta como la del puesto `k`.
