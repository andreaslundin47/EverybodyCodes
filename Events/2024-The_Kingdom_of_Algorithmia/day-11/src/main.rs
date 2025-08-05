use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1.txt");
    let rules = parse(input1);
    let total_termite_count = count_termites(&rules, "A", 4);
    println!("Part 1. Count: {}", total_termite_count);
}

fn part_two() {
    let input2 = include_str!("../input2.txt");
    let rules = parse(input2);
    let total_termite_count = count_termites(&rules, "Z", 10);
    println!("Part 2. Count: {}", total_termite_count);
}

fn part_three() {
    let input3 = include_str!("../input3.txt");
    let rules = parse(input3);

    let counts: Vec<usize> = rules
        .iter()
        .map(|rule| count_termites(&rules, rule.input, 20))
        .collect();

    let max_difference = counts.iter().max().unwrap() - counts.iter().min().unwrap();

    println!("Part 3. Difference: {}", max_difference);
}

fn count_termites(rules: &[Rule], starter: &str, days: usize) -> usize {
    let mut current_counts = HashMap::<&str, usize>::from([(starter, 1)]);

    for _day in 0..days {
        let mut new_counts = HashMap::new();

        for rule in rules {
            if let Some(input_count) = current_counts.get(&rule.input) {
                for output in &rule.output {
                    *new_counts.entry(*output).or_insert(0) += input_count;
                }
            }
        }

        current_counts = new_counts;
    }

    current_counts.values().sum()
}

struct Rule<'a> {
    input: &'a str,
    output: Vec<&'a str>,
}

fn parse(input: &str) -> Vec<Rule> {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(':').unwrap();
            let output = output.split(',').collect();

            Rule { input, output }
        })
        .collect()
}
