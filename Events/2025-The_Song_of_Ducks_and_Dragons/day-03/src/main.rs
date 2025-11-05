use itertools::Itertools;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn parse_crates(s: &str) -> Vec<usize> {
    s.trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let crate_sizes = parse_crates(input);
    let largest_packing_sum: usize = crate_sizes.iter().unique().sum();
    println!("Part 1. {}", largest_packing_sum);
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let crate_sizes = parse_crates(input);
    let minimum_20_crates_packing_sum: usize = crate_sizes.iter().unique().sorted().take(20).sum();
    println!("Part 2. {}", minimum_20_crates_packing_sum);
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let crate_sizes = parse_crates(input);

    let size_counts = crate_sizes.iter().counts();
    let minimum_number_sets: usize = *size_counts.values().max().unwrap_or(&0);
    println!("Part 3. {}", minimum_number_sets);
}
