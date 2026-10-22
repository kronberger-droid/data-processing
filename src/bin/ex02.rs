use std::{fs, str::FromStr};

use anyhow::{Context, Result, anyhow, bail};

enum StatKind {
    Total,
    Cpu(usize),
}

struct CpuStat {
    kind: StatKind,
    stats: [usize; 10],
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
    let cpus: Vec<CpuStat> = fs::read_to_string("/proc/stat")?
        .lines()
        .filter(|l| l.starts_with("cpu"))
        .map(CpuStat::from_str)
        .collect::<Result<Vec<CpuStat>, _>>()?;

    Ok(())
}
