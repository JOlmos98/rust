---
name: competitive-enunciado-rs
description: >-
  Crea o actualiza enunciados Markdown en src/bin/statements/ para ejercicios
  de competitive programming en src/bin/*.rs. Usar cuando el usuario pida poner
  el enunciado, pegue un statement de Codeforces/AtCoder/etc., o indique
  documentar el problema junto a un binario.
---

# Enunciados en `src/bin/statements/` (competitive)

## Cuándo aplicar

Cuando el usuario entregue un enunciado (copy-paste bruto) y pida documentarlo para un ejercicio de `src/bin/`, o diga algo equivalente a “pon el enunciado”.

## Dónde y cómo

1. Crear/actualizar `src/bin/statements/<mismo_nombre_que_el_rs>.md`  
   Ejemplo: `src/bin/cf_0001_watermelon.rs` → `src/bin/statements/cf_0001_watermelon.md`
2. El `.md` es **Markdown puro** (sin comentario `/* */` de Rust).
3. Conservar el texto del enunciado; solo formatear (títulos, negritas, code, fences, callouts).
4. **No** dejar el enunciado dentro del `.rs`.
5. **No** resolver el problema en el Resumen ni dar pistas de solución.
6. No tocar la lógica del `main` salvo que el usuario lo pida.

## Índice (obligatorio al inicio)

Al **comienzo** del `.md`, antes del enunciado, poner un TOC clickable con anclas al estilo **VS Code / GitHub Flavored Markdown**:

1. Heading `## Índice`
2. Lista anidada con enlaces `[texto](#slug)` a **cada** heading del documento (`#` y `##`), excepto el propio `## Índice`
3. Separar el índice del contenido con `---`
4. Generar el `slug` así (como el preview de VS Code / github-slugger):
   - minúsculas
   - quitar puntuación (`.` `(` `)` `,` `:` etc.); **mantener** letras Unicode (`á`, `ñ`, …)
   - espacios → `-`
   - colapsar `-` repetidos

Ejemplos de anclas:

| Heading | Ancla |
|---------|--------|
| `# Statement` | `#statement` |
| `## A. Watermelon` | `#a-watermelon` |
| `# Enunciado (español)` | `#enunciado-español` |
| `## A. Sandía` | `#a-sandía` |
| `# Resumen` | `#resumen` |

## Formato Markdown obligatorio

| Elemento | Markdown |
|----------|----------|
| Índice | `## Índice` + lista de enlaces (ver arriba) |
| Título del problema | `## A. Nombre` |
| Límites de tiempo/memoria | `**time limit per test:** …` / `**memory limit per test:** …` (ES: `**límite de tiempo por test:**` / `**límite de memoria por test:**`) |
| Secciones | `## Input` / `## Output` / `## Examples` (ES: `## Entrada` / `## Salida` / `## Ejemplos`) |
| Variables y literales | `` `w` ``, `` `YES` ``, `` `NO` ``, rangos `` `1 ≤ w ≤ 100` `` |
| Ejemplos I/O | Etiquetas `Input` / `Output` + bloque ` ```bash ` … ` ``` ` |
| Nota del enunciado | Callout `> [!Note]` con cada línea prefijada por `>` |
| Separador EN/ES | `---` entre Statement (EN) y enunciado español |

## Plantilla obligatoria

```md
## Índice

- [Statement](#statement)
  - [A. Problem Name](#a-problem-name)
  - [Input](#input)
  - [Output](#output)
  - [Examples](#examples)
- [Enunciado (español)](#enunciado-español)
  - [A. Nombre traducido](#a-nombre-traducido)
  - [Entrada](#entrada)
  - [Salida](#salida)
  - [Ejemplos](#ejemplos)
- [Resumen](#resumen)

---

# Statement

## A. Problem Name

**time limit per test:** 1 second  
**memory limit per test:** 64 megabytes

<texto narrativo del enunciado; variables en `code`>

## Input

<descripción; rangos y variables en `code`>

## Output

<descripción; YES/NO u otros literales en `code`>

## Examples

Input
```bash
<entrada de ejemplo>
```

Output
```bash
<salida de ejemplo>
```

> [!Note]
> <nota del enunciado, si existe>

---

# Enunciado (español)

## A. Nombre traducido

**límite de tiempo por test:** 1 segundo  
**límite de memoria por test:** 64 megabytes

<misma estructura y contenido, traducido al español>

## Entrada

...

## Salida

...

## Ejemplos

Input
```bash
...
```

Output
```bash
...
```

> [!Note]
> <nota en español, si existe>

# Resumen

<1–3 frases claras: qué entrada hay y qué hay que decidir/devolver. Sin algoritmo ni tip de solución.>
```

## Reglas del contenido

| Sección | Reglas |
|---------|--------|
| **Statement** | Primera sección en inglés. Título exacto `# Statement`. Texto fiel al original, solo con formato Markdown. |
| **Enunciado (español)** | Misma información y misma estructura Markdown; títulos en español. Las etiquetas `Input`/`Output` se mantienen. |
| **Resumen** | Solo reformulación del *qué pide*. Prohibido: “basta con…”, condiciones equivalentes a la solución, o código de solución. |

## Ejemplo mínimo de Resumen (Watermelon)

Correcto: decidir si se puede partir el peso `w` en dos partes positivas de peso par e imprimir YES/NO.

Incorrecto: decir que `w` debe ser par y mayor que 2.
