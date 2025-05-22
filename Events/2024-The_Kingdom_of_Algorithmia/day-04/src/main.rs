use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input1 = include_str!("input1");
    let input2 = include_str!("input2");
    let input3 = include_str!("input3");

    let s1 = count_strikes(input1).with_context(|| "While solving part 1")?;
    println!("Part 1. {}", s1);

    let s2 = count_strikes(input2).with_context(|| "While solving part 2")?;
    println!("Part 2. {}", s2);

    let s3 = count_strikes_advanced(input3).with_context(|| "While solving part 3")?;
    println!("Part 3. {}", s3);

    Ok(())
}

fn count_strikes(input: &str) -> Result<usize> {
    let nails = parse(input).with_context(|| "Parsing input file")?;
    let target: usize = nails
        .iter()
        .min()
        .copied()
        .with_context(|| "Taking min from empty nails list.")?;
    Ok(nails.iter().map(|n| n.abs_diff(target)).sum())
}

fn count_strikes_advanced(input: &str) -> Result<usize> {
    let nails = parse(input).with_context(|| "Parsing input file")?;

    let strikes = nails
        .iter()
        .map(|target| nails.iter().map(|n| n.abs_diff(*target)).sum())
        .min()
        .with_context(|| "Taking min number of strikes")?;

    Ok(strikes)
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .with_context(|| "Could not parse a row in input into a number")
        })
        .collect()
}
