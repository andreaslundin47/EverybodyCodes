fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = parse(include_str!("../input1.txt"));

    let sum: i32 = input
        .into_iter()
        .map(|pines| {
            let n = (pines as f32).sqrt().ceil() as i32;
            let square = n.pow(2);
            let to_add = square - pines;

            to_add
        })
        .sum();

    println!("Part 1. {}", sum);
}

fn part_two() {
    let input = parse(include_str!("../input2.txt"));

    let sum: i32 = input
        .into_iter()
        .map(|pines| {
            let n = ((pines as f32 + 4.) / 4.).ceil() as i32;
            let n = n.max(3);
            let boundary = 4 * n - 4;
            let to_add = boundary - pines;
            to_add
        })
        .sum();

    println!("Part 2. {}", sum);
}

fn part_three() {
    let input = parse(include_str!("../input3.txt"));

    let sum: i32 = input
        .into_iter()
        .map(|pines| {
            let n = (pines as f32 + 1.).sqrt().ceil() as i32;
            let n = n.max(3);
            let n = if n % 2 == 0 { n + 1 } else { n };
            let formation = n.pow(2) - 1;
            let to_add = formation - pines;

            to_add
        })
        .sum();

    println!("Part 3. {}", sum);
}

fn parse(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}
