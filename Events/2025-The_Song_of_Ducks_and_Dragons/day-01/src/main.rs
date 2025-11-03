fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let (names, moves) = parse(input);

    let length = names.len() as i32;
    let mut current = 0;

    for Step { delta } in moves.iter() {
        current = (current + delta).clamp(0, length - 1);
    }

    let name = names[current as usize];
    println!("Part 1. Name = {}", name);
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let (names, moves) = parse(input);

    let length = names.len() as i32;
    let mut current = 0;

    for Step { delta } in moves.iter() {
        current = (current + delta).rem_euclid(length);
    }

    let name = names[current as usize];
    println!("Part 2. Name = {}", name);
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let (mut names, moves) = parse(input);

    let length = names.len() as i32;

    for Step { delta } in moves.iter() {
        let swap_index = delta.rem_euclid(length) as usize;
        names.swap(0, swap_index);
    }

    let name = names[0];
    println!("Part 3. Name = {}", name);
}

struct Step {
    delta: i32,
}

fn parse(s: &str) -> (Vec<&str>, Vec<Step>) {
    let (names, moves) = s.trim().split_once("\n\n").unwrap();

    let names: Vec<&str> = names.split(',').collect();

    let moves: Vec<Step> = moves
        .split(',')
        .map(|m| {
            let (dir, count) = m.split_at(1);
            let count = count.parse::<i32>().unwrap();

            let delta = match dir {
                "L" => -1 * count,
                "R" => count,
                _ => panic!("Bad input!"),
            };

            Step { delta }
        })
        .collect();

    (names, moves)
}
