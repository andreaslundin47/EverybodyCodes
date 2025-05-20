use itertools::Itertools;

fn main() {
    let inputs = [
        include_str!("../input1"),
        include_str!("../input2"),
        include_str!("../input3"),
    ];

    for (i, input) in inputs.iter().enumerate() {
        let group_size = i + 1;
        let potions = count_potions(input, group_size);
        println!("Part {}: {} potions needed.", group_size, potions);
    }
}

fn count_potions(enemies: &str, group_size: usize) -> usize {
    enemies
        .chars()
        .chunks(group_size)
        .into_iter()
        .map(|group| {
            let group: Vec<char> = group.collect();

            let individual_potions: usize = group
                .iter()
                .map(|c| match c {
                    'B' => 1,
                    'C' => 3,
                    'D' => 5,
                    _ => 0,
                })
                .sum();

            let enemies_count = group.iter().filter(|&&e| e != 'x').count();
            let cooperation_potions = enemies_count * enemies_count.saturating_sub(1);

            individual_potions + cooperation_potions
        })
        .sum()
}
