use std::{fs, num::TryFromIntError};

#[derive(PartialEq, Eq, Debug)]
enum Cell {
    Roll,
    Empty,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let grid: Vec<Vec<Cell>> = input
        .trim()
        .split_whitespace()
        .map(|row_str| {
            row_str
                .chars()
                .map(|char| match char {
                    '@' => Cell::Roll,
                    '.' => Cell::Empty,
                    _ => panic!("invalid type"),
                })
                .collect()
        })
        .collect();

    println!("{:?}", grid);

    println!("Part 1:");
    println!("{}", part_1(&grid));
    println!("Part 2:");
    println!("{}", part_2(grid));
}

fn add_i32_to_usize(usize: usize, i32: i32) -> Result<usize, TryFromIntError> {
    let isize: i32 = usize.try_into().unwrap();
    (isize + i32).try_into()
}

fn part_1(grid: &Vec<Vec<Cell>>) -> i32 {
    let neighbors: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    grid.iter()
        .enumerate()
        .map(|(y, row)| -> i32 {
            row.iter()
                .enumerate()
                .map(|(x, cell)| -> i32 {
                    if *cell == Cell::Empty {
                        return 0;
                    };

                    let num_neighbors: usize = neighbors
                        .iter()
                        .map(|(delta_x, delta_y)| {
                            let neighbor_x: usize = add_i32_to_usize(x, *delta_x).ok()?;
                            let neighbor_y: usize = add_i32_to_usize(y, *delta_y).ok()?;
                            grid.get(neighbor_y).and_then(|row| row.get(neighbor_x))
                        })
                        .filter(|neighbor| match neighbor {
                            Some(Cell::Roll) => true,
                            _ => false,
                        })
                        .count()
                        .try_into()
                        .unwrap();

                    if num_neighbors < 4 { 1 } else { 0 }
                })
                .sum()
        })
        .sum()
}

fn part_2(mut grid: Vec<Vec<Cell>>) -> u64 {
    let neighbors: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut removed_any = true;
    let mut num_removed = 0;
    while removed_any {
        removed_any = false;

        let removal_candidates: Vec<(usize, usize)> = grid
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, cell)| {
                        if *cell == Cell::Empty {
                            return None;
                        };

                        let num_neighbors: usize = neighbors
                            .iter()
                            .map(|(delta_x, delta_y)| {
                                let neighbor_x: usize = add_i32_to_usize(x, *delta_x).ok()?;
                                let neighbor_y: usize = add_i32_to_usize(y, *delta_y).ok()?;
                                grid.get(neighbor_y).and_then(|row| row.get(neighbor_x))
                            })
                            .filter(|neighbor| match neighbor {
                                Some(Cell::Roll) => true,
                                _ => false,
                            })
                            .count()
                            .try_into()
                            .unwrap();

                        if num_neighbors < 4 {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Option<(usize, usize)>>>()
            })
            .flatten()
            .filter_map(|candiate| candiate)
            .collect();

        if removal_candidates.len() > 0 {
            removed_any = true;
        }

        removal_candidates.into_iter().for_each(|(x, y)| {
            grid[y][x] = Cell::Empty;
            num_removed += 1;
        });
    }

    num_removed
}
