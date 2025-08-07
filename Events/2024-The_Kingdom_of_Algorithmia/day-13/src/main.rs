use glam::IVec2;
use std::{cmp::Reverse, collections::HashMap};

const OFFSETS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let grid = parse(input);
    let times = shortest_times(&grid, grid.starts[0]);

    if let Some(time) = times.get(&grid.goals[0]) {
        println!("Part 1. Time: {}", time);
    }
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let grid = parse(input);
    let times = shortest_times(&grid, grid.starts[0]);

    if let Some(time) = times.get(&grid.goals[0]) {
        println!("Part 2. Time: {}", time);
    }
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let grid = parse(input);
    let times = shortest_times(&grid, grid.goals[0]);

    if let Some(shortest_time) = times
        .into_iter()
        .filter(|(pos, _)| grid.starts.contains(pos))
        .map(|(_, time)| time)
        .min()
    {
        println!("Part 2. Time: {}", shortest_time);
    }
}

fn shortest_times(grid: &Grid, start: IVec2) -> HashMap<IVec2, usize> {
    let mut prio_queue = priority_queue::PriorityQueue::new();
    let mut times = HashMap::from([(start, 0usize)]);

    prio_queue.push(start, Reverse(0usize));

    while let Some((current_node, Reverse(current_time))) = prio_queue.pop() {
        let current_elevation = grid.nodes.get(&current_node).expect("Should always exist");

        let neighbours = OFFSETS
            .iter()
            .map(|off| current_node + off)
            .filter(|n| grid.nodes.contains_key(n));

        for neighbour in neighbours {
            let neighbour_elevation = grid.nodes.get(&neighbour).expect("Should always exist");

            let height_diff = (neighbour_elevation - current_elevation).abs();
            let height_shift = height_diff.min(10 - height_diff) as usize;
            let neighbour_time = current_time + height_shift + 1;

            match times.get(&neighbour) {
                Some(&old_time) if neighbour_time < old_time => {
                    times.insert(neighbour, neighbour_time);
                    prio_queue.change_priority(&neighbour, Reverse(neighbour_time));
                }
                None => {
                    times.insert(neighbour, neighbour_time);
                    prio_queue.push(neighbour, Reverse(neighbour_time));
                }
                _ => (),
            }
        }
    }

    times
}

#[derive(Debug)]
struct Grid {
    nodes: HashMap<IVec2, i32>,
    starts: Vec<IVec2>,
    goals: Vec<IVec2>,
}

fn parse(input: &str) -> Grid {
    let mut nodes = HashMap::new();
    let mut starts = Vec::new();
    let mut goals = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);

            match c {
                'S' => {
                    nodes.insert(pos, 0);
                    starts.push(pos);
                }
                'E' => {
                    nodes.insert(pos, 0);
                    goals.push(pos);
                }
                '#' | ' ' => (),
                _ => {
                    let level = c.to_digit(10).unwrap();
                    nodes.insert(pos, level as i32);
                }
            }
        }
    }

    Grid {
        nodes,
        starts,
        goals,
    }
}
