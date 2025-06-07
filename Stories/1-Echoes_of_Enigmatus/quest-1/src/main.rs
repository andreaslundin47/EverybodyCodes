use std::collections::HashMap;

fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    let input = include_str!("../input1");
    let rows = parse_rows(input);
    let max: usize = rows
        .iter()
        .map(|row| sum_triple(row, eni_1))
        .max()
        .expect("A maximum available");
    println!("Part 1. {}", max);
}

fn part_two() {
    let input = include_str!("../input2");
    let rows = parse_rows(input);
    let max: usize = rows
        .iter()
        .map(|row| sum_triple(row, eni_2))
        .max()
        .expect("A maximum available");
    println!("Part 2. {}", max);
}

fn part_three() {
    let input = include_str!("../input3");
    let rows = parse_rows(input);
    let max: usize = rows
        .iter()
        .map(|row| sum_triple(row, eni_3))
        .max()
        .expect("A maximum available");
    println!("Part 3. {}", max);
}

fn eni_1(n_val: usize, exp_val: usize, mod_val: usize) -> usize {
    let mut score = 1;
    let mut rems = vec![];

    for _ in 0..exp_val {
        score = (score * n_val).rem_euclid(mod_val);
        rems.push(score);
    }

    concat_ints(&rems)
}

fn eni_2(n_val: usize, exp_val: usize, mod_val: usize) -> usize {
    let mut score = 1;
    let mut leading_iterations = exp_val.saturating_sub(5);
    let mut seen = HashMap::<usize, usize>::new();

    while leading_iterations > 0 {
        score = (score * n_val).rem_euclid(mod_val);
        leading_iterations -= 1;

        if let Some(&prev_iter) = seen.get(&score) {
            let period_len = prev_iter - leading_iterations;
            leading_iterations %= period_len;
        } else {
            seen.insert(score, leading_iterations);
        }
    }

    let final_iterations = exp_val.min(5);
    let mut rems = vec![];

    for _ in 0..final_iterations {
        score = (score * n_val).rem_euclid(mod_val);
        rems.push(score);
    }

    concat_ints(&rems)
}

fn eni_3(n_val: usize, exp_val: usize, mod_val: usize) -> usize {
    let mut score_sum = 0;
    let mut score = 1;
    let mut iterations = exp_val;
    let mut seen = HashMap::<usize, (usize, usize)>::new();

    while iterations > 0 {
        score = (score * n_val).rem_euclid(mod_val);
        score_sum += score;
        iterations -= 1;

        if let Some(&(prev_iter, prev_sum)) = seen.get(&score) {
            let period_len = prev_iter - iterations;
            let period_sum = score_sum - prev_sum;
            let full_periods = iterations / period_len;
            score_sum += full_periods * period_sum;
            iterations %= period_len;
        } else {
            seen.insert(score, (iterations, score_sum));
        }
    }

    score_sum
}

fn concat_ints(numbers: &[usize]) -> usize {
    let mut result = 0;
    for &n in numbers.iter().rev() {
        let digits = (n as f64).log10().floor() as usize + 1;
        result = result * 10_usize.pow(digits as u32) + n;
    }

    result
}

fn sum_triple(row: &Row, fun: fn(usize, usize, usize) -> usize) -> usize {
    fun(row.a, row.x, row.m) + fun(row.b, row.y, row.m) + fun(row.c, row.z, row.m)
}

#[derive(Debug)]
struct Row {
    a: usize,
    b: usize,
    c: usize,
    x: usize,
    y: usize,
    z: usize,
    m: usize,
}

fn parse_rows(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let values: Vec<usize> = line.split_whitespace().map(|pair| parse_kv(pair)).collect();
            if values.len() != 7 {
                panic!("Error parsing 7 input values in a line!");
            }
            Row {
                a: values[0],
                b: values[1],
                c: values[2],
                x: values[3],
                y: values[4],
                z: values[5],
                m: values[6],
            }
        })
        .collect()
}

fn parse_kv(input: &str) -> usize {
    let (_, value) = input.split_once('=').expect("'=' separator.");
    value.parse::<usize>().expect("Should parse as usize.")
}
