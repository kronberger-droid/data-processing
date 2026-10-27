use std::{
    fmt, fs,
    ops::Index,
    str::FromStr,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, anyhow, bail, ensure};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum StatKind {
    Total,
    Cpu(usize),
}

impl fmt::Display for StatKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatKind::Total => f.pad("cpu"),
            StatKind::Cpu(n) => f.pad(&format!("cpu{n}")),
        }
    }
}

enum Mode {
    User,
    Nice,
    System,
    Idle,
    Iowait,
    Irq,
    Softirq,
    Steal,
    Guest,
    GuestNice,
}

impl Mode {
    pub const ALL: [Mode; 10] = [
        Mode::User,
        Mode::Nice,
        Mode::System,
        Mode::Idle,
        Mode::Iowait,
        Mode::Irq,
        Mode::Softirq,
        Mode::Steal,
        Mode::Guest,
        Mode::GuestNice,
    ];
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match self {
            Mode::User => "user",
            Mode::Nice => "nice",
            Mode::System => "system",
            Mode::Idle => "idle",
            Mode::Iowait => "iowait",
            Mode::Irq => "irq",
            Mode::Softirq => "softirq",
            Mode::Steal => "steal",
            Mode::Guest => "guest",
            Mode::GuestNice => "gnice",
        })
    }
}

struct CpuStat {
    kind: StatKind,
    stats: [usize; 10],
}

impl CpuStat {
    fn delta(&self, prev: &Self) -> Result<Self> {
        (self.kind == prev.kind)
            .then(|| CpuStat {
                kind: self.kind,
                stats: std::array::from_fn(|i| self.stats[i].saturating_sub(prev.stats[i])),
            })
            .ok_or_else(|| anyhow!("cpu mismatch"))
    }

    fn total(&self) -> usize {
        self.stats.iter().sum()
    }
}

struct Snapshot(Vec<CpuStat>);

impl Snapshot {
    fn cpus(&self) -> &[CpuStat] {
        &self.0
    }

    fn read() -> Result<Self> {
        fs::read_to_string("/proc/stat")?.parse()
    }

    fn delta(&self, prev: &Self) -> Result<Delta> {
        ensure!(
            self.cpus().len() == prev.cpus().len(),
            "cpu count changed: {} vs {}",
            prev.cpus().len(),
            self.cpus().len()
        );
        self.into_iter()
            .zip(prev)
            .map(|(cur, prev)| cur.delta(prev))
            .collect::<Result<Snapshot>>()
            .map(Delta)
    }
}

impl FromStr for Snapshot {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .filter(|l| l.starts_with("cpu"))
            .map(CpuStat::from_str)
            .collect::<Result<Snapshot, _>>()
    }
}

impl FromIterator<CpuStat> for Snapshot {
    fn from_iter<T: IntoIterator<Item = CpuStat>>(iter: T) -> Self {
        let mut result = Vec::new();
        for stat in iter {
            result.push(stat);
        }
        Snapshot(result)
    }
}

impl<'a> IntoIterator for &'a Snapshot {
    type Item = &'a CpuStat;
    type IntoIter = std::slice::Iter<'a, CpuStat>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Counters accumulated over one interval, as opposed to since boot.
struct Delta(Snapshot);

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<6}", "")?;
        for m in Mode::ALL {
            write!(f, "{m:>8}")?;
        }
        for cpu in &self.0 {
            write!(f, "\n{:<6}", cpu.kind)?;
            let total = cpu.total();
            for m in Mode::ALL {
                if total == 0 {
                    write!(f, "{:>8}", "-")?;
                } else {
                    let pct = 100.0 * cpu[m] as f64 / total as f64;
                    write!(f, "{pct:>7.1}%")?;
                }
            }
        }
        Ok(())
    }
}

impl Index<Mode> for CpuStat {
    type Output = usize;
    fn index(&self, m: Mode) -> &Self::Output {
        &self.stats[m as usize]
    }
}

impl FromStr for CpuStat {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, cols) = s
            .split_once(' ')
            .with_context(|| format!("no columns: {s:?}"))?;

        let n = name
            .strip_prefix("cpu")
            .with_context(|| format!("not a cpu line: {s:?}"))?;

        let kind = if n.is_empty() {
            StatKind::Total
        } else {
            StatKind::Cpu(n.parse()?)
        };

        let numbers = cols
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()?;

        let stats: [usize; 10] = numbers
            .try_into()
            .map_err(|v: Vec<usize>| anyhow!("expected 10 columns, found {}", v.len()))?;

        Ok(CpuStat { kind, stats })
    }
}

fn main() -> Result<()> {
    let interval = Duration::from_secs(5);
    let mut prev = Snapshot::read()?;
    let mut next = Instant::now();
    loop {
        next += interval;
        thread::sleep(next.saturating_duration_since(Instant::now()));
        let current = Snapshot::read()?;
        println!("{}\n", current.delta(&prev)?);
        prev = current;
    }
}
