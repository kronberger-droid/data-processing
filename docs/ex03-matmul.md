---
tags: [ex03, linear-algebra]
---

# Ex03: Matrix multiplication

Sheet: `exercises/Angaben03.pdf`, due 2026-11-16.

## Definition

For $A in RR^(n times m)$ and $B in RR^(m times p)$ the product $C = A B in RR^(n times p)$ is

$$ c_(i j) = sum_(k=1)^m a_(i k) b_(k j), quad i = 1, dots, n, quad j = 1, dots, p $$

Entry $c_(i j)$ is the dot product of row $i$ of $A$ with column $j$ of $B$. The inner dimension $m$ has to match.

## Cost

$C$ has $n p$ entries and each needs $m$ multiply-adds, so one product costs $n m p$ operations, $cal(O)(n^3)$ for square matrices.

For the exercise ($n = 200$, 10 products):

$$ 10 dot 200^3 = 8 dot 10^7 "multiply-adds" $$

## Row view and loop order

Pulling the sum apart by rows gives a second reading of the same formula. With $bold(c)_i$ and $bold(b)_k$ the rows of $C$ and $B$:

$$ bold(c)_i = sum_(k=1)^m a_(i k) bold(b)_k $$

Row $i$ of $C$ is a linear combination of the rows of $B$, weighted by row $i$ of $A$. Both readings sum the same terms, only in a different order, so the result is identical up to floating-point rounding.

That is what makes the `i-k-j` loop order legal:

```rust
for i in 0..n {
    for k in 0..m {
        let a_ik = a[i * m + k];
        for j in 0..p {
            c[i * p + j] += a_ik * b[k * p + j];
        }
    }
}
```

The inner loop walks along a row of $B$ and a row of $C$, both contiguous in memory. The textbook `i-j-k` order walks down a column of $B$ instead, jumping $p$ elements per step and missing the cache.

## Memory layout

Store a matrix as one flat `Vec<f64>` in row-major order. Zero-based entry $(i, j)$ of an $n times p$ matrix sits at index

$$ i p + j $$

## Algebraic properties

- Associative: $(A B) C = A (B C)$
- Not commutative: $A B != B A$ in general
- Distributive: $A (B + C) = A B + A C$

Because of non-commutativity, a chain $A_1 A_2 dots A_10$ can only be split into contiguous groups whose partial products are multiplied back together in order. The sheet avoids this: the input is a list of independent pairs, and each worker computes complete products $C^((r)) = A^((r)) B^((r))$ for its share of the list.

## Parallel split

With $N$ products and $P$ workers, each worker gets about $N slash P$ products. The ideal speedup is

$$ S = T_1 / T_P <= min(P, N) $$

where $T_P$ is the runtime with $P$ workers. With $N = 10$, more than 10 workers cannot help. Overhead pushes $S$ below the bound: thread or process startup, and for processes also sending the matrices to the worker and the result back.
