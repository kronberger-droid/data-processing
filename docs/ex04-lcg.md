---
tags: [ex04, random-numbers]
sheet: "[[exercises/Angaben04.pdf]]"
part: 1
due: 2026-11-23
code: src/lcg.rs
---

# Ex04: Linear congruential generator

## Recurrence

$$ y_(k+1) = (a y_k + c) mod m $$

with multiplier $a$, increment $c$, modulus $m$ and seed $y_1$. Every state lies in $\{0, dots, m - 1\}$, so the sequence repeats after at most $m$ steps.

| Name | $a$ | $c$ | $m$ |
|---|---|---|---|
| ANSI C | $1103515245$ | $12345$ | $2^31$ |
| RANDU | $65539 = 2^16 + 3$ | $0$ | $2^31$ |

## Full period (Hull–Dobell)

With $c != 0$ the period is the full $m$ exactly when

1. $c$ and $m$ are coprime,
2. $a - 1$ is divisible by every prime factor of $m$,
3. $a - 1$ is divisible by $4$ if $m$ is.

For $m = 2^31$ this means $c$ odd and $a equiv 1 mod 4$. Both hold for the ANSI C parameters. The small test generator $a = 5$, $c = 1$, $m = 16$ satisfies them too and visits all 16 residues.

RANDU has $c = 0$, so state $0$ is a fixed point and the period is at most $2^29$.

## Scaling

$$ "uni01" = y / m in [0, 1) $$

$$ "uni"(i_max) = floor("uni01" dot (i_max + 1)) in \{0, dots, i_max\} $$

The obvious alternative $y mod (i_max + 1)$ uses the low bits of $y$. For $m = 2^e$ the lowest $j$ bits have period at most $2^j$: the lowest bit just alternates. Scaling uses the high bits, which have the long period.

## Why RANDU fails the spectral test

Square the multiplier:

$$ a^2 = (2^16 + 3)^2 = 2^32 + 6 dot 2^16 + 9 equiv 6 dot 2^16 + 9 = 6 a - 9 quad (mod 2^31) $$

so every three consecutive values satisfy

$$ y_(k+2) = 6 y_(k+1) - 9 y_k quad (mod 2^31) $$

Divided by $m$, the triplets $(x_k, x_(k+1), x_(k+2))$ satisfy $9 x_k - 6 x_(k+1) + x_(k+2) in ZZ$. Inside the unit cube that integer can only take the values $-5, dots, 9$, so all points lie on 15 parallel planes. The 3D plot of the triplets shows exactly those planes.
