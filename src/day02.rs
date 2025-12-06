use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<()> {
    let mut p1_invalid_id_sum: usize = 0;
    let mut p2_invalid_id_sum: usize = 0;

    for id_range in input.split(',') {
        let (start, end) = id_range
            .split_once('-')
            .ok_or_else(|| anyhow!("expecting `lower-upper` format in `{id_range}`"))?;

        for id in start.parse::<usize>()?..=end.parse()? {
            let s = id.to_string();
            let digits = s.len();
            let halfway = digits / 2;

            for chunk_size in (1..=halfway).rev() {
                if s.chars()
                    .chunks(chunk_size)
                    .into_iter()
                    .skip(1)
                    .all(|mut chunk| chunk.join("") == s[0..chunk_size])
                {
                    if digits % 2 == 0 && chunk_size == halfway {
                        p1_invalid_id_sum += id;
                    }

                    p2_invalid_id_sum += id;
                    break;
                }
            }
        }
    }

    println!("Part 1: Invalid ID sum = {p1_invalid_id_sum}");
    println!("Part 2: Invalid ID sum = {p2_invalid_id_sum}");

    Ok(())
}
