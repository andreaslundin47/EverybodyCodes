fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let spell_nums = parse(input);
    let sum: usize = count_blocks(90, &spell_nums);
    println!("Part 1. {sum}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let mut nums = parse(input);
    let spell_nums = reconstruct_spell(&mut nums);
    let prod: usize = spell_nums.iter().product();
    println!("Part 2. {prod}");
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let mut nums = parse(input);

    let spell_nums = reconstruct_spell(&mut nums);
    let blocks_available: usize = 2025_2025_2025_000;

    let mut low = 1;
    let mut high = blocks_available;

    while low < high {
        let mid = (low + high) / 2;
        if count_blocks(mid, &spell_nums) > blocks_available {
            high = mid - 1;
        } else {
            low = mid;
        }
    }

    let min_len = low;
    println!("Part 3. {min_len}");
}

fn parse(s: &str) -> Vec<usize> {
    s.trim()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

fn count_blocks(wall_length: usize, spell_numbers: &[usize]) -> usize {
    spell_numbers.iter().map(|n| wall_length / n).sum()
}

fn reconstruct_spell(wall: &mut [usize]) -> Vec<usize> {
    let mut spell_nums = vec![];

    for candidate in 1..=wall.len() {
        if wall[candidate - 1] > 0 {
            spell_nums.push(candidate);
            for j in (candidate..=wall.len()).step_by(candidate) {
                wall[j - 1] -= 1;
            }
        }
    }

    spell_nums
}
