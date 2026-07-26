use glam::IVec2;
use itertools::Itertools;

use std::collections::HashSet;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1.txt").trim();
    let stars = parse_stars(input);
    let size = constellation_size(&stars);
    println!("Part 1. Size = {}", size);
}

fn part_two() {
    let input = include_str!("../input2.txt").trim();
    let stars = parse_stars(input);
    let size = constellation_size(&stars);
    println!("Part 2. Size = {}", size);
}

fn part_three() {
    let input = include_str!("../input3.txt").trim();
    let stars = parse_stars(input);

    let constellations = get_constellations(&stars);

    // Product of 3 brightest constellations
    let product: usize = constellations
        .iter()
        .map(|constellation| constellation_size(constellation))
        .sorted()
        .rev()
        .take(3)
        .product();

    println!("Part 3. Product = {}", product);
}

fn parse_stars(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '*').then_some(IVec2::new(x as i32, y as i32)))
        })
        .collect()
}

fn get_constellations(stars: &[IVec2]) -> Vec<Vec<IVec2>> {
    let mut union = union_find::Union::new(stars.len());

    for ((i, s1), (j, s2)) in stars
        .iter()
        .enumerate()
        .cartesian_product(stars.iter().enumerate())
    {
        if s1.manhattan_distance(*s2) < 6 {
            union.union(i, j);
        }
    }

    union
        .get_groups()
        .into_iter()
        .map(|group| group.iter().map(|&i| stars[i]).collect())
        .collect()
}

fn constellation_size(stars: &[IVec2]) -> usize {
    // Use Prim's algoritm to minimize constellation size

    let mut constellation_size: usize = 0;
    let mut remaining_stars = stars.iter().skip(1).copied().collect::<HashSet<IVec2>>();
    let mut included_stars = HashSet::from([stars[0]]);

    while !remaining_stars.is_empty() {
        let (best_dist, new_star) = remaining_stars
            .iter()
            .cartesian_product(included_stars.iter())
            .map(|(new_star, old_star)| (old_star.manhattan_distance(*new_star), *new_star))
            .min_by_key(|pair| pair.0)
            .expect("Should have at least one pair to use");

        remaining_stars.remove(&new_star);
        included_stars.insert(new_star);

        constellation_size += best_dist as usize;
    }

    constellation_size + stars.len()
}

mod union_find {
    pub struct Union {
        parent: Vec<usize>,
    }

    impl Union {
        pub fn new(len: usize) -> Self {
            Union {
                parent: (0..len).collect(),
            }
        }

        pub fn find(&mut self, a: usize) -> usize {
            if self.parent[a] != a {
                self.parent[a] = self.find(self.parent[a]);
            }

            return self.parent[a];
        }

        pub fn union(&mut self, a: usize, b: usize) {
            let pa = self.find(a);
            let pb = self.find(b);

            if pa != pb {
                self.parent[pb] = self.parent[pa];
            }
        }

        pub fn get_groups(&mut self) -> Vec<Vec<usize>> {
            use std::collections::HashMap;

            let mut groups: HashMap<usize, Vec<usize>> = HashMap::new();

            for i in 0..self.parent.len() {
                let parent = self.find(i);

                groups
                    .entry(parent)
                    .and_modify(|gr| gr.push(i))
                    .or_insert(vec![i]);
            }

            groups.into_values().collect()
        }
    }
}
