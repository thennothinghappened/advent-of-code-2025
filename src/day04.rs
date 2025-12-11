use anyhow::{Context, Result, anyhow};
use grid::Grid;
use itertools::Itertools;

#[rustfmt::skip]
const DIRS: &[(isize, isize)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1),
];

pub fn run(input: &str) -> anyhow::Result<()> {
    let grid_width = input.find('\n').unwrap();
    let mut grid = Grid::<u8>::new(input.lines().count(), grid_width);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            grid[(y, x)] = c;
        }
    }

    let mut p1_accessible_count = 0;

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if grid[(y, x)] != b'@' {
                continue;
            }

            if DIRS
                .iter()
                .filter(|&&(y_offset, x_offset)| {
                    matches!(
                        grid.get(y as isize + y_offset, x as isize + x_offset),
                        Some(b'@')
                    )
                })
                .count()
                < 4
            {
                p1_accessible_count += 1;
                // println!("({x}, {y}) OK");
            }
        }
    }

    println!("Part 1: {p1_accessible_count}");
    // println!("{grid:#?}");

    Ok(())
}
