use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

const OFFSETS: [IVec2; 5] = [IVec2::ZERO, IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1.txt");
    let mut dice = parse_dice(input1);

    let mut sum = 0;
    let mut rolls = 0;

    while sum < 10_000 {
        rolls += 1;

        for die in dice.iter_mut() {
            sum += die.roll();
        }
    }

    println!("Part 1. Rolls: {}", rolls);
}

fn part_two() {
    let input2 = include_str!("../input2.txt");
    let (dice_input, track_input) = input2.split_once("\n\n").unwrap();

    let mut dice = parse_dice(dice_input);

    let track: Vec<i32> = track_input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let race_results: Vec<(usize, usize)> = dice
        .iter_mut()
        .map(|die| (rolls_in_race(&track, die), die.id))
        .collect();

    let winner_order: String = race_results
        .into_iter()
        .sorted_by_key(|(rolls, _id)| *rolls)
        .map(|(_, id)| id)
        .join(",");

    println!("Part 2. Rolls: {}", winner_order);
}

fn part_three() {
    let input3 = include_str!("../input3.txt");
    let (dice_input, grid_input) = input3.split_once("\n\n").unwrap();

    let grid = Grid::from_str(grid_input);
    let mut dice = parse_dice(dice_input);

    let all_winning_spaces: HashSet<IVec2> = {
        let mut set = HashSet::new();
        for die in dice.iter_mut() {
            set.extend(reachable_coins_by_walk(&grid, die));
        }
        set
    };

    let coins = all_winning_spaces.len();
    println!("Part 3. Coins: {}", coins);
}

fn rolls_in_race(track: &[i32], die: &mut Die) -> usize {
    let mut rolls = 0;

    for target_number in track.iter() {
        rolls += 1;

        while *target_number != die.roll() {
            rolls += 1;
        }
    }

    rolls
}

fn reachable_coins_by_walk(grid: &Grid, die: &mut Die) -> HashSet<IVec2> {
    let mut reached_coins = HashSet::new();
    let mut active_players: HashSet<IVec2> = grid.nodes.keys().cloned().collect();

    let mut current_die_face = die.roll();

    while !active_players.is_empty() {
        active_players = active_players
            .into_iter()
            .flat_map(|player| {
                if Some(&current_die_face) == grid.nodes.get(&player) {
                    reached_coins.insert(player);
                    OFFSETS.iter().map(|off| player + off).collect()
                } else {
                    vec![]
                }
            })
            .collect();

        current_die_face = die.roll();
    }

    reached_coins
}

struct Die {
    id: usize,
    seed: usize,
    roll_number: usize,
    faces: Vec<i32>,
    index: usize,
    pulse: usize,
}

impl Die {
    fn roll(&mut self) -> i32 {
        self.roll_number += 1;
        let spin = self.roll_number * self.pulse;
        self.pulse += spin;
        self.pulse %= self.seed;
        self.pulse += 1 + self.roll_number + self.seed;
        self.index = (self.index + spin) % self.faces.len();

        self.faces[self.index]
    }

    fn from_str(description: &str) -> Self {
        let parts: Vec<&str> = description.split(' ').collect();
        let id = parts[0]
            .strip_suffix(':')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let faces: Vec<i32> = parts[1]
            .strip_prefix("faces=[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .split(',')
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        let seed = parts[2]
            .strip_prefix("seed=")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Die {
            id,
            seed,
            roll_number: 0,
            faces,
            index: 0,
            pulse: seed,
        }
    }
}

fn parse_dice(input: &str) -> Vec<Die> {
    input.lines().map(Die::from_str).collect()
}

struct Grid {
    nodes: HashMap<IVec2, i32>,
}

impl Grid {
    fn from_str(input: &str) -> Self {
        let nodes = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        IVec2::new(x as i32, y as i32),
                        c.to_digit(10).unwrap() as i32,
                    )
                })
            })
            .collect();

        Grid { nodes }
    }
}
