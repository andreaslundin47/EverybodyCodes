use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let (names, rules) = parse(input);
    let name = names.iter().find(|name| rules.validate(name)).unwrap();
    println!("Part 1. {name}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let (names, rules) = parse(input);

    let index_sum: usize = names
        .iter()
        .enumerate()
        .filter_map(|(index, name)| rules.validate(name).then_some(index + 1))
        .sum();
    println!("Part 2. {index_sum}");
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let (names, rules) = parse(input);

    let names_count: usize = names
        .iter()
        .flat_map(|names_prefix| rules.get_valid_names(names_prefix))
        .unique()
        .count();
    println!("Part 3. {names_count}");
}

struct Rules {
    rules: HashMap<char, Vec<char>>,
}

impl Rules {
    fn validate(&self, name: &str) -> bool {
        for (c1, c2) in name.chars().tuple_windows() {
            if let Some(posts) = self.rules.get(&c1) {
                if !posts.contains(&c2) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    fn get_valid_names(&self, prefix: &str) -> Vec<String> {
        if !self.validate(prefix) {
            return vec![];
        }

        let mut names = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(prefix.to_string());

        while let Some(current_name) = queue.pop_front() {
            if current_name.len() < 11
                && let Some(last_char) = current_name.chars().last()
            {
                if let Some(posts) = self.rules.get(&last_char) {
                    for next_char in posts {
                        let next_name = format!("{current_name}{next_char}");
                        queue.push_back(next_name);
                    }
                }
            }

            if current_name.len() >= 7 {
                names.push(current_name);
            }
        }

        names
    }
}

fn parse(s: &str) -> (Vec<&str>, Rules) {
    let (names, rules) = s.trim().split_once("\n\n").unwrap();
    let names = names.split(',').collect();

    let rules = rules
        .lines()
        .map(|line| {
            let (pre, posts) = line.split_once(" > ").unwrap();
            let pre = pre.chars().next().unwrap();
            let posts: Vec<char> = posts
                .split(',')
                .map(|s| s.chars().next().unwrap())
                .collect();
            (pre, posts)
        })
        .collect();

    (names, Rules { rules })
}
