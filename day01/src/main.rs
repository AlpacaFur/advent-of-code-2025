use std::fs;

const DIAL_SIZE: i32 = 100;
const DIAL_START: i32 = 50;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|instr| {
            let multiplier = if instr.chars().nth(0).expect("bruh") == 'L' {
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

fn part_1(nums: &Vec<i32>) -> i32 {
    let mut current_val = DIAL_START;
    let mut zero_count = 0;
    nums.iter().for_each(|instr| {
        current_val = neg_inclusive_modulo(current_val + instr, DIAL_SIZE);
        if current_val == 0 {
            zero_count += 1
        }
    });
    return zero_count;
}

fn neg_inclusive_modulo(num: i32, mod_factor: i32) -> i32 {
    ((num % mod_factor) + mod_factor) % mod_factor
}

fn part_2(nums: &Vec<i32>) -> i32 {
    let mut current_val = DIAL_START;
    let mut zero_count = 0;
    nums.iter().for_each(|instr| {
        let full_rot_crosses = (instr / DIAL_SIZE).abs();
        zero_count += full_rot_crosses;

        let result = neg_inclusive_modulo(current_val + instr, DIAL_SIZE);
        if result == 0 {
            zero_count += 1;
        } else if current_val != 0 && (*instr < 0) && result > current_val {
            zero_count += 1;
        } else if (*instr > 0) && result < current_val {
            zero_count += 1;
        };

        current_val = result;
    });
    return zero_count;
}
