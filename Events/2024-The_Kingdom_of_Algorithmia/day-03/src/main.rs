fn main() {
    let input1 = include_str!("../input1");
    let input2 = include_str!("../input2");
    let input3 = include_str!("../input3");

    let offsets4 = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let offsets8 = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ];

    let mut grid1 = parse(input1);
    let count1 = count(&mut grid1, &offsets4);
    println!("Part 1: {}", count1);

    let mut grid2 = parse(input2);
    let count2 = count(&mut grid2, &offsets4);
    println!("Part 2: {}", count2);

    let mut grid3 = parse(input3);
    let count3 = count(&mut grid3, &offsets8);
    println!("Part 3: {}", count3);
}

fn count(grid: &mut Vec<Vec<usize>>, offsets: &[(i32, i32)]) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut changed = true;

    while changed {
        changed = false;

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let depth = grid[y as usize][x as usize];

                let can_dig = depth > 0
                    && offsets
                        .iter()
                        .all(|(dx, dy)| depth <= grid[(y + dy) as usize][(x + dx) as usize]);

                if can_dig {
                    grid[y as usize][x as usize] += 1;
                }

                changed = changed || can_dig;
            }
        }
    }

    grid.iter().map(|row| row.iter().sum::<usize>()).sum()
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Unexpected char while parsing input"),
                })
                .collect()
        })
        .collect()
}
