use std::collections::{HashMap, HashSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

use glam::IVec3;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let moves = parse(input);

    let mut max_height = 0;
    let mut current = IVec3::ZERO;

    for m in moves {
        current = current + m.dist * m.dir;
        max_height = max_height.max(current.z);
    }

    println!("Part 1. Height: {}", max_height);
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let branches: Vec<Vec<Move>> = input.lines().map(|line| parse(line)).collect();

    let mut unique_segments = HashSet::new();

    for branch in branches {
        let mut current = IVec3::ZERO;
        for m in branch {
            for _ in 0..m.dist {
                current = current + m.dir;
                unique_segments.insert(current);
            }
        }
    }

    println!("Part 2. Segments: {}", unique_segments.len());
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let branches: Vec<Vec<Move>> = input.lines().map(|line| parse(line)).collect();

    let mut unique_leaves = HashSet::new();
    let mut unique_segments = HashSet::new();

    for branch in branches {
        let mut current = IVec3::ZERO;
        for m in branch {
            for _ in 0..m.dist {
                current = current + m.dir;
                unique_segments.insert(current);
            }
        }

        unique_leaves.insert(current);
    }

    let mut trunk_murkiness: HashMap<IVec3, usize> = unique_segments
        .iter()
        .filter(|seg| seg.z > 0 && seg.x == 0 && seg.y == 0 && !unique_leaves.contains(seg))
        .map(|trunk| (*trunk, 0))
        .collect();

    // BFS from each leaf, and add distance to each trunk segment encountered
    for leaf in &unique_leaves {
        let mut queue = VecDeque::from([(*leaf, 0)]);
        let mut seen = HashSet::from([*leaf]);

        while let Some((current_leaf, current_dist)) = queue.pop_front() {
            if let Some(trunk) = trunk_murkiness.get_mut(&current_leaf) {
                *trunk += current_dist;
            }

            for offset in [
                IVec3::Z,
                IVec3::NEG_Z,
                IVec3::Y,
                IVec3::NEG_Y,
                IVec3::NEG_X,
                IVec3::X,
            ] {
                let neighbour = current_leaf + offset;

                if unique_segments.contains(&neighbour) && !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    queue.push_back((neighbour, current_dist + 1));
                }
            }
        }
    }

    let min_murkiness = trunk_murkiness
        .values()
        .min()
        .expect("Should exist if we have any trunk segments");

    println!("Part 3. Minimum murkiness: {}", min_murkiness);
}

struct Move {
    dir: IVec3,
    dist: i32,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir_char, dist_str) = s.split_at(1);
        let dist = dist_str
            .parse::<i32>()
            .map_err(|e: ParseIntError| e.to_string())?;
        let dir = match dir_char {
            "U" => IVec3::Z,
            "D" => IVec3::NEG_Z,
            "R" => IVec3::Y,
            "L" => IVec3::NEG_Y,
            "F" => IVec3::NEG_X,
            "B" => IVec3::X,
            _ => return Err(format!("Bad direction: {}", dir_char)),
        };
        Ok(Move { dir, dist })
    }
}

fn parse(input: &str) -> Vec<Move> {
    input
        .split(',')
        .map(|m| Move::from_str(m).expect("Invalid move"))
        .collect()
}
