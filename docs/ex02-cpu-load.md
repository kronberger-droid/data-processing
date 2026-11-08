---
tags: [ex02, linux]
sheet: "[[exercises/Angaben01-02.pdf]]"
part: Beispiel 2
due: 2026-11-09
code: src/bin/ex02.rs
run: cargo run --bin ex02
---

# Ex02: CPU load counter

Every 5 seconds, until stopped with Ctrl-C, read `/proc/stat`, take the difference to the previous read for every `cpu` line and print it as percentages that sum to 100 % per line.

## `/proc/stat`

The `cpu` lines are cumulative tick counters since boot, one line summed over all CPUs and then one per CPU:

```text
cpu  20956078 3872580 6749463 290779321 598253 1674155 469185 0 0 0
cpu0  1847102  296217  740951  17002230  66627  218350 158086 0 0 0
```

The ten columns are the time spent in each mode: `user`, `nice` (niced user processes), `system`, `idle`, `iowait` (idle while waiting for I/O), `irq`, `softirq`, `steal` (taken by the hypervisor), `guest` and `guest_nice` (running a virtual CPU for a guest OS, printed as `gnice`).

## Percentages

The counters only grow, so the load over an interval is the difference of two snapshots. For mode $j$ of one line:

$$ Delta_j = n_j (t) - n_j (t - Delta t), quad p_j = 100 % dot Delta_j / (sum_(l=0)^9 Delta_l) $$

By construction $sum_j p_j = 100 %$. Every value is a ratio of ticks, so the tick rate `USER_HZ` cancels and never has to be known. A line with no ticks in the interval prints `-` instead of dividing by zero.

## Guest time

The kernel counts guest time twice: in `guest` and again in `user`, likewise `guest_nice` in `nice`. Summing the raw columns then overshoots whenever a VM runs, and every share comes out too small. The parser splits it off as `htop` does:

$$ "user" <- "user" - "guest", quad "nice" <- "nice" - "guest_nice" $$

Afterwards the ten columns are disjoint, every tick is counted once and the shares sum to exactly 100 %.

## Details in the code

- **Saturating differences.** The kernel documents that `iowait` can decrease, so the deltas use `saturating_sub` instead of panicking or wrapping.
- **CPU count check.** Snapshots with a different number of lines, or lines for different CPUs, are an error rather than wrong numbers.
- **No drift.** The loop sleeps until the next deadline, not for a fixed 5 s, so the time spent reading and printing does not accumulate.

## Output

```text
          user    nice  system    idle  iowait     irq softirq   steal   guest   gnice
cpu       3.9%    0.7%    1.1%   93.9%    0.0%    0.4%    0.1%    0.0%    0.0%    0.0%
cpu0      5.4%    1.6%    1.4%   91.1%    0.0%    0.4%    0.0%    0.0%    0.0%    0.0%
cpu1      5.4%    0.8%    1.0%   92.4%    0.0%    0.4%    0.0%    0.0%    0.0%    0.0%
...
```
