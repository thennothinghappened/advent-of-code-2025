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
    let mut grid = Grid::<FloorCell>::new(input.lines().count(), grid_width);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            grid[(y, x)] = match c {
                b'@' => FloorCell::Roll(0xff),
                b'.' => FloorCell::Empty,
                _ => unreachable!(),
            }
        }
    }

    let mut p1_accessible_count = 0;
    let mut p2_accessible_count = 0;

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            roll_calculate_access(&mut grid, x, y);

            if roll_is_accessible(&grid, x, y) {
                p1_accessible_count += 1;
            }
        }
    }

    // debug_print_grid(&grid);

    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            if roll_is_accessible(&grid, x, y) {
                p2_accessible_count += roll_remove_and_propagate(&mut grid, x, y);
            }
        }
    }

    println!("Part 1: {p1_accessible_count}");
    println!("Part 2: {p2_accessible_count}");

    Ok(())
}

fn roll_is_accessible(grid: &Grid<FloorCell>, x: usize, y: usize) -> bool {
    let Some(&FloorCell::Roll(neighbour_count)) = grid.get(y, x) else {
        return false;
    };

    assert!(neighbour_count <= 8);
    neighbour_count < 4
}

fn roll_calculate_access(grid: &mut Grid<FloorCell>, x: usize, y: usize) {
    if !matches!(grid.get(y, x), Some(FloorCell::Roll(_))) {
        return;
    };

    grid[(y, x)] = FloorCell::Roll(
        dirs_as_offsets(x, y)
            .filter(|&(x, y)| matches!(grid.get(y, x), Some(FloorCell::Roll(_))))
            .count() as u8,
    );
}

fn roll_remove_and_propagate(grid: &mut Grid<FloorCell>, x: usize, y: usize) -> usize {
    assert!(matches!(grid.get(y, x), Some(FloorCell::Roll(_))));

    let mut remove_count = 1;
    grid[(y, x)] = FloorCell::Empty;

    // debug_print_grid(grid);

    for (neighbour_x, neighbour_y) in dirs_as_offsets(x, y) {
        let mut should_remove = false;

        if let Some(FloorCell::Roll(neighbour_count)) = grid.get_mut(neighbour_y, neighbour_x) {
            *neighbour_count -= 1;

            if *neighbour_count < 4 {
                should_remove = true;
            }
        }

        if should_remove {
            remove_count += roll_remove_and_propagate(grid, neighbour_x, neighbour_y);
        }
    }

    remove_count
}

fn dirs_as_offsets(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    DIRS.iter().filter_map(move |(y_offset, x_offset)| {
        Some((
            (x as isize + x_offset).try_into().ok()?,
            (y as isize + y_offset).try_into().ok()?,
        ))
    })
}

// fn debug_print_grid(grid: &Grid<FloorCell>) {
//     for row in grid.iter_rows() {
//         println!(
//             "{}",
//             row.map(|cell| match *cell {
//                 FloorCell::Empty => '.',
//                 FloorCell::Roll(neighbour_count) => (neighbour_count + b'0').into(),
//             })
//             .join("")
//         );
//     }

//     println!();
// }

#[derive(Default, Clone, Copy)]
enum FloorCell {
    #[default]
    Empty,
    Roll(u8),
}
