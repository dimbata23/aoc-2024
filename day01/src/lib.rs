use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run() -> io::Result<()> {
    let (col1, col2) = parse_file("input")?;

    let res_part1 = calculate_part1(&col1, &col2);
    let res_part2 = calculate_part2(&col1, &col2);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");

    Ok(())
}

fn calculate_part1(col1: &[i32], col2: &[i32]) -> u64 {
    let mut col2 = col2.to_vec();
    col2.sort();

    col1.iter()
        .sorted()
        .zip(col2.iter())
        .map(|(&a, &b)| dist(a, b))
        .sum()
}

fn calculate_part2(col1: &[i32], col2: &[i32]) -> usize {
    col1.iter()
        .map(|&num1| {
            let num1_as_usize = num1 as usize;
            let matching_count = col2.iter().filter(|&&num2| num2 == num1).count();
            num1_as_usize * matching_count
        })
        .sum()
}

fn dist(num1: i32, num2: i32) -> u64 {
    (num1 as i64 - num2 as i64).abs() as u64
}

fn parse_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let res = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let mut parts = line.split_whitespace();
            let value1 = parts.next()?.parse::<i32>().ok()?;
            let value2 = parts.next()?.parse::<i32>().ok()?;
            Some((value1, value2))
        })
        .filter_map(|opt| opt)
        .unzip();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_file_parsing() {
        let res = parse_file("sample_input");
        assert!(res.is_ok());
        let (col1, col2) = res.unwrap();
        assert_eq!(col1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(col2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn dist_fn_test() {
        assert_eq!(dist(3, 8), 5);
        assert_eq!(dist(10, -3), 13);
        assert_eq!(dist(i32::MAX, i32::MIN), i32::MAX as u64 * 2 + 1);
    }

    #[test]
    fn sample_part1() {
        let res = parse_file("sample_input");
        assert!(res.is_ok());
        let (col1, col2) = res.unwrap();
        let res = calculate_part1(&col1, &col2);
        assert_eq!(res, 11);
    }

    #[test]
    fn sample_part2() {
        let res = parse_file("sample_input");
        assert!(res.is_ok());
        let (col1, col2) = res.unwrap();
        let res = calculate_part2(&col1, &col2);
        assert_eq!(res, 31);
    }
}
