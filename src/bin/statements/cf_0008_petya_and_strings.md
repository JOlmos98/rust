## Índice

- [Statement](#statement)
  - [A. Petya and Strings](#a-petya-and-strings)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Petya y las cadenas](#a-petya-y-las-cadenas)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Petya and Strings

**time limit per test:** 2 seconds  
**memory limit per test:** 256 megabytes

Little Petya loves presents. His mum bought him two strings of the same size for his birthday. The strings consist of uppercase and lowercase Latin letters. Now Petya wants to compare those two strings lexicographically. The letters' case does not matter, that is an uppercase letter is considered equivalent to the corresponding lowercase letter. Help Petya perform the comparison.

## Input

Each of the first two lines contains a bought string. The strings' lengths range from `1` to `100` inclusive. It is guaranteed that the strings are of the same length and also consist of uppercase and lowercase Latin letters.

## Output

If the first string is less than the second one, print `"-1"`. If the second string is less than the first one, print `"1"`. If the strings are equal, print `"0"`. Note that the letters' case is not taken into consideration when the strings are compared.

## Examples

Input
```bash
aaaa
aaaA
```

Output
```bash
0
```

Input
```bash
abs
Abz
```

Output
```bash
-1
```

Input
```bash
abcdefg
AbCdEfF
```

Output
```bash
1
```

> [!Note]
> If you want more formal information about the lexicographical order (also known as the "dictionary order" or "alphabetical order"), you can visit the following site:
>
> http://en.wikipedia.org/wiki/Lexicographical_order

---

# Enunciado (español)

## A. Petya y las cadenas

**límite de tiempo por test:** 2 segundos  
**límite de memoria por test:** 256 megabytes

Al pequeño Petya le encantan los regalos. Su madre le compró dos cadenas del mismo tamaño para su cumpleaños. Las cadenas consisten en letras latinas mayúsculas y minúsculas. Ahora Petya quiere comparar esas dos cadenas lexicográficamente. El uso de mayúsculas o minúsculas no importa: una letra mayúscula se considera equivalente a la minúscula correspondiente. Ayuda a Petya a realizar la comparación.

## Entrada

Cada una de las dos primeras líneas contiene una de las cadenas compradas. La longitud de las cadenas está entre `1` y `100` inclusive. Se garantiza que las cadenas tienen la misma longitud y que consisten en letras latinas mayúsculas y minúsculas.

## Salida

Si la primera cadena es menor que la segunda, imprime `"-1"`. Si la segunda cadena es menor que la primera, imprime `"1"`. Si las cadenas son iguales, imprime `"0"`. Ten en cuenta que no se considera el uso de mayúsculas o minúsculas al comparar las cadenas.

## Ejemplos

Input
```bash
aaaa
aaaA
```

Output
```bash
0
```

Input
```bash
abs
Abz
```

Output
```bash
-1
```

Input
```bash
abcdefg
AbCdEfF
```

Output
```bash
1
```

> [!Note]
> Si quieres información más formal sobre el orden lexicográfico (también conocido como "orden de diccionario" u "orden alfabético"), puedes visitar el siguiente sitio:
>
> http://en.wikipedia.org/wiki/Lexicographical_order

# Resumen

Dadas dos cadenas de la misma longitud (entre 1 y 100, letras latinas mayúsculas/minúsculas), compáralas lexicográficamente ignorando mayúsculas y minúsculas. Imprime `-1` si la primera es menor, `1` si la segunda es menor, o `0` si son iguales.
