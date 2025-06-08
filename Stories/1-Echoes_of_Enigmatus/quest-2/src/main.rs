use std::cmp::Ordering;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let input1 = include_str!("../input1.txt");
    let input2 = include_str!("../input2.txt");
    let input3 = include_str!("../input3.txt");

    println!("Part 1: {}", run(input1, SwapMode::Shallow));
    println!("Part 2: {}", run(input2, SwapMode::Shallow));
    println!("Part 3: {}", run(input3, SwapMode::Deep));
}

fn run(input: &str, mode: SwapMode) -> String {
    let commands = parse(input);

    let mut tree1 = Tree::default();
    let mut tree2 = Tree::default();

    commands.into_iter().for_each(|command| match command {
        Command::Add {
            id,
            left_rank,
            left_symbol,
            right_rank,
            right_symbol,
        } => {
            tree1.add(id, left_rank, left_symbol);
            tree2.add(id, right_rank, right_symbol);
        }
        Command::Swap { id } => {
            tree1.swap(&mut tree2, id, mode);
        }
    });

    format!("{}{}", tree1.get_level(), tree2.get_level())
}

enum Command<'a> {
    Add {
        id: usize,
        left_rank: usize,
        left_symbol: &'a str,
        right_rank: usize,
        right_symbol: &'a str,
    },
    Swap {
        id: usize,
    },
}

fn parse(input: &str) -> Vec<Command> {
    fn parse_node_data(s: &str) -> (usize, &str) {
        let s = s.strip_prefix("[").unwrap_or(s);
        let s = s.strip_suffix("]").unwrap_or(s);
        let (rank, symbol) = s.split_once(',').expect("Invalid side format");
        (rank.trim().parse().expect("Invalid rank"), symbol.trim())
    }

    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts.as_slice() {
                ["ADD", id, left, right] => {
                    let id = id.strip_prefix("id=").unwrap().parse().unwrap();
                    let left = left.strip_prefix("left=").unwrap();
                    let right = right.strip_prefix("right=").unwrap();
                    let (left_rank, left_symbol) = parse_node_data(left);
                    let (right_rank, right_symbol) = parse_node_data(right);
                    Command::Add {
                        id,
                        left_rank,
                        left_symbol,
                        right_rank,
                        right_symbol,
                    }
                }
                ["SWAP", id] => {
                    let id = id.parse().unwrap();
                    Command::Swap { id }
                }
                _ => panic!("Invalid operation: {line}"),
            }
        })
        .collect()
}

#[derive(Debug, Default)]
struct Tree {
    root: Option<Rc<RefCell<Node>>>,
}

impl Tree {
    fn add(&mut self, id: usize, rank: usize, symbol: &str) {
        let n = Rc::new(RefCell::new(Node {
            id,
            rank,
            symbol: symbol.to_string(),
            left: None,
            right: None,
        }));

        self.insert(n);
    }
    fn insert(&mut self, new_node: Rc<RefCell<Node>>) {
        if let Some(root) = &mut self.root {
            root.borrow_mut().insert(new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn find_id(&self, id: usize) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = vec![];

        if let Some(root) = &self.root {
            if root.borrow().id == id {
                nodes.push(Rc::clone(root));
            }

            nodes.extend(root.borrow().find_id(id));
        }

        nodes
    }

    fn swap(&mut self, other: &mut Self, id: usize, mode: SwapMode) {
        let nodes = [self.find_id(id), other.find_id(id)].concat();
        assert!(nodes.len() == 2);

        let (mut a, mut b) = (nodes[0].borrow_mut(), nodes[1].borrow_mut());
        a.swap(&mut b, mode);
    }

    fn get_level(&self) -> String {
        let mut levels = HashMap::<usize, String>::new();

        fn helper(
            current: &Option<Rc<RefCell<Node>>>,
            lvl: usize,
            levels: &mut HashMap<usize, String>,
        ) {
            if let Some(node) = current {
                levels
                    .entry(lvl)
                    .or_default()
                    .push_str(node.borrow().symbol.as_str());

                helper(&node.borrow().left, lvl + 1, levels);
                helper(&node.borrow().right, lvl + 1, levels);
            }
        }

        helper(&self.root, 0, &mut levels);

        levels
            .into_iter()
            .max_by_key(|(lvl_dist, lvl_str)| (lvl_str.len(), -(*lvl_dist as i64)))
            .unwrap()
            .1
    }
}

#[derive(Clone, Copy)]
enum SwapMode {
    Shallow,
    Deep,
}

#[derive(Debug)]
struct Node {
    id: usize,
    rank: usize,
    symbol: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn insert(&mut self, new_node: Rc<RefCell<Node>>) {
        let new_rank = new_node.borrow().rank;

        match new_rank.cmp(&self.rank) {
            Ordering::Less => {
                if let Some(left) = &self.left {
                    left.borrow_mut().insert(new_node);
                } else {
                    self.left = Some(new_node);
                }
            }
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    right.borrow_mut().insert(new_node);
                } else {
                    self.right = Some(new_node);
                }
            }

            _ => (),
        }
    }

    fn find_id(&self, id: usize) -> Vec<Rc<RefCell<Node>>> {
        let mut nodes = vec![];

        if let Some(left) = &self.left {
            if left.borrow().id == id {
                nodes.extend(vec![Rc::clone(left)]);
            } else {
                nodes.extend(left.borrow().find_id(id));
            }
        }

        if let Some(right) = &self.right {
            if right.borrow().id == id {
                nodes.extend(vec![Rc::clone(right)]);
            } else {
                nodes.extend(right.borrow().find_id(id));
            }
        }

        nodes
    }

    fn swap(&mut self, other: &mut Self, mode: SwapMode) {
        std::mem::swap(&mut self.rank, &mut other.rank);
        std::mem::swap(&mut self.symbol, &mut other.symbol);

        if let SwapMode::Deep = mode {
            std::mem::swap(&mut self.left, &mut other.left);
            std::mem::swap(&mut self.right, &mut other.right);
        }
    }
}
