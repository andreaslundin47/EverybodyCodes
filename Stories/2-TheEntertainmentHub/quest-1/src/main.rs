use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1.txt");
    let (machine, tokens) = parse(input1);

    let mut coins = 0;
    for (index, token) in tokens.iter().enumerate() {
        let toss_slot = index + 1;
        let final_slot = machine.drop_token(token, toss_slot);
        let coins_won = (final_slot * 2).saturating_sub(toss_slot);
        coins += coins_won
    }

    println!("Part 1. Coins won: {}", coins);
}

fn part_two() {
    let input2 = include_str!("../input2.txt");
    let (machine, tokens) = parse(input2);

    let mut coins = 0;

    for token in tokens.iter() {
        let max_coins_won = (1..=machine.slot_count)
            .map(|toss_slot| {
                let final_slot = machine.drop_token(token, toss_slot);
                let coins_won = (final_slot * 2).saturating_sub(toss_slot);
                coins_won
            })
            .max()
            .expect("Should be able to get a maximum from scores for each slot");

        coins += max_coins_won;
    }

    println!("Part 2. Coins won: {}", coins);
}

fn part_three() {
    let input3 = include_str!("../input3.txt");
    let (machine, tokens) = parse(input3);

    let pre_calculated_scores: Vec<Vec<usize>> = tokens
        .iter()
        .map(|token_index| {
            (1..=machine.slot_count)
                .map(|toss_slot| {
                    let final_slot = machine.drop_token(token_index, toss_slot);
                    let coins_won = (final_slot * 2).saturating_sub(toss_slot);

                    coins_won
                })
                .collect()
        })
        .collect();

    let mut min_score = usize::MAX;
    let mut max_score = 0usize;

    for slot_permutation in (1..=machine.slot_count).permutations(tokens.len()) {
        let score: usize = slot_permutation
            .iter()
            .enumerate()
            .map(|(token_index, slot_number)| pre_calculated_scores[token_index][slot_number - 1])
            .sum();

        min_score = min_score.min(score);
        max_score = max_score.max(score);
    }

    println!("Part 3. min-max: {} {}", min_score, max_score);
}

fn parse(input: &str) -> (Machine, Vec<Token>) {
    let (machine, tokens) = input.split_once("\n\n").unwrap();

    let parts: HashMap<IVec2, char> = machine
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (IVec2::new(x as i32, y as i32), c))
                .collect::<HashMap<IVec2, char>>()
        })
        .collect();

    let max_x: i32 = parts.keys().map(|v| v.x).max().unwrap();
    let slot_count = (max_x / 2 + 1) as usize;

    let machine = Machine { parts, slot_count };

    let tokens = tokens
        .lines()
        .map(|line| {
            let moves = line
                .chars()
                .into_iter()
                .map(|c| match c {
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => panic!("Not a valid direction!"),
                })
                .collect();

            Token { moves }
        })
        .collect();

    (machine, tokens)
}

enum Dir {
    Left,
    Right,
}

struct Token {
    moves: Vec<Dir>,
}
struct Machine {
    parts: HashMap<IVec2, char>,
    slot_count: usize,
}

impl Machine {
    fn drop_token(&self, token: &Token, toss_slot: usize) -> usize {
        let mut pos = IVec2::new((toss_slot as i32 - 1) * 2, 0);

        let mut dirs = token.moves.iter();

        loop {
            if let Some(part) = self.parts.get(&pos) {
                if part == &'.' {
                    pos = pos + IVec2::Y;
                } else {
                    let token_direction = match dirs.next().unwrap() {
                        Dir::Left => IVec2::NEG_X,
                        Dir::Right => IVec2::X,
                    };

                    let next_pos = pos + token_direction;

                    if self.parts.contains_key(&next_pos) {
                        pos = next_pos;
                    } else {
                        pos = pos - token_direction;
                    }
                }
            } else {
                break;
            }
        }

        let final_slot = pos.x / 2 + 1;

        final_slot as usize
    }
}
