## Índice

- [Statement](#statement)
  - [A. Boy or Girl](#a-boy-or-girl)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Chico o chica](#a-chico-o-chica)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Boy or Girl

**time limit per test:** 1 second  
**memory limit per test:** 256 megabytes

Those days, many boys use beautiful girls' photos as avatars in forums. So it is pretty hard to tell the gender of a user at the first glance. Last year, our hero went to a forum and had a nice chat with a beauty (he thought so). After that they talked very often and eventually they became a couple in the network.

But yesterday, he came to see "her" in the real world and found out "she" is actually a very strong man! Our hero is very sad and he is too tired to love again now. So he came up with a way to recognize users' genders by their user names.

This is his method: if the number of distinct characters in one's user name is odd, then he is a male, otherwise she is a female. You are given the string that denotes the user name, please help our hero to determine the gender of this user by his method.

## Input

The first line contains a non-empty string, that contains only lowercase English letters — the user name. This string contains at most `100` letters.

## Output

If it is a female by our hero's method, print `"CHAT WITH HER!"` (without the quotes), otherwise, print `"IGNORE HIM!"` (without the quotes).

## Examples

Input
```bash
wjmzbmr
```

Output
```bash
CHAT WITH HER!
```

Input
```bash
xiaodao
```

Output
```bash
IGNORE HIM!
```

Input
```bash
sevenkplus
```

Output
```bash
CHAT WITH HER!
```

> [!Note]
> For the first example. There are 6 distinct characters in `"wjmzbmr"`. These characters are: `"w"`, `"j"`, `"m"`, `"z"`, `"b"`, `"r"`. So wjmzbmr is a female and you should print `"CHAT WITH HER!"`.

---

# Enunciado (español)

## A. Chico o chica

**límite de tiempo por test:** 1 segundo  
**límite de memoria por test:** 256 megabytes

En aquellos días, muchos chicos usaban fotos de chicas bonitas como avatares en los foros. Por eso es bastante difícil saber el género de un usuario a primera vista. El año pasado, nuestro héroe entró en un foro y tuvo una agradable charla con una belleza (o eso pensaba). Después hablaron muy a menudo y acabaron siendo pareja en la red.

Pero ayer fue a verla en el mundo real y descubrió que "ella" era en realidad un hombre muy fuerte. Nuestro héroe está muy triste y ahora está demasiado cansado para volver a enamorar. Así que ideó una forma de reconocer el género de los usuarios por sus nombres de usuario.

Este es su método: si el número de caracteres distintos en el nombre de usuario es impar, entonces es un hombre; en caso contrario, es una mujer. Se te da la cadena que denota el nombre de usuario; ayuda a nuestro héroe a determinar el género de este usuario según su método.

## Entrada

La primera línea contiene una cadena no vacía que solo contiene letras inglesas minúsculas — el nombre de usuario. Esta cadena contiene como máximo `100` letras.

## Salida

Si es una mujer según el método de nuestro héroe, imprime `"CHAT WITH HER!"` (sin las comillas); en caso contrario, imprime `"IGNORE HIM!"` (sin las comillas).

## Ejemplos

Input
```bash
wjmzbmr
```

Output
```bash
CHAT WITH HER!
```

Input
```bash
xiaodao
```

Output
```bash
IGNORE HIM!
```

Input
```bash
sevenkplus
```

Output
```bash
CHAT WITH HER!
```

> [!Note]
> Para el primer ejemplo. Hay 6 caracteres distintos en `"wjmzbmr"`. Esos caracteres son: `"w"`, `"j"`, `"m"`, `"z"`, `"b"`, `"r"`. Así que wjmzbmr es una mujer y debes imprimir `"CHAT WITH HER!"`.

# Resumen

Dado un nombre de usuario (cadena no vacía de como máximo 100 letras minúsculas inglesas), cuenta cuántos caracteres distintos tiene. Si esa cantidad es impar, imprime `IGNORE HIM!`; si es par, imprime `CHAT WITH HER!`.
