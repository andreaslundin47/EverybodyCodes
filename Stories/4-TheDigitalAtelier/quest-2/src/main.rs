use glam::IVec2;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::{
        complete::{anychar, char, i32 as nom_i32, line_ending},
        streaming::alpha1,
    },
    combinator::opt,
    multi::many0,
    sequence::{delimited, preceded, separated_pair, terminated},
};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let raw_input = include_str!("../input1.txt");
    let config = config_parser
        .parse(raw_input)
        .expect("Failed to parse input")
        .1;
    let count = beetles(&config).len();
    println!("Part 1. {}", count);

    let raw_input = include_str!("../input2.txt");
    let config = config_parser
        .parse(raw_input)
        .expect("Failed to parse input")
        .1;
    let beatles = beetles(&config);
    let count = fireflies(&beatles).len();
    println!("Part 2. {}", count);

    let raw_input = include_str!("../input3.txt");
    let config = config_parser
        .parse(raw_input)
        .expect("Failed to parse input")
        .1;
    let beatles = beetles(&config);
    let count = fireflies(&beatles).len();
    println!("Part 3. {}", count);
}

fn midpoint(a: &IVec2, b: &IVec2) -> IVec2 {
    let x = (a.x + b.x) / 2;
    let y = (a.y + b.y) / 2;

    IVec2::new(x, y)
}

fn beetles(config: &Config) -> HashSet<IVec2> {
    let mut beetles = HashSet::from([config.start]);

    if let Some(moves) = config.moves.as_deref() {
        // Use specified sequence of moves
        let mut position = config.start;

        for beacon in moves {
            if let Some(dest) = config.beacons.get(&beacon) {
                position = midpoint(&position, dest);
                beetles.insert(position);
            }
        }
    } else {
        // Use BFS to explore all possible moves
        let mut queue = VecDeque::from([config.start]);

        while let Some(current_pos) = queue.pop_front() {
            for beacon in config.beacons.values() {
                let new_pos = midpoint(&current_pos, beacon);

                if !beetles.contains(&new_pos) {
                    beetles.insert(new_pos);
                    queue.push_back(new_pos);
                }
            }
        }
    }

    beetles
}

fn fireflies(beatles: &HashSet<IVec2>) -> HashSet<IVec2> {
    let mut fireflies = HashSet::new();

    for beatle in beatles {
        for offset in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
            let firefly = beatle + offset;

            if !beatles.contains(&firefly) {
                fireflies.insert(firefly);
            }
        }
    }

    fireflies
}

struct Config {
    start: IVec2,
    beacons: HashMap<char, IVec2>,
    moves: Option<Vec<char>>,
}

fn coordinate(input: &str) -> IResult<&str, IVec2> {
    let (i, (x, y)) = delimited(
        char('['),
        separated_pair(nom_i32, char(','), nom_i32),
        char(']'),
    )
    .parse(input)?;

    Ok((i, IVec2::new(x, y)))
}

fn beacon(input: &str) -> IResult<&str, (char, IVec2)> {
    separated_pair(anychar, char('='), coordinate).parse(input)
}

fn start(input: &str) -> IResult<&str, IVec2> {
    terminated(preceded(tag("START="), coordinate), line_ending).parse(input)
}

fn moves(input: &str) -> IResult<&str, Option<Vec<char>>> {
    opt(preceded(tag("MOVES="), alpha1).map(|s: &str| s.chars().collect())).parse(input)
}

fn beacons(input: &str) -> IResult<&str, HashMap<char, IVec2>> {
    let (i, beacons) = many0(terminated(beacon, line_ending)).parse(input)?;
    Ok((i, beacons.into_iter().collect()))
}

fn config_parser(input: &str) -> IResult<&str, Config> {
    let (input, start) = start(input)?;
    let (input, beacons) = beacons(input)?;
    let (input, moves) = moves.parse(input)?;

    Ok((
        input,
        Config {
            start,
            beacons,
            moves,
        },
    ))
}
