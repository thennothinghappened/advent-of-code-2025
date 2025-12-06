use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<()> {
    let mut invalid_id_sum: usize = 0;

    for id_range in input.split(',') {
        let (start, end) = id_range
            .split_once('-')
            .ok_or_else(|| anyhow!("expecting `lower-upper` format in `{id_range}`"))?;

        for id in start.parse::<usize>()?..=end.parse()? {
            let s = id.to_string();

            if s.len() % 2 != 0 {
                continue;
            }

            let (left, right) = s.split_at(s.len() / 2);

            if left != right {
                continue;
            }

            invalid_id_sum += id;
        }
    }

    println!("Part 1: Invalid ID sum = {invalid_id_sum}");

    Ok(())
}
