fn main() {
    part_one();
    part_two();
    part_three();
}

fn part_one() {
    // s(n) = n^2
    // n^2 > T
    // n = roof(sqrt(T))

    let input1 = include_str!("../input1");

    let available_blocks = input1.parse::<usize>().expect("Should be an integer");
    let layers = (available_blocks as f64).sqrt().ceil() as usize;
    let base_width = 2 * layers - 1;
    let blocks_needed = layers.pow(2);
    let additional_blocks = blocks_needed - available_blocks;
    let answer = additional_blocks * base_width;

    println!("Part 1. {}", answer);
}

fn part_two() {
    let input2 = include_str!("../input2");

    let number_priests = input2.parse::<usize>().expect("Should be an integer");
    let number_accolytes = 1111;
    let priests_marble_supply = 2024_00_00;

    let mut current_layer_thickness = 1;
    let mut blocks_used = 1;
    let mut temple_width = 1;

    while blocks_used < priests_marble_supply {
        current_layer_thickness =
            (current_layer_thickness * number_priests).rem_euclid(number_accolytes);

        temple_width += 2;
        blocks_used += temple_width * current_layer_thickness;
    }

    let additional_blocks_needed = blocks_used - priests_marble_supply;
    let answer = additional_blocks_needed * temple_width;

    println!("Part 2. {}", answer);
}

fn part_three() {
    let input3 = include_str!("../input3");

    let number_high_priests = input3.parse::<usize>().expect("Should be an integer");
    let number_high_priest_accolytes = 10;
    let priests_platinum_supply = 2024_00_000;

    let mut columns: Vec<usize> = vec![1];
    let mut current_layer_thickness = 1;
    let mut blocks_needed = 1;

    columns.reserve_exact(100_000);

    while blocks_needed < priests_platinum_supply {
        current_layer_thickness = (current_layer_thickness * number_high_priests)
            .rem_euclid(number_high_priest_accolytes)
            + number_high_priest_accolytes;

        for c in columns.iter_mut() {
            *c += current_layer_thickness;
        }
        columns.push(current_layer_thickness);

        let base_width = 2 * columns.len() - 1;

        let blocks_all_layers = columns[0] + 2 * columns.iter().skip(1).sum::<usize>();

        let mut blocks_to_remove = (base_width * number_high_priests * columns[0])
            .rem_euclid(number_high_priest_accolytes);

        if columns.len() > 2 {
            for height in columns[1..columns.len() - 1].iter() {
                blocks_to_remove += 2
                    * (base_width * number_high_priests * height)
                        .rem_euclid(number_high_priest_accolytes);
            }
        }

        blocks_needed = blocks_all_layers - blocks_to_remove;
    }

    let additional_blocks_needed = blocks_needed - priests_platinum_supply;
    println!("Part 3. {}", additional_blocks_needed);
}
