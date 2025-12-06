use std::{fs::read_to_string, time::Instant};

use owo_colors::OwoColorize;

mod day01;
mod day02;

type DayFunc = fn(&str) -> anyhow::Result<()>;
const DAYS: &[(&str, DayFunc)] = &[("Secret Entrance", day01::run), ("Gift Shop", day02::run)];

fn main() {
    let mut day_number = 1;

    for (title, day) in DAYS {
        let formatted_day_number = format!("{day_number:02}");
        println!(
            "{}",
            format!("-- Day {formatted_day_number}: {title} --")
                .blue()
                .bold()
        );

        if let Ok(input) = read_to_string(format!("src/day{formatted_day_number}.txt")) {
            let start_time = Instant::now();

            match day(&input) {
                Ok(_) => println!("\n{}", "OK!".green()),
                Err(error) => println!("\n{} {error:?}", "Error:".red()),
            }

            println!("Took {:.2?}", start_time.elapsed());
        } else {
            println!("(Skipped: no input)");
        }

        println!();
        day_number += 1;
    }
}
