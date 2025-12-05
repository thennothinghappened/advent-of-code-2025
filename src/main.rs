use std::fs::read_to_string;

mod day01;

fn main() {
    println!("-- Day 01 --");
    day01::run(&read_to_string("src/day01.txt").unwrap());
}
