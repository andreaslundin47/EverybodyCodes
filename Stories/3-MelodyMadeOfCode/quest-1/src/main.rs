use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input1 = include_str!("../input1.txt");

    let sum: usize = input1
        .lines()
        .filter_map(|line| dragon_duck::Scale::new(line).id_if_green_color_dominates())
        .sum();

    println!("Part 1. ID sum = {}", sum);
}

fn part_two() {
    let input2 = include_str!("../input2.txt");

    let scales: Vec<dragon_duck::Scale> = input2
        .lines()
        .map(|line| dragon_duck::Scale::new(line))
        .collect();

    let shiniest_scale = scales
        .iter()
        .max_by(|a, b| match a.shine.cmp(&b.shine) {
            std::cmp::Ordering::Equal => b.color_sum().cmp(&a.color_sum()), // tie-breaker selects for the one with darker colour
            other => other,
        })
        .expect("scales should not be empty");

    println!("Part 2. ID of darkest shiny scale = {}", shiniest_scale.id);
}

fn part_three() {
    let input3 = include_str!("../input3.txt");

    let scales: Vec<dragon_duck::Scale> = input3
        .lines()
        .map(|line| dragon_duck::Scale::new(line))
        .collect();

    let mut groups: HashMap<dragon_duck::UniqueCategory, Vec<&dragon_duck::Scale>> = HashMap::new();

    for scale in scales.iter() {
        if let Some(category) = scale.get_category() {
            groups
                .entry(category)
                .and_modify(|e| e.push(scale))
                .or_insert(vec![scale]);
        }
    }

    let largest_group = groups.values().max_by_key(|group| group.len());

    if let Some(group) = largest_group {
        let id_sum: usize = group.iter().map(|scale| scale.id).sum();
        println!("Part 3. Sum of IDs in largest group = {}", id_sum);
    } else {
        println!("Part 3. There is not a single largest group");
    }
}

mod dragon_duck {
    use crate::colors;

    pub struct Scale {
        pub id: usize,
        color: colors::Color,
        pub shine: Option<u8>,
    }

    impl Scale {
        pub fn new(input: &str) -> Self {
            let (id, colors) = input.split_once(':').unwrap();
            let id = id.parse::<usize>().unwrap();
            let parts = colors.split_whitespace().collect::<Vec<&str>>();

            use crate::binary_conversions;

            let red = binary_conversions::to_number(parts[0]);
            let green = binary_conversions::to_number(parts[1]);
            let blue = binary_conversions::to_number(parts[2]);

            let shine = if parts.len() > 3 {
                Some(binary_conversions::to_number(parts[3]))
            } else {
                None
            };

            Scale {
                id,
                color: colors::Color::new(red, green, blue),
                shine,
            }
        }

        pub fn color_sum(&self) -> u8 {
            self.color.color_sum()
        }

        pub fn id_if_green_color_dominates(&self) -> Option<usize> {
            self.color.is_majority_green().then_some(self.id)
        }

        pub fn get_category(&self) -> Option<UniqueCategory> {
            let color = self.color.dominant_color();

            let shininess = match self.shine {
                Some(s) if s < 31 => Shininess::Matte,
                Some(s) if s > 32 => Shininess::Shiny,
                _ => Shininess::Undefined,
            };

            if color != colors::DominantColor::Unclear && shininess != Shininess::Undefined {
                Some(UniqueCategory { color, shininess })
            } else {
                None
            }
        }
    }

    #[derive(PartialEq, Eq, Hash)]
    pub struct UniqueCategory {
        color: colors::DominantColor,
        shininess: Shininess,
    }

    #[derive(PartialEq, Eq, Hash)]
    pub enum Shininess {
        Shiny,
        Matte,
        Undefined,
    }
}

pub mod colors {
    #[derive(PartialEq, Eq, Hash)]
    pub enum DominantColor {
        Red,
        Green,
        Blue,
        Unclear,
    }

    #[derive(Debug)]
    pub struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Color {
        pub fn new(red: u8, green: u8, blue: u8) -> Self {
            Color { red, green, blue }
        }

        pub fn is_majority_green(&self) -> bool {
            self.green > self.red && self.green > self.blue
        }

        pub fn color_sum(&self) -> u8 {
            self.red + self.green + self.blue
        }

        pub fn dominant_color(&self) -> DominantColor {
            match (self.red, self.green, self.blue) {
                (r, g, b) if r > g && r > b => DominantColor::Red,
                (r, g, b) if g > r && g > b => DominantColor::Blue,
                (r, g, b) if b > r && b > g => DominantColor::Green,
                _ => DominantColor::Unclear,
            }
        }
    }
}

mod binary_conversions {
    pub fn to_number(component: &str) -> u8 {
        let mut out = 0;
        for c in component.chars() {
            out = out << 1;
            if c.is_uppercase() {
                out += 1;
            }
        }
        out
    }
}
