use anyhow::{Context, Result, bail};
use glam::IVec2;
use itertools::{Itertools, Position};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Device {
    Plus,
    Minus,
    Equal,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Segment {
    Plus,
    Minus,
    Equal,
    S,
}

type Plan = Vec<Device>;

struct Race {
    id: String,
    devices: Plan,
}

impl Race {
    fn score(&self) -> i32 {
        let mut acc = 10;

        self.devices
            .iter()
            .map(|device| {
                acc = std::cmp::Ord::max(
                    0,
                    acc + match device {
                        Device::Plus => 1,
                        Device::Minus => -1,
                        Device::Equal => 0,
                    },
                );
                acc
            })
            .sum()
    }

    fn score_on_track(&self, track: &[Segment], repeats: usize) -> i64 {
        let mut acc = 10;
        let full_track = track.iter().cycle().take(track.len() * repeats);

        std::iter::zip(full_track, self.devices.iter().cycle())
            .map(|(seg, dev)| {
                acc = std::cmp::Ord::max(
                    0,
                    acc + match seg {
                        Segment::Plus => 1,
                        Segment::Minus => -1,
                        Segment::Equal | Segment::S => match dev {
                            Device::Plus => 1,
                            Device::Minus => -1,
                            Device::Equal => 0,
                        },
                    },
                );
                acc
            })
            .sum()
    }
}

fn main() -> Result<()> {
    part_one().context("Failed to run part one")?;
    part_two().context("Failed to run part two")?;
    part_three().context("Failed to run part three")?;
    Ok(())
}

fn part_one() -> Result<()> {
    let input1 = include_str!("../input1.txt");
    let races = parse(input1).context("Failed to parse input")?;
    let results = races
        .iter()
        .map(|race| (race.id.clone(), race.score()))
        .collect::<Vec<_>>();

    let order = results
        .iter()
        .sorted_by_key(|(_, score)| *score)
        .rev()
        .map(|x| x.0.as_str())
        .collect::<String>();

    println!("Part 1: {}", order);
    Ok(())
}

fn part_two() -> Result<()> {
    let races_input = include_str!("../input2.txt");
    let track_input = include_str!("../track2.txt");

    let races = parse(races_input).context("Failed to parse input")?;
    let track = parse_rect_track(track_input);

    let results: Vec<(&str, i64)> = races
        .iter()
        .map(|race| (race.id.as_str(), race.score_on_track(&track, 10)))
        .collect();

    let order = results
        .iter()
        .sorted_by_key(|(_, score)| *score)
        .rev()
        .map(|res| res.0)
        .collect::<String>();

    println!("Part 2. {}", order);
    Ok(())
}

fn part_three() -> Result<()> {
    let races_input = include_str!("../input3.txt");
    let track_input = include_str!("../track3.txt");

    let races = parse(races_input).context("Failed to parse input")?;
    let track = parse_curvy_track(track_input);
    let score_to_beat = races[0].score_on_track(&track, 2024);

    let all_plans = get_valid_action_plans();

    let count = all_plans
        .into_iter()
        .map(|plan| {
            let race = Race {
                id: "".to_owned(),
                devices: plan,
            };

            race.score_on_track(&track, 2024)
        })
        .filter(|score| *score > score_to_beat)
        .count();

    println!("Part 3. {}", count);

    Ok(())
}

fn get_valid_action_plans() -> Vec<Plan> {
    fn generate(current: Plan, p: usize, m: usize, e: usize) -> Vec<Plan> {
        let mut plans: Vec<Plan> = vec![];

        if (p, m, e) == (0, 0, 0) {
            return vec![current];
        }

        if p > 0 {
            let ncurrent = [current.clone(), vec![Device::Plus]].concat();
            plans.extend(generate(ncurrent, p - 1, m, e));
        }

        if m > 0 {
            let ncurrent = [current.clone(), vec![Device::Minus]].concat();
            plans.extend(generate(ncurrent, p, m - 1, e));
        }

        if e > 0 {
            let ncurrent = [current.clone(), vec![Device::Equal]].concat();
            plans.extend(generate(ncurrent, p, m, e - 1));
        }

        plans
    }

    generate(vec![], 5, 3, 3)
}

fn parse(input: &str) -> Result<Vec<Race>> {
    input
        .lines()
        .map(|line| {
            let (id, device_list) = line
                .split_once(':')
                .with_context(|| format!("Missing ':' in line: {}", line))?;
            let devices = device_list
                .split(',')
                .map(|device| match device {
                    "+" => Ok(Device::Plus),
                    "-" => Ok(Device::Minus),
                    "=" => Ok(Device::Equal),
                    _ => bail!("Unknown device symbol: {}", device),
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Race {
                id: id.to_owned(),
                devices,
            })
        })
        .collect()
}

fn parse_rect_track(input: &str) -> Vec<Segment> {
    let mut track = vec![];
    let mut left_col = vec![];
    let mut right_col = vec![];

    for (pos, line) in input.lines().with_position() {
        match pos {
            Position::First => {
                track.extend(line.chars().skip(1));
                left_col.push(line.chars().next().unwrap());
            }
            Position::Middle => {
                let mut chars = line.chars().filter(|c| !c.is_whitespace());
                left_col.push(chars.next().unwrap());
                right_col.push(chars.next().unwrap());
            }
            Position::Last => {
                track.extend(right_col.clone());
                track.extend(line.chars().rev());
                track.extend(left_col.clone().into_iter().rev());
            }
            Position::Only => panic!("Track should not be a single row!"),
        }
    }

    track
        .into_iter()
        .map(|c| match c {
            '+' => Segment::Plus,
            '-' => Segment::Minus,
            '=' => Segment::Equal,
            'S' => Segment::S,
            _ => panic!("Unexpected char in track!"),
        })
        .collect()
}

fn parse_curvy_track(input: &str) -> Vec<Segment> {
    let mut grid = HashMap::<IVec2, Segment>::new();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            if !c.is_whitespace() {
                let seg = match c {
                    '+' => Segment::Plus,
                    '-' => Segment::Minus,
                    '=' => Segment::Equal,
                    'S' => Segment::S,
                    _ => panic!("Unexpected char in track!"),
                };

                grid.insert(
                    IVec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    seg,
                );
            }
        });
    });

    let mut track = Vec::<Segment>::new();

    let mut cur = IVec2::new(0, 0);
    let mut dir = IVec2::X;

    loop {
        let next_dirs = [dir, dir.rotate(IVec2::NEG_Y), dir.rotate(IVec2::Y)];

        if let Some((d, Some(v))) = next_dirs
            .map(|d| (d, grid.get(&(cur + d))))
            .into_iter()
            .find(|(_, v)| v.is_some())
        {
            track.push(*v);

            cur = cur + d;
            dir = d;

            if *v == Segment::S {
                break;
            }
        } else {
            panic!("Could not find next step along the track!");
        }
    }

    track
}
