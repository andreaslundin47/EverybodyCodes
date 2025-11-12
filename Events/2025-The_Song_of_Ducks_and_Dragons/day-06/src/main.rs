use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let mut sum = 0;
    let mut mentors = 0;

    for c in input.trim().chars() {
        if c == 'A' {
            mentors += 1;
        } else if c == 'a' {
            sum += mentors;
        }
    }

    println!("Part 1. {sum}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let mut mentors = HashMap::<char, usize>::new();
    let mut sum = 0;

    for c in input.trim().chars() {
        if c.is_ascii_uppercase() {
            mentors
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        } else if c == 'a' {
            let mentor = c.to_ascii_uppercase();
            sum += *mentors.entry(mentor).or_default();
        }
    }

    println!("Part 2. {sum}");
}

fn part_three() {
    let input = include_str!("../input3.txt");
    let chars: Vec<char> = input.trim().chars().collect();
    let f1 = count_relationships(&chars);

    let chars_repeat_once: Vec<char> = [chars.clone(), chars].concat();
    let f2 = count_relationships(&chars_repeat_once);

    let sum = f1 + 999 * (f2 - f1);
    println!("Part 3. {sum}");
}

fn count_relationships(chars: &[char]) -> usize {
    let mut mentors = HashMap::<char, usize>::new();
    let mut sum = 0;

    for c in chars.iter().take(1001) {
        if c.is_ascii_uppercase() {
            mentors
                .entry(*c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_lowercase() {
            let mentor = c.to_ascii_uppercase();
            sum += *mentors.entry(mentor).or_default();
        }

        if i >= 1000 {
            let left = chars[i - 1000];
            if left.is_ascii_uppercase() {
                mentors.entry(left).and_modify(|counter| *counter -= 1);
            }
        }

        if i + 1001 < chars.len() {
            let right = chars[i + 1001];
            if right.is_ascii_uppercase() {
                mentors
                    .entry(right)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }

    sum
}
