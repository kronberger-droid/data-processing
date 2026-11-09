---
tags: [ex04, monte-carlo]
sheet: "[[exercises/Angaben04.pdf]]"
slides: "[[exercises/MC_int.pdf]]"
part: 2
due: 2026-11-23
code: src/bin/ex04/main.rs
---

# Ex04: Monte Carlo integration

## Mean value estimator

The integral of $f$ over the cube $B = [a, b]^d$ with volume $V = (b - a)^d$ equals $V$ times the mean of $f$ over $B$. Replacing that mean by an average over $n$ uniform samples $bold(x)_i in B$ gives

$$ I = integral_B f(bold(x)) dif bold(x) approx hat(I)_n = V / n sum_(i=1)^n f(bold(x)_i) $$

`integrate` computes this:

1. Set the sum to $0$.
2. Repeat for $i = 1, dots, n$: draw each of the $d$ coordinates as $a + (b - a) u$ with $u$ from `uni01`, and add $f(bold(x)_i)$ to the sum.
3. Return $V dot "sum" slash n$.

The coordinates are floats. Only the counters are integers.

## Error

The samples are independent, so with $sigma^2 = "Var"(f(bold(x)))$

$$ "Var"(hat(I)_n) = (V^2 sigma^2) / n, quad sigma_(hat(I)) = (V sigma) / sqrt(n) $$

The error falls as $1 slash sqrt(n)$ in every dimension, which is a slope of $-1 slash 2$ on a log-log plot. Ten times the accuracy costs a hundred times the samples.

## Compared with hit or miss

The slides draw a height $y_i in [y_0, y_1]$ as well and count a hit $t <- t + 1$ if $y_i <= f(bold(x)_i)$. That uses $d + 1$ random numbers per throw, and for $y_0 = 0$

$$ I approx V (y_1 - y_0) t / n $$

The slides write $I approx t slash n$, which assumes a box of volume 1. Both methods converge as $1 slash sqrt(n)$, but for $0 <= f <= y_1$ the variance of hit or miss is never smaller: it only keeps one yes or no bit where the mean value method uses $f(bold(x)_i)$ itself.

## The two uses in this exercise

Known integral, with $d = 1$:

$$ integral_0^pi sin x dif x = 2 $$

Volume of the $d$-dimensional unit ball, with $B = [-1, 1]^d$ and $f$ the indicator of $|bold(x)|^2 <= 1$. Here $f$ is 0 or 1, so the sum counts hits and both methods coincide. The analytic value follows from

$$ V_d = (2 pi) / d V_(d-2), quad V_0 = 1, quad V_1 = 2 $$

The share of hits $V_d slash 2^d$ drops quickly, about $3.3 dot 10^(-4)$ at $d = 12$. The relative error is about $1 slash sqrt("hits")$, so $n = 10^6$ gives only about 330 hits and 5.5 % there.
