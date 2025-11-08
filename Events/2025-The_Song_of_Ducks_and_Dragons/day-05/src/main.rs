use itertools::Itertools;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt");
    let sword = Sword::new(input);
    let quality = sword.read_spine();
    println!("Part 1. {quality}");
}

fn part_two() {
    let input = include_str!("../input2.txt");
    let sword_qualities: Vec<i64> = input
        .lines()
        .map(|line| {
            let sword = Sword::new(line);
            sword.read_spine()
        })
        .collect();

    let min = sword_qualities.iter().min().unwrap();
    let max = sword_qualities.iter().max().unwrap();

    let diff = max - min;
    println!("Part 2. {diff}");
}

fn part_three() {
    let input = include_str!("../input3.txt");

    let swords_sorted: Vec<Sword> = input
        .lines()
        .map(|line| Sword::new(line))
        .sorted_by_key(|sw| sw.sort_key())
        .rev()
        .collect();

    let checksum: usize = swords_sorted
        .iter()
        .enumerate()
        .map(|(i, sword)| (i + 1) * sword.id)
        .sum();

    println!("Part 3. {checksum}");
}

struct Segment {
    left: Option<i32>,
    value: i32,
    right: Option<i32>,
}

impl Segment {
    fn new(value: i32) -> Self {
        Segment {
            left: None,
            value,
            right: None,
        }
    }
    fn insert(&mut self, v: i32) -> bool {
        if v < self.value && self.left.is_none() {
            self.left = Some(v);
            return true;
        }
        if v > self.value && self.right.is_none() {
            self.right = Some(v);
            return true;
        }

        false
    }

    fn score(&self) -> i64 {
        let mut out = vec![];
        if let Some(ll) = self.left {
            out.push(ll);
        }
        out.push(self.value);
        if let Some(rr) = self.right {
            out.push(rr);
        }

        out.into_iter().join("").parse::<i64>().unwrap()
    }
}

struct Sword {
    id: usize,
    fishbone: Vec<Segment>,
}

impl Sword {
    fn new(input: &str) -> Self {
        let (id, values) = input.trim().split_once(':').unwrap();
        let id = id.parse::<usize>().unwrap();
        let values_iter = values.split(',').map(|v| v.parse::<i32>().unwrap());

        let mut sword = Sword {
            id,
            fishbone: vec![],
        };

        for v in values_iter {
            sword.insert(v);
        }

        sword
    }

    fn insert(&mut self, v: i32) {
        for segment in self.fishbone.iter_mut() {
            if segment.insert(v) {
                return;
            }
        }
        self.fishbone.push(Segment::new(v));
    }

    fn read_spine(&self) -> i64 {
        self.fishbone
            .iter()
            .map(|s| s.value)
            .join("")
            .parse::<i64>()
            .unwrap()
    }

    fn sort_key(&self) -> (i64, Vec<i64>, usize) {
        let quality = self.read_spine();
        let segment_scores = self.fishbone.iter().map(|seg| seg.score()).collect();

        (quality, segment_scores, self.id)
    }
}
