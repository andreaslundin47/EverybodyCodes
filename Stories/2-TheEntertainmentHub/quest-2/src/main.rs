use std::collections::VecDeque;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1.txt");
    let mut ballons = input1.chars().collect::<VecDeque<char>>();

    let mut index = 0;
    let mut fluffbolt = Some('R');

    while !ballons.is_empty() {
        if fluffbolt == None {
            index += 1;
            fluffbolt = Some(['R', 'G', 'B'][index % 3]);
        } else {
            let leader = ballons.pop_front();

            if fluffbolt != leader {
                fluffbolt = None;
            }
        }
    }

    let fluffbolts_used = index + 1;

    println!("Part 1. Used: {}", fluffbolts_used);
}

fn part_two() {
    let input2 = include_str!("../input2.txt");
    let input_count = input2.chars().count();

    let mut ballons = input2
        .chars()
        .cycle()
        .take(100 * input_count)
        .collect::<VecDeque<char>>();

    let mut index = 0;
    let mut fluffbolt = Some('R');

    while !ballons.is_empty() {
        if fluffbolt == None {
            index += 1;
            fluffbolt = Some(['R', 'G', 'B'][index % 3]);
            continue;
        }

        let is_even_ring = ballons.len() % 2 == 0;
        let middle_index = (ballons.len() / 2).saturating_sub(1);

        let leader = ballons.pop_front();

        if fluffbolt != leader {
            fluffbolt = None;
            continue;
        }

        if is_even_ring {
            let _middle_ballon = ballons.remove(middle_index);
        }

        fluffbolt = None;
    }

    let fluffbolts_used = index + 1;

    println!("Part 2. Used: {}", fluffbolts_used);
}

fn part_three() {
    let input3 = include_str!("../input3.txt");
    let input_count = input3.chars().count();

    let mut front_half_ballons = input3
        .chars()
        .cycle()
        .take(50_000 * input_count)
        .collect::<VecDeque<char>>();

    let mut back_half_ballons = input3
        .chars()
        .cycle()
        .take(50_000 * input_count)
        .collect::<VecDeque<char>>();

    // Invariant: Between shots, the front and the back are the same length, or
    // the front is exactly 1 ballon longer than the back half

    let mut index = 0;
    let mut fluffbolt = Some('R');

    while !front_half_ballons.is_empty() {
        if fluffbolt == None {
            index += 1;
            fluffbolt = Some(['R', 'G', 'B'][index % 3]);
            continue;
        }

        let is_even_length_ring = front_half_ballons.len() == back_half_ballons.len();

        // Always pop the first ballon
        let leader = front_half_ballons.pop_front();

        if is_even_length_ring {
            if fluffbolt == leader {
                // Fluffbolt survived past the first ballon, and can reach one more
                let _middle = back_half_ballons.pop_front();
            } else {
                // Fluffbolt did not survive past the first ballon, so we just adjust
                // the halves to maintain the invariant
                if let Some(middle) = back_half_ballons.pop_front() {
                    front_half_ballons.push_back(middle);
                }
            }
        }

        fluffbolt = None;
    }

    let fluffbolts_used = index + 1;

    println!("Part 3. Used: {}", fluffbolts_used);
}
