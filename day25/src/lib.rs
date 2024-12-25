use std::fs;
use std::io;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    println!("Part one result: {res_part1}");

    println!("Part two result: No part 2 :O");
    Ok(())
}

fn calculate_part1(input: &Schematics) -> usize {
    input.calc_valid_pairs()
}

type Schematic = Vec<Vec<char>>;
type Heights = Vec<usize>;

#[derive(Debug, Clone)]
struct Schematics {
    keys: Vec<Schematic>,
    locks: Vec<Schematic>,
    keys_heights: Vec<Heights>,
    locks_heights: Vec<Heights>,
}

fn is_lock(schematic: &[Vec<char>]) -> bool {
    schematic[0].iter().all(|&c| c == '#')
}

fn is_key(schematic: &[Vec<char>]) -> bool {
    schematic[0].iter().all(|&c| c == '.')
}

fn can_fit(lock: &[usize], key: &[usize], max_height: usize) -> bool {
    lock.iter()
        .zip(key.iter())
        .all(|(&l, &k)| l + k <= max_height)
}

fn calculate_heights(schematic: &[Vec<char>], is_lock: bool) -> Vec<usize> {
    let cols = schematic[0].len();
    let rows = schematic.len();
    let mut heights = vec![0; cols];

    for col in 0..cols {
        for row in 0..rows {
            let index = if is_lock { row } else { rows - row - 1 };
            if schematic[index][col] == '#' {
                heights[col] += 1;
            } else {
                break;
            }
        }
    }

    heights
}

fn parse_schematic(schematic: &str) -> Vec<Vec<char>> {
    schematic
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn parse_file(file_path: &str) -> io::Result<Schematics> {
    let mut res = Schematics::new();
    let content = fs::read_to_string(file_path)?;
    let schematics: Vec<&str> = content.split("\n\n").collect();

    for schematic in schematics {
        let parsed = parse_schematic(schematic);
        if is_lock(&parsed) {
            res.locks.push(parsed);
        } else if is_key(&parsed) {
            res.keys.push(parsed);
        }
    }

    for lock in &res.locks {
        res.locks_heights.push(calculate_heights(lock, true));
    }

    for key in &res.keys {
        res.keys_heights.push(calculate_heights(key, false));
    }

    Ok(res)
}

impl Schematics {
    fn new() -> Self {
        Schematics {
            keys: vec![],
            locks: vec![],
            keys_heights: vec![],
            locks_heights: vec![],
        }
    }

    fn calc_valid_pairs(&self) -> usize {
        let mut valid_pairs = 0;
        const SCHEM_MAX_HEIGHT: usize = 7;

        for lock in &self.locks_heights {
            for key in &self.keys_heights {
                if can_fit(lock, key, SCHEM_MAX_HEIGHT) {
                    valid_pairs += 1;
                }
            }
        }

        valid_pairs
    }
}
