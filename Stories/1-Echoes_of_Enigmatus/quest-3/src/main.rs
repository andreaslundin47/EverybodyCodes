use anyhow::{Context, Result};

fn main() -> Result<()> {
    part_one()?;
    part_two()?;
    part_three()?;
    Ok(())
}

fn part_one() -> Result<()> {
    let input = include_str!("../input1.txt");
    let mut snails = parse(input).context("Parsing part 1")?;

    for _day in 0..100 {
        for snail in snails.iter_mut() {
            if snail.y == 1 {
                snail.y = snail.x;
                snail.x = 1;
            } else {
                snail.x += 1;
                snail.y -= 1;
            }
        }
    }

    let snail_sum = snails.iter().map(|s| s.x + 100 * s.y).sum::<usize>();
    println!("Part 1. {}", snail_sum);
    Ok(())
}

fn part_two() -> Result<()> {
    let input = include_str!("../input2.txt");
    let snails = parse(input).context("Parsing part 2.")?;
    println!("Part 2. {}", time_to_sync_snails(&snails));
    Ok(())
}

fn part_three() -> Result<()> {
    let input = include_str!("../input3.txt");
    let snails = parse(input).context("Parsing part 3.")?;
    println!("Part 3. {}", time_to_sync_snails(&snails));
    Ok(())
}

fn time_to_sync_snails(snails: &[Snail]) -> usize {
    if snails.is_empty() {
        return 0;
    }

    let mut time = snails[0].y - 1;
    let mut common_period = snails[0].x + snails[0].y - 1;

    for snail in snails.iter().skip(1) {
        let offset = snail.y - 1;
        let period = snail.x + snail.y - 1;

        // Advance time until new snail and all previous all sync at y == 1
        while time.rem_euclid(period) != offset {
            time += common_period;
        }

        // Include the new snail in the common period
        for reps in 1.. {
            if (reps * common_period).rem_euclid(period) == 0 {
                common_period *= reps;
                break;
            }
        }
    }

    time
}

struct Snail {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Result<Vec<Snail>> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(' ')
                .context("Failed to split line into x and y component.")?;
            let x = x
                .strip_prefix("x=")
                .context("Missing 'x=' prefix")?
                .parse::<usize>()
                .context("Failed to parse x value")?;
            let y = y
                .strip_prefix("y=")
                .context("Missing 'y=' prefix")?
                .parse::<usize>()
                .context("Failed to parse y value")?;

            Ok(Snail { x, y })
        })
        .collect()
}
