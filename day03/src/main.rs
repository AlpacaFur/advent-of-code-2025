use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let banks: Vec<Vec<u32>> = input
        .trim()
        .split_whitespace()
        .map(|bank| {
            bank.chars()
                .map(|char| char.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    println!("Part 1:");
    println!("{}", part_1(&banks));
    println!("Part 2:");
    println!("{}", part_2(&banks));
}

fn part_1(banks: &Vec<Vec<u32>>) -> u32 {
    banks
        .iter()
        .map(|bank| {
            let last_index = bank.len() - 1;
            let mut max_so_far = bank[0];
            let mut next_biggest = bank[1];
            bank[1..last_index].iter().for_each(|battery| {
                if *battery > max_so_far {
                    max_so_far = *battery;
                    next_biggest = 0;
                } else {
                    next_biggest = next_biggest.max(*battery)
                }
            });

            next_biggest = next_biggest.max(bank[last_index]);
            max_so_far * 10 + next_biggest
        })
        .sum()
}

const SEQUENCE_LENGTH: usize = 12;

fn part_2(banks: &Vec<Vec<u32>>) -> u64 {
    banks
        .iter()
        .map(|bank| {
            let mut largest_battery_seq = [0; SEQUENCE_LENGTH];
            let mut next_empty_index = 0;

            bank.iter().enumerate().for_each(|(index, new_battery)| {
                let earliest_assignable = SEQUENCE_LENGTH.saturating_sub(bank.len() - index);
                let latest_comparable = (next_empty_index).min(SEQUENCE_LENGTH);
                for index in earliest_assignable..latest_comparable {
                    let current_battery = largest_battery_seq[index];
                    if *new_battery > current_battery {
                        largest_battery_seq[index] = *new_battery;
                        next_empty_index = index + 1;
                        return;
                    }
                }

                if next_empty_index < SEQUENCE_LENGTH {
                    largest_battery_seq[next_empty_index] = *new_battery;
                    next_empty_index += 1;
                    return;
                }
            });

            largest_battery_seq
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let power: u32 = (SEQUENCE_LENGTH - index - 1)
                        .try_into()
                        .expect("Failed to fit power in u32");
                    let value: u64 = (*value).into();
                    value * (10_u64.pow(power))
                })
                .sum::<u64>()
        })
        .sum()
}
