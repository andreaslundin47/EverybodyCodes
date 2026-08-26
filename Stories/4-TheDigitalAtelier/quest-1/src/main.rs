use std::{collections::HashSet, ops::RangeInclusive};

fn main() {
    let inputs = [
        include_str!("../input1.txt"),
        include_str!("../input2.txt"),
        include_str!("../input3.txt"),
    ];

    let rules = [jump_rule_one, jump_rule_two, jump_rule_three];

    for (i, (input, rule)) in inputs.iter().zip(rules).enumerate() {
        let steps = parse(input.trim());
        let sum: i32 = steps.iter().map(|steps| rule(steps)).sum();
        println!("Part {}. Sum = {}", i + 1, sum);
    }
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| line.split(',').map(|v| v.parse::<i32>().unwrap()).collect())
        .collect()
}

fn jump_rule_one(steps: &[i32]) -> i32 {
    let mut current = 0;
    let mut visited = HashSet::<i32>::from([0]);

    for step in steps.iter() {
        let back = current - step;

        if back >= 0 && !visited.contains(&back) {
            current = back;
            visited.insert(back);
        } else {
            current = current + step;
            visited.insert(current);
        }
    }

    current
}

fn jump_rule_two(steps: &[i32]) -> i32 {
    let mut current = 0;
    let mut visited = HashSet::<i32>::from([0]);

    for step in steps.iter() {
        let back = current - step;

        if back >= 0 && !visited.contains(&back) {
            current = back;
            visited.insert(back);
        } else {
            current = current + step;

            while visited.contains(&current) {
                current += 1;
            }

            visited.insert(current);
        }
    }

    current
}

fn jump_rule_three(steps: &[i32]) -> i32 {
    let mut current = 0;
    let mut visited = HashSet::<i32>::from([0]);
    let mut arcs: Vec<JumpArc> = vec![];
    let mut current_jump_upper = false;

    'step_loop: for step in steps.iter() {
        let back = current - step;

        let no_backward_arc_crossings: bool = arcs
            .iter()
            .all(|a| a.crossing(current, back, current_jump_upper) == CrossingType::None);

        if back >= 0 && !visited.contains(&back) && no_backward_arc_crossings {
            arcs.push(JumpArc::new(current, back, current_jump_upper));
            current = back;
            visited.insert(back);
            current_jump_upper = !current_jump_upper;
        } else {
            let mut forward = current + step;

            while visited.contains(&forward) {
                forward += 1;
            }

            'arc_loop: loop {
                for arc in arcs.iter() {
                    match arc.crossing(current, forward, current_jump_upper) {
                        CrossingType::OutOfArc => continue 'step_loop,
                        CrossingType::IntoArc => {
                            forward = arc.range.end() + 1;
                            while visited.contains(&forward) {
                                forward += 1;
                            }
                            continue 'arc_loop;
                        }
                        CrossingType::None => (),
                    }
                }

                arcs.push(JumpArc::new(current, forward, current_jump_upper));
                current = forward;
                visited.insert(forward);
                current_jump_upper = !current_jump_upper;
                break;
            }
        }
    }

    current
}

#[derive(Eq, PartialEq)]
enum CrossingType {
    None,
    OutOfArc,
    IntoArc,
}

struct JumpArc {
    range: RangeInclusive<i32>,
    upper: bool,
}

impl JumpArc {
    fn new(start: i32, end: i32, upper: bool) -> Self {
        let range = start.min(end)..=start.max(end);
        Self { range, upper }
    }

    fn crossing(&self, start: i32, end: i32, upper: bool) -> CrossingType {
        if self.upper != upper {
            return CrossingType::None;
        }

        let s_in = self.range.contains(&start);
        let e_in = self.range.contains(&end);

        if s_in && !e_in {
            return CrossingType::OutOfArc;
        }

        if !s_in && e_in {
            return CrossingType::IntoArc;
        }

        CrossingType::None
    }
}
