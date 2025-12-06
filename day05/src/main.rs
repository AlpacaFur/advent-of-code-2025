use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Missing input file!");

    let [ranges, ingredients] = input.trim().split("\n\n").collect::<Vec<&str>>()[..] else {
        panic!("Could'nt split input.");
    };

    let ranges: Vec<(u64, u64)> = ranges
        .split("\n")
        .map(|range| -> (u64, u64) {
            let [start, end] = range.split("-").collect::<Vec<_>>()[..] else {
                panic!("couldn't split range")
            };
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let ingredients: Vec<u64> = ingredients
        .split("\n")
        .map(|ingredient| ingredient.parse().unwrap())
        .collect();

    println!("Part 1:");
    println!("{}", part_1(&ranges, &ingredients));
    println!("Part 2:");
    println!("{}", part_2(ranges));
}

fn part_1(ranges: &Vec<(u64, u64)>, ingredients: &Vec<u64>) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| {
            ranges
                .iter()
                .any(|(start, end)| *ingredient >= start && *ingredient <= end)
        })
        .count()
}

fn part_2(mut ranges: Vec<(u64, u64)>) -> u64 {
    ranges.sort_by(|(start_1, _), (start_2, _)| start_1.cmp(start_2));
    let mut intervals = vec![];

    let mut current_range = ranges[0];
    ranges.iter().for_each(|(new_start, new_end)| {
        if *new_start <= current_range.1 + 1 {
            current_range.1 = current_range.1.max(*new_end)
        } else {
            intervals.push(current_range);
            current_range = (*new_start, *new_end)
        }
    });
    intervals.push(current_range);

    intervals.iter().map(|(start, end)| end - start + 1).sum()
}
