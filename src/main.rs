use std::{env::args, fs::read_to_string, time::Instant};

use owo_colors::OwoColorize;

mod day01;
mod day02;
mod day03;

type DayFunc = fn(&str) -> anyhow::Result<()>;
type Day = (&'static str, DayFunc);

const DAYS: &[Day] = &[
    ("Secret Entrance", day01::run),
    ("Gift Shop", day02::run),
    ("Lobby", day03::run),
];

fn main() {
    if let Some(day_number) = args().nth(1).and_then(|it| it.parse::<usize>().ok()) {
        let Some(day) = DAYS.get(day_number - 1) else {
            println!("Day {day_number} not implemented!");
            return;
        };

        run_day(day_number, day);
    } else {
        let mut day_number = 1;

        for day in DAYS {
            run_day(day_number, day);
            day_number += 1;
        }
    }
}

fn run_day(day_number: usize, day: &Day) {
    let formatted_day_number = format!("{day_number:02}");
    println!(
        "{}",
        format!("-- Day {formatted_day_number}: {} --", day.0)
            .blue()
            .bold()
    );

    if let Ok(input) = read_to_string(format!("src/day{formatted_day_number}.txt")) {
        let start_time = Instant::now();

        match day.1(&input) {
            Ok(_) => println!("\n{}", "OK!".green()),
            Err(error) => println!("\n{} {error:?}", "Error:".red()),
        }

        println!("Took {:.2?}", start_time.elapsed());
    } else {
        println!("(Skipped: no input)");
    }

    println!();
}
