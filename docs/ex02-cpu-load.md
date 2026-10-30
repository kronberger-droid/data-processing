---
tags: [ex02, linux]
---

# Ex02: CPU load counter

Sheet: `exercises/Angaben01-02.pdf`, Beispiel 2, due 2026-11-09. Code: `src/bin/ex02.rs`. Run with `cargo run --bin ex02`, stop with Ctrl-C.

Every 5 seconds the program reads `/proc/stat` and prints, per CPU, the share of the last interval spent in each mode.

## `/proc/stat`

The lines starting with `cpu` hold cumulative counters since boot: one `cpu` line summed over all CPUs, then `cpu0`, `cpu1`, … one per CPU.

```text
cpu  20956078 3872580 6749463 290779321 598253 1674155 469185 0 0 0
cpu0 1847102 296217 740951 17002230 66627 218350 158086 0 0 0
cpu1 1251021 256243 360396 18335766 21044 79780 31139 0 0 0
```

The ten columns are time spent in each mode, in ticks of `USER_HZ` (usually 1/100 s):

| # | Mode | Meaning |
|---|---|---|
| 0 | `user` | normal processes in user mode |
| 1 | `nice` | niced processes in user mode |
| 2 | `system` | kernel mode |
| 3 | `idle` | idle |
| 4 | `iowait` | idle while waiting for I/O |
| 5 | `irq` | servicing interrupts |
| 6 | `softirq` | servicing softirqs |
| 7 | `steal` | time a hypervisor gave to other VMs |
| 8 | `guest` | running a virtual CPU for a guest OS |
| 9 | `guest_nice` | running a niced guest |

The code mirrors this with `Mode` as the column index and `Index<Mode>` on `CpuStat`, so `cpu[Mode::Idle]` reads column 3.

## Percentages

The counters only ever grow, so the load over an interval comes from the difference between two snapshots. For mode $j$ of one CPU line:

$$ Delta_j = n_j (t) - n_j (t - Delta t) $$

$$ p_j = 100 % dot Delta_j / (sum_(l=0)^9 Delta_l) $$

By construction $sum_j p_j = 100 %$ per line, as the sheet asks. Since every value is a ratio of ticks, `USER_HZ` cancels out and never needs to be known. A line whose total is $0$, a CPU with no ticks at all in the interval, prints `-` instead of dividing by zero.

## Details in the code

- **Saturating difference.** The deltas use `saturating_sub`. The kernel documents that `iowait` can decrease in some conditions, and a plain `usize` subtraction would then panic in debug builds or wrap to a huge value in release.
- **CPU count check.** `Snapshot::delta` refuses to compare snapshots with different numbers of CPU lines, and `CpuStat::delta` refuses to pair up different CPUs, so a CPU going offline between reads gives an error instead of wrong numbers.
- **No drift.** The loop sleeps until `next += interval` rather than for a fixed 5 s. Sleeping a fixed time would add the time spent reading and printing on top of each interval and slowly drift; sleeping until a deadline keeps the intervals at 5 s on average.

## Caveat: guest time is counted twice

The kernel adds guest time to `user` as well as to `guest`, and guest nice time to both `nice` and `guest_nice`. Summing all ten columns therefore counts it twice. On a machine not running VMs both guest columns stay $0$ and the percentages are exact. With VMs running, the total is too large by the guest time, and every mode's share comes out too small. `htop` avoids this by subtracting `guest` from `user` and `guest_nice` from `nice` before summing.
