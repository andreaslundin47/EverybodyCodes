use glam::IVec2;
use std::collections::{HashMap, HashSet};

const KNIGHTS_MOVES: [IVec2; 8] = [
    IVec2::new(2, 1),
    IVec2::new(2, -1),
    IVec2::new(-2, 1),
    IVec2::new(-2, -1),
    IVec2::new(1, 2),
    IVec2::new(1, -2),
    IVec2::new(-1, 2),
    IVec2::new(-1, -2),
];

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt").trim();
    let board = Board::parse(input);

    let mut reachable = HashSet::from([board.init_state.dragon]);
    for _turn in 0..4 {
        for d in reachable.clone().into_iter() {
            for delta in KNIGHTS_MOVES.iter() {
                let neighbour = d + delta;
                reachable.insert(neighbour);
            }
        }
    }

    let eaten_sheep = reachable
        .iter()
        .filter(|d_pos| board.init_state.sheep.contains(d_pos))
        .count();

    println!("Part 1. {eaten_sheep}");
}

fn part_two() {
    let input = include_str!("../input2.txt").trim();
    let board = Board::parse(input);

    let mut reachable = HashSet::from([board.init_state.dragon]);
    let mut eaten = HashSet::new();

    for turn in 0..20 {
        let dragon_starts = reachable;
        reachable = HashSet::new();

        for d in dragon_starts {
            for delta in KNIGHTS_MOVES.iter() {
                let neighbour = d + delta;
                reachable.insert(neighbour);
            }
        }

        for d in reachable.iter() {
            if board.shelters.contains(d) {
                continue;
            }

            // Dragon makes a move onto a sheep on dragon's turn
            let possible_sheep = d - (turn + 1) * IVec2::Y;
            if board.init_state.sheep.contains(&possible_sheep) && !eaten.contains(&possible_sheep)
            {
                eaten.insert(possible_sheep);
            }

            // Sheep made a move onto the dragon on sheep's turn
            let possible_sheep = d - turn * IVec2::Y;
            if board.init_state.sheep.contains(&possible_sheep) && !eaten.contains(&possible_sheep)
            {
                eaten.insert(possible_sheep);
            }
        }
    }

    let sheep_eaten = eaten.iter().count();
    println!("Part 2. {sheep_eaten}");
}

fn part_three() {
    let input = include_str!("../input3.txt").trim();
    let board = Board::parse(input);
    let count = board.count_dragon_wins_sequences();
    println!("Part 3. {count}");
}

struct Board {
    width: i32,
    height: i32,
    shelters: Vec<IVec2>,
    init_state: State,
}

impl Board {
    fn parse(s: &str) -> Self {
        let mut board = Board {
            width: 0,
            height: 0,
            shelters: vec![],
            init_state: State {
                dragon: IVec2::ZERO,
                sheep: vec![],
                dragons_move: false,
            },
        };

        for (y, line) in s.lines().enumerate() {
            board.height = y as i32 + 1;
            for (x, c) in line.chars().enumerate() {
                board.width = x as i32 + 1;
                let pos = IVec2::new(x as i32, y as i32);

                match c {
                    'D' => board.init_state.dragon = pos,
                    'S' => board.init_state.sheep.push(pos),
                    '#' => board.shelters.push(pos),
                    _ => {}
                };
            }
        }

        board
    }

    fn inside_board(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    fn count_dragon_wins_sequences(&self) -> usize {
        let mut memo: HashMap<State, usize> = HashMap::new();
        self.count_move_seqs(&self.init_state, &mut memo)
    }

    fn count_move_seqs(&self, state: &State, memo: &mut HashMap<State, usize>) -> usize {
        if let Some(&sequence_count) = memo.get(state) {
            return sequence_count;
        }

        if state.sheep.len() == 0 {
            return 1;
        }

        if state.sheep.iter().all(|s| !self.inside_board(s)) {
            return 0;
        }

        let mut count = 0;

        if state.dragons_move {
            for delta in KNIGHTS_MOVES.iter() {
                let new_dragon = state.dragon + delta;

                if self.inside_board(&new_dragon) {
                    let mut new_state = state.clone();
                    new_state.dragon = new_dragon;
                    new_state.dragons_move = false;
                    if !self.shelters.contains(&new_dragon)
                        && let Some(index) = new_state.sheep.iter().position(|s| s == &new_dragon)
                    {
                        new_state.sheep.remove(index);
                    }
                    count += self.count_move_seqs(&new_state, memo);
                }
            }
        } else {
            let mut no_sheep_moved = true;

            for (i, sheep) in state.sheep.iter().enumerate() {
                if self.inside_board(sheep) {
                    let new_sheep = sheep + IVec2::Y;
                    if new_sheep != state.dragon || self.shelters.contains(&new_sheep) {
                        no_sheep_moved = false;
                        let mut new_state = state.clone();
                        new_state.sheep[i] = new_sheep;
                        new_state.dragons_move = true;
                        count += self.count_move_seqs(&new_state, memo);
                    }
                }
            }

            if no_sheep_moved {
                let mut new_state = state.clone();
                new_state.dragons_move = true;
                count += self.count_move_seqs(&new_state, memo);
            }
        }

        memo.insert(state.clone(), count);
        count
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    dragon: IVec2,
    sheep: Vec<IVec2>,
    dragons_move: bool,
}
