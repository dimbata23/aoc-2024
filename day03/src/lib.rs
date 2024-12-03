use regex::Regex;
use std::{fs, io};

pub fn run() -> io::Result<()> {
    let input = fs::read_to_string("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &str) -> u64 {
    get_operations(input)
        .into_iter()
        .map(|op| match op {
            Operation::Mul(num1, num2) => num1 * num2,
            _ => 0,
        })
        .sum()
}

fn calculate_part2(input: &str) -> u64 {
    let mut process = true;

    get_operations(input)
        .into_iter()
        .map(|op| match op {
            Operation::Do => {
                process = true;
                0
            }
            Operation::Dont => {
                process = false;
                0
            }
            Operation::Mul(num1, num2) if process => num1 * num2,
            _ => 0,
        })
        .sum()
}

fn get_operations(input: &str) -> Vec<Operation> {
    let re = Regex::new(r"(mul)\((\d+?),(\d+?)\)|(do\(\))|(don't\(\))").unwrap();

    re.captures_iter(input)
        .filter_map(|groups| {
            if let Some(_) = groups.get(4) {
                Some(Operation::Do)
            } else if let Some(_) = groups.get(5) {
                Some(Operation::Dont)
            } else if let Some(_) = groups.get(1) {
                let num1: u64 = groups.get(2)?.as_str().parse().ok()?;
                let num2: u64 = groups.get(3)?.as_str().parse().ok()?;
                Some(Operation::Mul(num1, num2))
            } else {
                None
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
enum Operation {
    Do,
    Dont,
    Mul(u64, u64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_sample_part1() {
        let res = calculate_part1(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );
        assert_eq!(res, 161);
    }

    #[test]
    fn parse_calc_sample_part1() {
        let input = fs::read_to_string("sample_input_1");
        assert!(input.is_ok());
        let input = input.unwrap();
        let res = calculate_part1(&input);
        assert_eq!(res, 161);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("sample_input_2");
        assert!(input.is_ok());
        let input = input.unwrap();

        let ops = get_operations(&input);
        assert_eq!(
            ops,
            vec![
                Operation::Mul(2, 4),
                Operation::Dont,
                Operation::Mul(5, 5),
                Operation::Mul(11, 8),
                Operation::Do,
                Operation::Mul(8, 5)
            ]
        );

        let res = calculate_part2(&input);
        assert_eq!(res, 48);
    }
}
