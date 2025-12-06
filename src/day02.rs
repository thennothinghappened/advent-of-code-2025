use std::collections::HashSet;

use anyhow::{Context, Result, anyhow};
use itertools::Itertools;
use owo_colors::OwoColorize;

pub fn run(input: &str) -> anyhow::Result<()> {
    let mut p1_invalid_id_sum: u64 = 0;
    let mut p2_invalid_id_sum: u64 = 0;

    let mut p1_invalid_id_sum_ctrl: usize = 0;
    let mut p2_invalid_id_sum_ctrl: usize = 0;

    for id_range in input.split(',') {
        let (start, end) = id_range
            .split_once('-')
            .ok_or_else(|| anyhow!("expecting `lower-upper` format in `{id_range}`"))?;

        let start_num = start.parse::<u64>()?;
        let start_digits = start.len();

        let end_num = end.parse::<u64>()?;
        let end_digits = end.len();

        let range = end_num - start_num;
        let mut forbidden = HashSet::<u64>::new();

        println!("\n{}", format!("-- {start_num}-{end_num} --").bold());

        if start_digits == end_digits {
            for chunk_size in (1..=((start_digits + 1) / 2)).rev() {
                println!("\n--> Trying chunk size {chunk_size}");

                let sample_chunk = start[0..chunk_size].parse::<u64>()?;
                let num_chunks = start_digits / chunk_size;

                if dbg!(chunk_size * num_chunks) != dbg!(start_digits) {
                    println!("Discarding - wrong length");
                    continue;
                }

                let mut desired_value: u64 = 0;

                for _ in 0..num_chunks {
                    desired_value *= 10_u64.pow(chunk_size as u32);
                    desired_value += sample_chunk;
                }

                println!("desired value: {desired_value}");
                assert_eq!(desired_value.to_string().len(), start_digits);

                if (start_num..=end_num).contains(&desired_value)
                    && !forbidden.contains(&desired_value)
                {
                    println!("{}\n", format!("{desired_value} matches!").green());

                    if chunk_size == start_digits.div_ceil(2) && start_digits.is_multiple_of(2) {
                        println!("exp:: p1_invalid_id_sum += {desired_value}");
                        p1_invalid_id_sum += desired_value;
                    }

                    // println!("exp:: p2_invalid_id_sum ({p2_invalid_id_sum}) += {desired_value}");
                    p2_invalid_id_sum += dbg!(desired_value);
                    forbidden.insert(desired_value);

                    break;
                }
            }

            for chunk_size in (1..=((end_digits + 1) / 2)).rev() {
                println!("\n--> Trying chunk size {chunk_size}");

                let sample_chunk = end[0..chunk_size].parse::<u64>()?;
                let num_chunks = end_digits / chunk_size;

                if dbg!(chunk_size * num_chunks) != dbg!(end_digits) {
                    println!("Discarding - wrong length");
                    continue;
                }

                let mut desired_value: u64 = 0;

                for _ in 0..num_chunks {
                    desired_value *= 10_u64.pow(chunk_size as u32);
                    desired_value += sample_chunk;
                }

                println!("desired value: {desired_value}");
                assert_eq!(desired_value.to_string().len(), end_digits);

                if (start_num..=end_num).contains(&desired_value)
                    && !forbidden.contains(&desired_value)
                {
                    println!("{}\n", format!("{desired_value} matches!").green());

                    if chunk_size == start_digits.div_ceil(2) && end_digits.is_multiple_of(2) {
                        println!("exp:: p1_invalid_id_sum += {desired_value}");
                        p1_invalid_id_sum += desired_value;
                    }

                    // println!("exp:: p2_invalid_id_sum ({p2_invalid_id_sum}) += {desired_value}");
                    p2_invalid_id_sum += dbg!(desired_value);
                    forbidden.insert(desired_value);

                    break;
                }
            }
        } else {
            for id in start_num..=end_num {
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
                            println!("EXP FALLBACK:: p1_invalid_id_sum += {id}");
                            p1_invalid_id_sum += id;
                        }

                        // println!("EXP FALLBACK:: p2_invalid_id_sum ({p2_invalid_id_sum}) += {id}");
                        p2_invalid_id_sum += id;
                        break;
                    }
                }
            }
        }

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
                        println!("CONTROL:: p1_invalid_id_sum_ctrl += {id}");
                        p1_invalid_id_sum_ctrl += id;
                    }

                    // println!("CONTROL:: p2_invalid_id_sum_ctrl ({p2_invalid_id_sum_ctrl}) += {id}");
                    p2_invalid_id_sum_ctrl += id;
                    break;
                }
            }
        }

        assert_eq!(p1_invalid_id_sum, p1_invalid_id_sum_ctrl as u64);
        // assert_eq!(p2_invalid_id_sum, p2_invalid_id_sum_ctrl as u64);

        println!("\n{}\n", "OK.".bold());
    }

    println!("Part 1: Invalid ID sum = {p1_invalid_id_sum}");
    println!("Part 2: Invalid ID sum = {p2_invalid_id_sum}");

    Ok(())
}
