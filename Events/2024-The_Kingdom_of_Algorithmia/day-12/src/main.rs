use glam::IVec2;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let targets = parse_targets(input);

    let ranking_value: i32 = targets.into_iter().map(|t| ranking_score(t)).sum();
    println!("Part 1. Ranking: {}", ranking_value);
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let targets = parse_targets(input);

    let ranking_value: i32 = targets.into_iter().map(|t| ranking_score(t)).sum();
    println!("Part 2. Ranking: {}", ranking_value);
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let meteors = parse_meteors(input);

    let total_ranking_score: i32 = meteors
        .into_iter()
        .map(|m| ranking_score(meteor_intercept_position(m)))
        .sum();
    println!("Part 3. Ranking: {}", total_ranking_score);
}

fn ranking_score(target: IVec2) -> i32 {
    let (x, y) = (target.x, target.y);

    if y - x >= 0 {
        let turret = y - x + 1;
        let power = x;
        turret * power
    } else if x > y && x <= 2 * y {
        let turret = 1;
        let power = y;
        turret * power
    } else {
        let turret = (x + y) % 3 + 1;
        let power = (x + y) / 3;
        turret * power
    }
}

fn meteor_intercept_position(meteor: IVec2) -> IVec2 {
    let (x, y) = (meteor.x / 2, meteor.y - (meteor.x - meteor.x / 2));
    IVec2 { x, y }
}

fn parse_targets(input: &str) -> Vec<IVec2> {
    let mut targets = Vec::new();

    for (y, line) in input.lines().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as i32 - 1;
            let y = y as i32 - 1;

            match c {
                'T' => targets.push(IVec2::new(x, y)),
                'H' => {
                    targets.push(IVec2::new(x, y));
                    targets.push(IVec2::new(x, y));
                }
                _ => (),
            }
        }
    }

    targets
}

fn parse_meteors(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(' ').unwrap();
            IVec2::new(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}
