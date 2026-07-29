use std::str::FromStr;

fn main() {
    part_one();
    part_two();
    part_three();
}

// ########################################################################
//
fn parse(s: &str) -> Vec<Node> {
    s.lines().map(|line| Node::new(line)).collect()
}

// ########################################################################
//
fn part_one() {
    let input = include_str!("../input1.txt");
    let nodes = parse(input);
    let mut nodes_iter = nodes.into_iter();

    let mut root = nodes_iter.next().unwrap();

    for node in nodes_iter {
        let next = &mut Some(Box::new(node));
        root.insert_strong(next);
    }

    let mut ids: Vec<usize> = vec![];

    root.checksum(&mut ids);

    let checksum: usize = ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (i + 1) * id)
        .sum();

    println!("Part 1. Checksum: {}", checksum);
}

// ########################################################################
//
fn part_two() {
    let input = include_str!("../input2.txt");
    let nodes = parse(input);
    let mut nodes_iter = nodes.into_iter();

    let mut root = nodes_iter.next().unwrap();

    for node in nodes_iter {
        let next = &mut Some(Box::new(node));
        root.insert_weak(next);
    }

    let mut ids: Vec<usize> = vec![];

    root.checksum(&mut ids);

    let checksum: usize = ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (i + 1) * id)
        .sum();

    println!("Part 2. Checksum: {}", checksum);
}

// ########################################################################
//
fn part_three() {
    let input = include_str!("../input3.txt");
    let nodes = parse(input);
    let mut nodes_iter = nodes.into_iter();

    let mut root = nodes_iter.next().unwrap();

    for node in nodes_iter {
        let next = &mut Some(Box::new(node));

        while next.is_some() {
            root.insert_with_replace(next);
        }
    }

    let mut ids: Vec<usize> = vec![];

    root.checksum(&mut ids);

    let checksum: usize = ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (i + 1) * id)
        .sum();

    println!("Part 3. Checksum: {}", checksum);
}

// ########################################################################

#[derive(Debug, Eq, PartialEq, Clone)]
struct Connection {
    color: String,
    shape: String,
}

impl Connection {
    fn strong_match(&self, other: &Self) -> bool {
        self == other
    }

    fn weak_match(&self, other: &Self) -> bool {
        (self.color == other.color && self.shape != other.shape)
            || (self.color != other.color && self.shape == other.shape)
    }

    fn any_match(&self, other: &Self) -> bool {
        self.weak_match(other) || self.strong_match(other)
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!("Expected 2 words, got {}", parts.len()));
        }
        Ok(Connection {
            color: parts[0].to_owned(),
            shape: parts[1].to_owned(),
        })
    }
}

// ########################################################################

#[derive(Debug)]
struct Node {
    id: usize,
    plug: Connection,
    left_socket: Connection,
    left: Option<Box<Node>>,
    right_socket: Connection,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(s: &str) -> Node {
        let parts: Vec<&str> = s.trim().split(", ").collect();

        let id = parts[0]
            .strip_prefix("id=")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let plug = parts[1].strip_prefix("plug=").unwrap().parse().unwrap();

        let left_socket = parts[2]
            .strip_prefix("leftSocket=")
            .unwrap()
            .parse()
            .unwrap();

        let right_socket = parts[3]
            .strip_prefix("rightSocket=")
            .unwrap()
            .parse()
            .unwrap();

        Node {
            id,
            plug,
            left_socket,
            left: None,
            right_socket,
            right: None,
        }
    }

    // ########################################################################

    fn checksum(&self, checksum: &mut Vec<usize>) {
        self.left.as_ref().map(|left| left.checksum(checksum));
        checksum.push(self.id);
        self.right.as_ref().map(|right| right.checksum(checksum));
    }

    // ########################################################################

    fn insert_strong(&mut self, next: &mut Option<Box<Node>>) {
        let Some(next_node) = next.as_mut() else {
            return;
        };

        let plug = &next_node.plug.clone();

        if self.left.is_none() && self.left_socket.strong_match(plug) {
            std::mem::swap(&mut self.left, next);
            return;
        }

        if let Some(left_node) = self.left.as_mut() {
            left_node.insert_strong(next);
        }

        if next.is_none() {
            return;
        }

        if self.right.is_none() && self.right_socket.strong_match(plug) {
            std::mem::swap(&mut self.right, next);
            return;
        }

        if let Some(right_node) = self.right.as_mut() {
            right_node.insert_strong(next);
        }
    }

    // ########################################################################

    fn insert_weak(&mut self, next: &mut Option<Box<Node>>) {
        let Some(next_node) = next.as_mut() else {
            return;
        };

        let plug = &next_node.plug.clone();

        if self.left.is_none() && self.left_socket.any_match(&plug) {
            std::mem::swap(&mut self.left, next);
            return;
        }

        if let Some(left_node) = self.left.as_mut() {
            left_node.insert_weak(next);
        }

        if next.is_none() {
            return;
        }

        if self.right.is_none() && self.right_socket.any_match(&plug) {
            std::mem::swap(&mut self.right, next);
            return;
        }

        if let Some(right_node) = self.right.as_mut() {
            right_node.insert_weak(next);
        }
    }

    // ########################################################################

    fn insert_with_replace(&mut self, next: &mut Option<Box<Node>>) {
        let Some(next_node) = next.as_mut() else {
            return;
        };

        let mut plug = next_node.plug.clone();

        if self.left.is_none() && self.left_socket.any_match(&plug) {
            std::mem::swap(&mut self.left, next);
            return;
        }

        if let Some(left_node) = self.left.as_ref() {
            if self.left_socket.weak_match(&left_node.plug) && self.left_socket.strong_match(&plug)
            {
                plug = left_node.plug.clone();
                std::mem::swap(&mut self.left, next);
            } else if let Some(left) = &mut self.left {
                left.insert_with_replace(next);
            }
        }

        if next.is_none() {
            return;
        }

        if self.right.is_none() && self.right_socket.any_match(&plug) {
            std::mem::swap(&mut self.right, next);
            return;
        }

        if let Some(right_node) = self.right.as_ref() {
            if self.right_socket.weak_match(&right_node.plug)
                && self.right_socket.strong_match(&plug)
            {
                std::mem::swap(&mut self.right, next);
            } else if let Some(right) = self.right.as_mut() {
                right.insert_with_replace(next);
            }
        }
    }
}

// ########################################################################
