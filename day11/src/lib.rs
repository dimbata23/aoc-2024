use std::collections::HashMap;
use std::fs;
use std::io;

type DataType = u64;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[DataType]) -> usize {
    let mut stones = input.to_vec();
    (0..25).for_each(|_| stones = do_blink(&stones));
    stones.len()
}

fn calculate_part2(input: &[DataType]) -> usize {
    let mut map = HashMap::new();
    input.iter().for_each(|&stone| {
        *map.entry(stone).or_default() += 1;
    });

    (0..75).for_each(|_| map = do_blink_map(&map));

    map.values().sum()
}

fn do_blink(input: &[DataType]) -> Vec<DataType> {
    input
        .iter()
        .flat_map(|&stone| match process_stone(stone) {
            BlinkRes::Stone(res) => vec![res],
            BlinkRes::Split((lhs, rhs)) => vec![lhs, rhs],
        })
        .collect()
}

fn do_blink_map(map: &HashMap<DataType, usize>) -> HashMap<DataType, usize> {
    let mut new_map = HashMap::new();

    for (&stone, &cnt) in map {
        match process_stone(stone) {
            BlinkRes::Stone(res) => {
                *new_map.entry(res).or_default() += cnt;
            }
            BlinkRes::Split((lhs, rhs)) => {
                *new_map.entry(lhs).or_default() += cnt;
                *new_map.entry(rhs).or_default() += cnt;
            }
        };
    }

    new_map
}

enum BlinkRes {
    Stone(DataType),
    Split((DataType, DataType)),
}

fn process_stone(stone: DataType) -> BlinkRes {
    if stone == 0 {
        return BlinkRes::Stone(1);
    }

    let digits_cnt = digits_cnt(stone);
    if digits_cnt % 2 == 0 {
        let split = split_number(stone, digits_cnt / 2);
        BlinkRes::Split(split)
    } else {
        BlinkRes::Stone(stone * 2024)
    }
}

fn digits_cnt(n: DataType) -> usize {
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        count += 1;
        num /= 10;
    }
    count
}

fn split_number(n: DataType, index: usize) -> (DataType, DataType) {
    let divisor = 10u64.pow(index as u32);
    (n / divisor, n % divisor)
}

fn parse_file(file_path: &str) -> io::Result<Vec<DataType>> {
    Ok(fs::read_to_string(file_path)?
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect())
}
