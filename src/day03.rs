use anyhow::{Context, Result, anyhow};
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<()> {
    let banks = input.lines();

    let p1_jolts = banks
        .map(|bank| {
            let mut tens_index = 0;
            let mut tens_digit = 0;

            for (index, digit) in bank.bytes().dropping_back(1).map(|b| b - b'0').enumerate() {
                if digit > tens_digit {
                    tens_index = index;
                    tens_digit = digit;
                }
            }

            let mut ones_digit = 0;

            for digit in bank.bytes().dropping(tens_index + 1).map(|b| b - b'0') {
                if digit > ones_digit {
                    ones_digit = digit;
                }
            }

            ((tens_digit * 10) + ones_digit) as u32
        })
        .sum::<u32>();

    println!("Part 1: {p1_jolts}");

    Ok(())
}
