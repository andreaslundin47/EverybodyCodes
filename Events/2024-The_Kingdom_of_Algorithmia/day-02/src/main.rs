use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    part1();
    part2();
    part3();
}

fn part3() {
    let input3 = include_str!("../input3");

    let (words, wall_of_text) = input3.split_once("\n\n").unwrap();

    let words: Vec<String> = parse_words(words);

    let wall: Vec<Vec<char>> = wall_of_text
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let height = wall.len() as i32;
    let width = wall[0].len() as i32;

    let deltas: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut used = HashSet::new();

    for word in &words {
        for (dy, dx) in &deltas {
            for (sy, sx) in (0..height).cartesian_product(0..width) {
                let coord_chars: Vec<((usize, usize), char)> = word
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        (
                            (
                                (sy + dy * i as i32) as usize,
                                (sx + dx * i as i32).rem_euclid(width) as usize,
                            ),
                            c,
                        )
                    })
                    .collect();

                if coord_chars
                    .iter()
                    .any(|((y, _), _)| *y >= (height as usize))
                {
                    continue;
                }

                if coord_chars.iter().all(|((y, x), c)| wall[*y][*x] == *c) {
                    for (coord, _) in coord_chars {
                        used.insert(coord);
                    }
                }
            }
        }
    }

    let scales = used.len();
    println!("Part 3. {}", scales);
}

fn part2() {
    let input2 = include_str!("../input2");

    let (words, sentences) = input2.split_once("\n\n").unwrap();
    let words: Vec<String> = parse_words(words);

    let rev_words: Vec<String> = words
        .iter()
        .map(|word| word.chars().rev().collect::<String>())
        .collect();

    let all_words = [words, rev_words].concat();

    let symbols_count: usize = sentences
        .lines()
        .map(|sentence| {
            let mut used: HashSet<usize> = HashSet::new();

            for i in 0..sentence.len() {
                for word in &all_words {
                    if sentence[i..].starts_with(word) {
                        for pos in i..i + word.len() {
                            used.insert(pos);
                        }
                    }
                }
            }

            used.len()
        })
        .sum();

    println!("Part 2. {}", symbols_count);
}

fn part1() {
    let input1 = include_str!("../input1");
    let (words, text) = input1.split_once("\n\n").unwrap();

    let words = parse_words(words);

    let word_count: usize = (0..text.len())
        .map(|i| words.iter().filter(|w| text[i..].starts_with(*w)).count())
        .sum();

    println!("Part 1. {}", word_count);
}

fn parse_words(input: &str) -> Vec<String> {
    input
        .strip_prefix("WORDS:")
        .unwrap()
        .split(',')
        .map(|w| w.to_owned())
        .collect()
}
