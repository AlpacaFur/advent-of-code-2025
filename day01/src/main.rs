use std::fs;

const DIAL_SIZE: i32 = 100;
const DIAL_START: i32 = 50;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|instr| {
            let multiplier = if instr.chars().nth(0).unwrap() == 'L' {
                -1
            } else {
                1
            };
            return multiplier
                * instr[1..]
                    .parse::<i32>()
                    .expect("Failed to parse input number");
        })
        .collect();

    println!("Part 1:");
    println!("{}", part_1(&nums));
    println!("Part 2:");
    println!("{}", part_2(&nums));
}

fn wrap_num(num: i32, wrap_bound: i32) -> i32 {
    ((num % wrap_bound) + wrap_bound) % wrap_bound
}

fn part_1(nums: &Vec<i32>) -> i32 {
    let mut current_val = DIAL_START;
    let mut zero_count = 0;
    nums.iter().for_each(|instr| {
        current_val = wrap_num(current_val + instr, DIAL_SIZE);
        if current_val == 0 {
            zero_count += 1
        }
    });
    return zero_count;
}

fn part_2(nums: &Vec<i32>) -> i32 {
    let mut current_val = DIAL_START;
    let mut zero_count = 0;

    nums.iter().for_each(|instr| {
        let full_rot_crosses = (instr / DIAL_SIZE).abs();
        zero_count += full_rot_crosses;

        let raw_rotation = current_val + (instr % DIAL_SIZE);
        let wrapped_rotation = wrap_num(current_val + instr, DIAL_SIZE);
        if (wrapped_rotation != raw_rotation && current_val != 0) || wrapped_rotation == 0 {
            zero_count += 1;
        }

        current_val = wrapped_rotation;
    });

    zero_count
}
