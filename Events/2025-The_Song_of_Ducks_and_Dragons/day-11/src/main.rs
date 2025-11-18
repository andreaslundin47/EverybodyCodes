fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let mut birds = parse(input);

    let mut turn = 0;

    loop {
        let mut no_change = true;

        for i in 0..birds.len() - 1 {
            if birds[i + 1] < birds[i] {
                no_change = false;
                birds[i + 1] += 1;
                birds[i] -= 1;
            }
        }

        if no_change {
            break;
        }
        turn += 1;
    }

    for _ in turn..10 {
        for i in 0..birds.len() - 1 {
            if birds[i] < birds[i + 1] {
                birds[i + 1] -= 1;
                birds[i] += 1;
            }
        }
    }

    let checksum: usize = birds.iter().enumerate().map(|(i, b)| (i + 1) * b).sum();
    println!("Part 1. {checksum}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let mut birds = parse(input);

    let mut turns = 0;
    loop {
        let mut no_change = true;

        for i in 0..birds.len() - 1 {
            if birds[i] > birds[i + 1] {
                no_change = false;
                birds[i] -= 1;
                birds[i + 1] += 1;
            }
        }

        if no_change {
            break;
        }
        turns += 1;
    }

    loop {
        let mut no_change = true;

        for i in 0..birds.len() - 1 {
            if birds[i] < birds[i + 1] {
                no_change = false;
                birds[i + 1] -= 1;
                birds[i] += 1;
            }
        }

        if no_change {
            break;
        }
        turns += 1;
    }

    println!("Part 2. {turns}");
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let birds = parse(input);

    // Input is already sorted from smallest to largest. We have a very nice special case
    let average_birds: usize = birds.iter().sum::<usize>() / birds.len();
    let turns: usize = birds.iter().map(|b| b.saturating_sub(average_birds)).sum();
    println!("Part 3. {turns}");
}

fn parse(s: &str) -> Vec<usize> {
    s.trim()
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}
