use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt").trim();
    let triange = Triangle::parse(input);

    let mut count = 0;

    triange.grid.iter().for_each(|row| {
        row.iter().tuple_windows().for_each(|(a, b)| {
            if a == b && a == &'T' {
                count += 1;
            }
        })
    });

    triange.grid.iter().tuple_windows().for_each(|(r1, r2)| {
        r1.iter().skip(1).zip(r2).for_each(|(a, b)| {
            if a == b && a == &'T' {
                count += 1;
            }
        });
    });

    println!("Part 1. {count}");
}

fn part_two() {
    let input = include_str!("../input2.txt").trim();
    let triangle = Triangle::parse(input);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::from([triangle.start]);
    queue.push_back((triangle.start, 0));

    while let Some((current_pos, current_dist)) = queue.pop_front() {
        if current_pos == triangle.exit {
            println!("Part 2. {current_dist}");
            return;
        }

        for (ny, nx) in triangle.get_neighbours(current_pos) {
            if triangle.grid[ny][nx] == '#' {
                continue;
            }

            if !seen.contains(&(ny, nx)) {
                seen.insert((ny, nx));
                queue.push_back(((ny, nx), current_dist + 1));
            }
        }
    }
    println!("Part 2. None found...");
}

fn part_three() {
    let input = include_str!("../input3.txt").trim();
    let triangle = Triangle::parse(input);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::from([triangle.start]);
    queue.push_back((triangle.start, 0));

    while let Some((current_pos, current_dist)) = queue.pop_front() {
        if current_pos == triangle.exit {
            println!("Part 3. {current_dist}");
            return;
        }

        let current_pos = triangle.jump_counter_clockwise(current_pos);

        for (ny, nx) in triangle.get_neighbours(current_pos) {
            if triangle.grid[ny][nx] == '#' {
                continue;
            }

            if !seen.contains(&(ny, nx)) {
                seen.insert((ny, nx));
                queue.push_back(((ny, nx), current_dist + 1));
            }
        }
    }
    println!("Part 3. None found...");
}

type Point = (usize, usize);

#[derive(Debug)]
struct Triangle {
    grid: Vec<Vec<char>>,
    start: Point,
    exit: Point,
}

impl Triangle {
    fn parse(s: &str) -> Self {
        let mut start = (0, 0);
        let mut exit = (0, 0);
        let mut grid = vec![];

        for (y, line) in s.lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.trim_start_matches('.').chars().enumerate() {
                if c != '.' {
                    row.push(c)
                }
                if c == 'S' {
                    start = (y, x);
                }
                if c == 'E' {
                    exit = (y, x);
                }
            }
            grid.push(row);
        }

        Self { grid, start, exit }
    }

    fn get_neighbours(&self, current: Point) -> Vec<Point> {
        let (y, x) = current;
        let mut candidates = vec![];

        candidates.push((y, x));

        if x > 0 {
            candidates.push((y, x - 1));
        }

        if x < self.grid[y].len() - 1 {
            candidates.push((y, x + 1));
        }

        if x % 2 == 0 && y > 0 {
            candidates.push((y - 1, x + 1));
        }

        if x % 2 == 1 {
            candidates.push((y + 1, x - 1));
        }

        candidates
    }

    fn jump_counter_clockwise(&self, current: Point) -> Point {
        let (y, x) = current;

        let ny = self.grid.len() - 1 - (x + 1) / 2 - y;
        let nx = y * 2 + if x % 2 == 1 { 1 } else { 0 };

        (ny, nx)
    }
}
