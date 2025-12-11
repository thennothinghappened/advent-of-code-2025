use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<()> {
    let banks = input.lines().map(|line| line.as_bytes());

    let mut p1_jolts: u64 = 0;
    let mut p2_jolts: u64 = 0;

    for bank in banks {
        // Part 1: 2-digit number.
        p1_jolts += bank_calculate_joltage(bank, 1);

        // Part 2: 12-digit number.
        p2_jolts += bank_calculate_joltage(bank, 11);
    }

    println!("Part 1: {p1_jolts}");
    println!("Part 2: {p2_jolts}");

    Ok(())
}

fn bank_calculate_joltage(bank: &[u8], digits: u32) -> u64 {
    let mut best_index = 0;
    let mut best_digit = 0;

    for (index, &digit) in bank.iter().dropping_back(digits as usize).enumerate() {
        if digit > best_digit {
            best_index = index;
            best_digit = digit;
        }
    }

    best_digit -= b'0';

    if digits == 0 {
        return best_digit as u64;
    }

    (best_digit as u64) * 10_u64.pow(digits)
        + bank_calculate_joltage(&bank[best_index + 1..], digits - 1)
}
