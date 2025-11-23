use glam::IVec2;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let world = World::new(input);
    println!(
        "Part 1. {}",
        match get_shortest_distance(&world) {
            Some(min_dist) => min_dist.to_string(),
            None => "Did not find target...".to_string(),
        }
    );
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let world = World::new(input);
    println!(
        "Part 2. {}",
        match get_shortest_distance(&world) {
            Some(min_dist) => min_dist.to_string(),
            None => "Did not find target...".to_string(),
        }
    );
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let world = World::new(input);
    println!(
        "Part 3. {}",
        match get_shortest_distance(&world) {
            Some(min_dist) => min_dist.to_string(),
            None => "Did not find target...".to_string(),
        }
    );
}

fn get_shortest_distance(world: &World) -> Option<i32> {
    let mut priority_queue = PriorityQueue::new();
    priority_queue.push(world.start, Reverse(0));
    let mut dists = HashMap::from([(world.start, 0)]);
    let mut visited = HashSet::new();

    while let Some((current_pos, Reverse(current_dist))) = priority_queue.pop() {
        visited.insert(current_pos);

        if current_pos == world.end {
            return Some(current_dist);
        }

        for neighbour_pos in world.get_valid_neighbours(&current_pos) {
            if visited.contains(&neighbour_pos) {
                continue;
            };

            let delta = neighbour_pos.manhattan_distance(current_pos) as i32;
            dists.insert(neighbour_pos, current_dist + delta);

            if let Some(&previous_dist) = dists.get(&neighbour_pos)
                && current_dist + delta < previous_dist
            {
                priority_queue.change_priority(&neighbour_pos, Reverse(current_dist + delta));
            } else {
                priority_queue.push(neighbour_pos, Reverse(current_dist + delta));
            }
        }
    }

    None
}

#[derive(Debug)]
struct CollisionBox {
    x_range: Range<i32>,
    y_range: Range<i32>,
}

impl CollisionBox {
    fn new(start: IVec2, end: IVec2) -> Self {
        Self {
            x_range: start.x.min(end.x)..(start.x.max(end.x) + 1),
            y_range: start.y.min(end.y)..(start.y.max(end.y) + 1),
        }
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.x_range.start < other.x_range.end && self.x_range.end > other.x_range.start)
            && (self.y_range.start < other.y_range.end && self.y_range.end > other.y_range.start)
    }
}

#[derive(Debug)]
struct World {
    walls: Vec<CollisionBox>,
    start: IVec2,
    end: IVec2,
    reduced_xs: Vec<i32>,
    reduced_ys: Vec<i32>,
}

impl World {
    fn get_valid_neighbours(&self, point: &IVec2) -> impl Iterator<Item = IVec2> {
        [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y]
            .iter()
            .filter_map(|delta| self.get_neighbour(point, delta))
    }

    fn get_neighbour(&self, point: &IVec2, delta: &IVec2) -> Option<IVec2> {
        let old_x_index = self.reduced_xs.iter().position(|x| x == &point.x)?;
        let old_y_index = self.reduced_ys.iter().position(|y| y == &point.y)?;

        let new_x_index: usize = old_x_index.checked_add_signed(delta.x.try_into().ok()?)?;
        let new_y_index: usize = old_y_index.checked_add_signed(delta.y.try_into().ok()?)?;

        let new_x = self.reduced_xs.get(new_x_index)?;
        let new_y = self.reduced_ys.get(new_y_index)?;

        let new_pos = IVec2::new(*new_x, *new_y);

        let shifted_old_pos = IVec2::new(
            point.x + (new_pos.x - point.x).signum(),
            point.y + (new_pos.y - point.y).signum(),
        );

        let travel_hit_box = CollisionBox::new(shifted_old_pos, new_pos);

        if self.walls.iter().any(|wall| wall.overlaps(&travel_hit_box)) {
            None
        } else {
            Some(new_pos)
        }
    }

    fn new(s: &str) -> Self {
        let start = IVec2::ZERO;
        let mut current_dir = IVec2::NEG_Y;
        let mut current_pos = start;
        let mut walls = vec![];
        let mut xs = HashSet::new();
        let mut ys = HashSet::new();

        for steps in s.trim().split(',') {
            let (rotation, count) = steps.split_at(1);

            current_dir = match rotation {
                "L" => IVec2::NEG_Y.rotate(current_dir),
                "R" => IVec2::Y.rotate(current_dir),
                _ => panic!("Unexpected input format"),
            };

            let count = count.parse::<i32>().expect("Should parse as an integer");

            walls.push(CollisionBox::new(
                current_pos,
                current_pos + (count - 1) * current_dir,
            ));

            let previous_pos = current_pos;
            current_pos += count * current_dir;

            [-1, 0, 1].iter().for_each(|delta| {
                xs.insert(previous_pos.x + delta);
                ys.insert(previous_pos.y + delta);

                xs.insert(current_pos.x + delta);
                ys.insert(current_pos.y + delta);
            });
        }

        Self {
            walls,
            start,
            end: current_pos,
            reduced_xs: xs.into_iter().sorted().collect::<Vec<i32>>(),
            reduced_ys: ys.into_iter().sorted().collect::<Vec<i32>>(),
        }
    }
}
