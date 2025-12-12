use std::ops::{Range, RangeInclusive};

use anyhow::{Context, Result, anyhow};
use grid::Grid;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<()> {
    let (fresh_ranges, available_ids) = input.split_once("\n\n").ok_or_else(|| {
        anyhow!("expecting a break between fresh and available parts of database")
    })?;

    let mut p1_fresh_count = 0;

    let fresh_ranges: Vec<RangeInclusive<u64>> = fresh_ranges
        .lines()
        .map(|range| {
            let (low, high) = range
                .split_once('-')
                .context("expecting a range: `low-high`")?;

            Ok(low.parse()?..=high.parse()?)
        })
        .collect::<anyhow::Result<_>>()?;

    for id_text in available_ids.lines() {
        let id = id_text
            .parse::<u64>()
            .context("expecting a positive numeric id")?;

        if fresh_ranges.iter().any(|range| range.contains(&id)) {
            p1_fresh_count += 1;
        }
    }

    println!("Part 1: {p1_fresh_count}");

    Ok(())
}
