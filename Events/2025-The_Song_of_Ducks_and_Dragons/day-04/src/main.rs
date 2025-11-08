fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let gears = parse(input);

    let first = gears.first().unwrap();
    let last = gears.last().unwrap();

    let turns = (2025 * first) / last;
    println!("Part 1. {turns}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let gears = parse(input);

    let first = gears.first().unwrap().clone();
    let last = gears.last().unwrap();
    let target = 10_000_000_000_000;

    let turns = (target * last).div_ceil(first);
    println!("Part 2. {turns}");
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let lines: Vec<&str> = input.trim().lines().collect();

    let mut factor = 100 * lines[0].parse::<usize>().unwrap();

    for i in 1..lines.len() - 1 {
        let (a, b) = lines[i].split_once('|').unwrap();
        let ratio = b.parse::<usize>().unwrap() / a.parse::<usize>().unwrap();
        factor *= ratio;
    }

    factor /= lines[lines.len() - 1].parse::<usize>().unwrap();
    println!("Part 3. {factor}");
}

fn parse(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}
