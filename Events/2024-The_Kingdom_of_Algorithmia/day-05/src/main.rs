use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    part_one().with_context(|| "Failed during part 1.")?;
    part_two().with_context(|| "Failed during part 2.")?;
    part_three().with_context(|| "Failed during part 3.")?;

    Ok(())
}

fn part_one() -> Result<()> {
    let input1 = include_str!("../input1");
    let mut dance = Dance::parse(input1)?;

    for _ in 0..10 {
        dance.move_dancer()?;
    }

    println!("Part 1. {}", dance.get_number()?);
    Ok(())
}

fn part_two() -> Result<()> {
    let input2 = include_str!("../input2");
    let mut dance = Dance::parse(input2)?;

    let mut counts: HashMap<usize, usize> = HashMap::new();

    loop {
        dance.move_dancer()?;
        let number = dance.get_number()?;
        let count: &mut usize = counts.entry(number).or_default();
        *count += 1;

        if *count == 2024 {
            println!(
                "Part 2. {} at round {}. Prod = {}",
                number,
                dance.get_round(),
                number * dance.get_round(),
            );
            return Ok(());
        }
    }
}

fn part_three() -> Result<()> {
    let input3 = include_str!("../input3");
    let mut dance = Dance::parse(input3)?;

    let mut max_number = dance.get_number()?;
    let mut seen_dances = HashSet::from([dance.grid.clone()]);

    loop {
        dance.move_dancer()?;
        let number = dance.get_number()?;
        max_number = max_number.max(number);
        let is_new_grid = seen_dances.insert(dance.grid.clone());

        if !is_new_grid {
            println!("Part 3. Largest number: {}", max_number);
            return Ok(());
        }
    }
}

struct Dance {
    grid: Vec<VecDeque<usize>>,
    round: usize,
}

impl Dance {
    fn parse(input: &str) -> Result<Self> {
        let mut columns = Vec::new();

        for (i, line) in input.lines().enumerate() {
            let row = line.split_whitespace().map(|n| n.parse::<usize>());

            if i == 0 {
                for n in row {
                    columns.push(VecDeque::from([
                        n.with_context(|| "Could not parse input as usize")?
                    ]));
                }
            } else {
                for (j, n) in row.enumerate() {
                    columns[j].push_back(n.with_context(|| "Could not parse input as usize.")?);
                }
            }
        }

        Ok(Dance {
            grid: columns,
            round: 0,
        })
    }

    fn print(&self) {
        for column in self.grid.iter() {
            for v in column.iter() {
                print!("{} ", *v);
            }
            println!();
        }
        println!();
        println!();
    }

    fn get_round(&self) -> usize {
        self.round
    }

    fn get_number(&self) -> Result<usize> {
        let concat_string: String = self
            .grid
            .iter()
            .map(|col| {
                col.front()
                    .context("A column in the dance is empty.")
                    .map(|n| n.to_string())
            })
            .collect::<Result<String>>()?;

        concat_string
            .parse::<usize>()
            .with_context(|| "Failed to build a number from the dancer in front.")
    }

    fn move_dancer(&mut self) -> Result<()> {
        self.round += 1;

        let columns = self.grid.len();
        let i_from = (self.round - 1) % columns;
        let i_to = (i_from + 1) % columns;

        let dancer = self.grid[i_from]
            .pop_front()
            .context("Trid to take a dancer from a column that is empty.")?;

        let column_len = self.grid[i_to].len();

        let is_moving_down = match column_len == 1 {
            true => dancer % 2 == 1,
            false => ((dancer - 1) / column_len) % 2 == 0,
        };

        let is_at_endpoint = dancer % column_len == 0;

        let insert_index = if is_at_endpoint {
            match is_moving_down {
                true => column_len - 1,
                false => 1,
            }
        } else {
            match is_moving_down {
                true => (dancer - 1) % column_len,
                false => column_len - (dancer % column_len) + 1,
            }
        };

        self.grid[i_to].insert(insert_index, dancer);

        Ok(())
    }
}
