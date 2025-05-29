use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1");
    let roots = parse(input1);

    let mut queue = VecDeque::from([("".to_owned(), "RR".to_owned(), 1)]);
    let mut fruit_path_by_distances = HashMap::<usize, Vec<String>>::new();

    while let Some((path, root, dist)) = queue.pop_front() {
        if let Some(destinations) = roots.get(&root) {
            for destination in destinations {
                match destination.as_str() {
                    "@" => {
                        fruit_path_by_distances
                            .entry(dist)
                            .or_default()
                            .push(format!("{}{}@", path, &root[0..1]));
                    }
                    _ => {
                        queue.push_back((
                            format!("{}{}", path, &root[0..1]),
                            destination.clone(),
                            dist + 1,
                        ));
                    }
                }
            }
        }
    }

    if let Some(paths) = fruit_path_by_distances
        .values()
        .find(|paths| paths.len() == 1)
    {
        println!("Part 1. Path: {}", paths[0]);
    }
}

fn part_two() {
    let input2 = include_str!("../input2");
    let path = get_path(input2).expect("Should return path 2.");
    println!("Part 2. Path: {}", path);
}

fn part_three() {
    let input3 = include_str!("../input3");
    let path = get_path(input3).expect("Should return path 3.");
    println!("Part 3. Path: {}", path);
}

fn get_path(input: &str) -> Result<String> {
    let roots = parse(input);

    let mut queue = VecDeque::from([("".to_owned(), "RR".to_owned(), 1)]);
    let mut fruit_path_by_distances = HashMap::<usize, Vec<String>>::new();

    while let Some((previous_path, branching_root, dist)) = queue.pop_front() {
        if let Some(destinations) = roots.get(&branching_root) {
            destinations
                .iter()
                .for_each(|destination| match destination.as_str() {
                    "BUG" | "ANT" => (),
                    "@" => {
                        fruit_path_by_distances
                            .entry(dist)
                            .or_default()
                            .push(format!("{}{}@", previous_path, &branching_root[0..1]));
                    }
                    _ => {
                        queue.push_back((
                            format!("{}{}", previous_path, &branching_root[0..1]),
                            destination.clone(),
                            dist + 1,
                        ));
                    }
                });
        }
    }

    let unique_lengh_path = fruit_path_by_distances
        .values()
        .find(|paths| paths.len() == 1)
        .context("There is no path with a unique path-length.")?[0]
        .clone();

    Ok(unique_lengh_path)
}

fn parse(input: &str) -> HashMap<String, Vec<String>> {
    let mut roots = HashMap::new();

    for line in input.lines() {
        let (root, branches) = line
            .split_once(':')
            .expect("Root and branches should be separated by a ':'");

        let branches = branches.split(',').map(|s| s.to_string()).collect();
        roots.insert(root.to_string(), branches);
    }

    roots
}
