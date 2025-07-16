fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let stamps = [1, 3, 5, 10];
    let input = include_str!("../input1.txt");
    let brightnesses = parse(input);
    let top = *brightnesses.iter().max().unwrap();

    let table = LookUp::new(&stamps, top);
    let beetles: usize = brightnesses.iter().map(|&b| table.beetles(b)).sum();
    println!("Part 1. Beetles: {}", beetles);
}

fn part_two() {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    let input = include_str!("../input2.txt");
    let brightnesses = parse(input);
    let top = *brightnesses.iter().max().unwrap();

    let table = LookUp::new(&stamps, top);
    let beetles: usize = brightnesses.iter().map(|&b| table.beetles(b)).sum();
    println!("Part 2. Beetles: {}", beetles);
}

fn part_three() {
    let stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    let input = include_str!("../input3.txt");
    let brightnesses = parse(input);
    let top = *brightnesses.iter().max().unwrap();
    let table = LookUp::new(&stamps, top);

    let beetles: usize = brightnesses.iter().map(|&b| table.dual_beetles(b)).sum();

    println!("Part 3. Beetles: {}", beetles);
}

struct LookUp {
    table: Vec<usize>,
}

impl LookUp {
    fn new(stamps: &[usize], highest: usize) -> Self {
        let mut table: Vec<usize> = vec![highest; highest + 1];

        table[0] = 0;

        for &s in stamps {
            for i in s..=highest {
                table[i] = table[i].min(1 + table[i - s]);
            }
        }

        LookUp { table }
    }

    fn beetles(&self, brightness: usize) -> usize {
        self.table[brightness]
    }

    fn dual_beetles(&self, brightness: usize) -> usize {
        let mut min_beetles = usize::MAX;
        let middle = brightness / 2;

        for i in middle..middle + 50 {
            min_beetles = min_beetles.min(self.table[i] + self.table[brightness - i]);
        }

        min_beetles
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}
