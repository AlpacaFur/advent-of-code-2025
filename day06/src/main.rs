use std::fs;

#[derive(Debug)]
enum Operation {
    Plus,
    Times,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let mut rows: Vec<&str> = input.trim().split("\n").collect();
    let operations: Vec<Operation> = rows
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|str| match str {
            "*" => Operation::Times,
            "+" => Operation::Plus,
            _ => panic!("invalid operation"),
        })
        .collect();
    let rows: Vec<Vec<u64>> = rows
        .into_iter()
        .map(|row| {
            row.trim().split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", operations);
    println!("{:?}", rows);

    println!("Part 1:");
    println!("{}", part_1(&rows, &operations));
    // println!("Part 2:");
    // println!("{}", part_2(ranges));
}

fn part_1(number_rows: &Vec<Vec<u64>>, operations: &Vec<Operation>) -> u64 {
    let mut result: Vec<u64> = vec![];
    for index in 0..operations.len() {
        let operation = &operations[index];
        result.push(
            number_rows
                .iter()
                .map(|row| row[index])
                .reduce(|acc, num| match operation {
                    Operation::Plus => acc + num,
                    Operation::Times => acc * num,
                })
                .unwrap(),
        );
    }
    result.iter().sum()
}

fn part_2() -> u64 {
    0
}
