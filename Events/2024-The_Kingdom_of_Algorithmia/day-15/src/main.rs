use ahash::{AHashMap, AHashSet};
use glam::IVec2;
use itertools::Itertools;
use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let maze = parse_input(input);
    if let Some(distance) = find_distance(&maze) {
        println!("Part 1. Distance: {}", distance);
    } else {
        println!("Part 1. Could not find a solution...");
    }
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let maze = parse_input(input);
    if let Some(distance) = find_distance(&maze) {
        println!("Part 2. Distance: {}", distance);
    } else {
        println!("Part 2. Could not find a solution...");
    }
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let maze = parse_input(input);
    if let Some(distance) = find_distance(&maze) {
        println!("Part 3. Distance: {}", distance);
    } else {
        println!("Part 3. Could not find a solution...");
    }
}

fn find_distance(maze: &AHashMap<IVec2, char>) -> Option<usize> {
    let start_and_end: IVec2 = maze.keys().find(|&&pos| pos.y == 0).cloned().unwrap();

    let unique_herbs: Vec<char> = maze
        .values()
        .filter(|v| v != &&'.')
        .unique()
        .cloned()
        .collect();

    let herb_score: usize = unique_herbs.into_iter().map(|h| 1 << h as u8 - b'A').sum();

    let mut seen = AHashSet::new();
    let mut queue: VecDeque<(IVec2, usize, usize)> = VecDeque::from([(start_and_end, 0, 0)]);

    while let Some((current_pos, current_inventory, current_dist)) = queue.pop_front() {
        if current_pos == start_and_end && current_inventory == herb_score {
            return Some(current_dist);
        }

        for offset in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
            let neighbour_pos = current_pos + offset;

            if let Some(&symbol) = maze.get(&neighbour_pos) {
                let herb = match symbol {
                    '.' => 0,
                    _ => 1 << symbol as u8 - b'A',
                };

                let neighbour_inventory = current_inventory | herb;

                if seen.contains(&(neighbour_pos, neighbour_inventory)) {
                    continue;
                }

                seen.insert((neighbour_pos, neighbour_inventory));
                queue.push_back((neighbour_pos, neighbour_inventory, current_dist + 1));
            }
        }
    }

    None
}

fn parse_input(input: &str) -> AHashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                (c != '#' && c != '~').then_some((IVec2::new(x as i32, y as i32), c))
            })
        })
        .collect()
}
