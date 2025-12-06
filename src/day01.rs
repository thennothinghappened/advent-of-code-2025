use std::{thread::sleep, time::Duration};

use anyhow::Context;

pub fn run(input: &str) -> anyhow::Result<()> {
    let mut pos = 50;
    let mut finish_at_zero_count = 0;
    let mut passed_zero_count = 0;

    for line in input.lines() {
        let dir = if line.starts_with('R') { 1 } else { -1 };
        let magnitude = line[1..]
            .parse::<i32>()
            .with_context(|| format!("Invalid instruction magnitude for {line}"))?;

        let new_pos = pos + dir * magnitude;
        let wrap_count = new_pos.abs() / 100;

        passed_zero_count += wrap_count;

        // Check for crossing positive->negative or the other way.
        if pos != 0 && new_pos.signum() != pos.signum() {
            passed_zero_count += 1;
        }

        pos = new_pos.rem_euclid(100);

        if pos == 0 {
            finish_at_zero_count += 1;
        }
    }

    println!("Part 1:");
    println!("Dial hit zero {finish_at_zero_count} times");

    println!("Part 2:");
    println!("Dial passed through OR hit zero {passed_zero_count} times");

    Ok(())
}
