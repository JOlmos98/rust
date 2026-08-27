## Índice

- [Statement](#statement)
  - [A. Way Too Long Words](#a-way-too-long-words)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Palabras demasiado largas](#a-palabras-demasiado-largas)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Way Too Long Words

**time limit per test:** 1 second  
**memory limit per test:** 256 megabytes

Sometimes some words like `"localization"` or `"internationalization"` are so long that writing them many times in one text is quite tiresome.

Let's consider a word too long, if its length is strictly more than `10` characters. All too long words should be replaced with a special abbreviation.

This abbreviation is made like this: we write down the first and the last letter of a word and between them we write the number of letters between the first and the last letters. That number is in decimal system and doesn't contain any leading zeroes.

Thus, `"localization"` will be spelt as `"l10n"`, and `"internationalization"` will be spelt as `"i18n"`.

You are suggested to automatize the process of changing the words with abbreviations. At that all too long words should be replaced by the abbreviation and the words that are not too long should not undergo any changes.

## Input

The first line contains an integer `n` (`1 ≤ n ≤ 100`). Each of the following `n` lines contains one word. All the words consist of lowercase Latin letters and possess the lengths of from `1` to `100` characters.

## Output

Print `n` lines. The `i`-th line should contain the result of replacing of the `i`-th word from the input data.

## Examples

InputCopy
```bash
4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis
```

OutputCopy
```bash
word
l10n
i18n
p43s
```

---

# Enunciado (español)

## A. Palabras demasiado largas

**límite de tiempo por test:** 1 segundo  
**límite de memoria por test:** 256 megabytes

A veces algunas palabras como `"localization"` o `"internationalization"` son tan largas que escribirlas muchas veces en un texto resulta bastante cansado.

Consideramos que una palabra es demasiado larga si su longitud es estrictamente mayor que `10` caracteres. Todas las palabras demasiado largas deben reemplazarse por una abreviatura especial.

Esta abreviatura se forma así: escribimos la primera y la última letra de la palabra y, entre ellas, el número de letras que hay entre la primera y la última. Ese número está en sistema decimal y no contiene ceros a la izquierda.

Así, `"localization"` se escribirá como `"l10n"`, e `"internationalization"` como `"i18n"`.

Se te sugiere automatizar el proceso de cambiar las palabras por abreviaturas. Todas las palabras demasiado largas deben reemplazarse por la abreviatura y las que no lo son no deben sufrir ningún cambio.

## Entrada

La primera línea contiene un entero `n` (`1 ≤ n ≤ 100`). Cada una de las siguientes `n` líneas contiene una palabra. Todas las palabras consisten en letras latinas minúsculas y tienen longitudes de `1` a `100` caracteres.

## Salida

Imprime `n` líneas. La línea `i`-ésima debe contener el resultado de reemplazar la `i`-ésima palabra de los datos de entrada.

## Ejemplos

Input
```bash
4
word
localization
internationalization
pneumonoultramicroscopicsilicovolcanoconiosis
```

Output
```bash
word
l10n
i18n
p43s
```

# Resumen

Dado un entero `n` y luego `n` palabras, para cada palabra imprime la misma palabra si su longitud no es estrictamente mayor que `10`, o bien la abreviatura formada por la primera letra, el número de letras intermedias y la última letra si lo es.



Es decir, metemos un 2 y después dos palabras como "hola" y "abreviatura" y tiene que imprimir:

hola
a9s

a9s porque la longitud total es 11, mayor que 10, aunque la suma de las intermedias no lo sean.