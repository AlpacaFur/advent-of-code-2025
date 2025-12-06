use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let ranges: Vec<(u64, u64)> = input
        .trim()
        .split(",")
        .map(|range| {
            let mut elems = range.split("-");
            (
                elems.next().unwrap().parse::<u64>().unwrap(),
                elems.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect();

    println!("Part 1:");
    println!("{}", part_1(&ranges));
    println!("Part 2:");
    println!("{}", part_2(&ranges));
}

fn get_digit(num: u64, digit: u64, digit_len: u64) -> u64 {
    (num / 10_u64.pow((digit_len - digit - 1).try_into().unwrap())) % 10
}

fn is_repeater(num: u64) -> bool {
    let digit_len: u64 = (num.ilog10() + 1).into();
    if digit_len % 2 == 1 {
        return false;
    }
    let half = digit_len / 2;
    for digit_index in 0..half {
        if get_digit(num, digit_index, digit_len) != get_digit(num, digit_index + half, digit_len) {
            return false;
        }
    }
    return true;
}

fn part_1(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|range| {
            let mut sum = 0;
            for val in (range.0)..(range.1 + 1) {
                if is_repeater(val) {
                    sum += val
                };
            }
            sum
        })
        .sum()
}

fn try_group_size(num: u64, digit_len: u64, num_groups: u64, group_size: u64) -> bool {
    for group in 1..(num_groups) {
        for digit in 0..group_size {
            if get_digit(num, digit, digit_len)
                != get_digit(num, digit + group * group_size, digit_len)
            {
                return false;
            }
        }
    }
    true
}

fn is_repeater_2(num: u64) -> bool {
    let digit_len: u64 = (num.ilog10() + 1).into();

    for num_groups_candidate in 2..(digit_len + 1) {
        if digit_len % num_groups_candidate != 0 {
            continue;
        }
        let size = digit_len / num_groups_candidate;
        if try_group_size(num, digit_len, num_groups_candidate, size) {
            return true;
        }
    }
    return false;
}

#[test]
fn test_is_repeater_2() {
    assert!(!is_repeater_2(1));
    assert!(!is_repeater_2(10));
    assert!(!is_repeater_2(101));
    assert!(!is_repeater_2(1001));
    assert!(is_repeater_2(11));
    assert!(is_repeater_2(111));
    assert!(is_repeater_2(1111));
    assert!(is_repeater_2(1010));
    assert!(is_repeater_2(1212));
    assert!(is_repeater_2(11111));
    assert!(is_repeater_2(111111));
    assert!(is_repeater_2(121212));
    assert!(is_repeater_2(123123));
    assert!(is_repeater_2(1111111));
    assert!(is_repeater_2(11111111));
    assert!(is_repeater_2(111111111));
    assert!(is_repeater_2(1111111111));
}

fn part_2(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges
        .iter()
        .map(|range| {
            let mut sum = 0;
            for val in (range.0)..(range.1 + 1) {
                if is_repeater_2(val) {
                    sum += val
                };
            }
            sum
        })
        .sum()
}
