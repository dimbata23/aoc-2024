use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| eq.is_possible_with_2ops())
        .map(|eq| eq.res)
        .sum()
}

fn calculate_part2(input: &[Equation]) -> u64 {
    input
        .iter()
        .filter(|eq| eq.is_possible_with_3ops())
        .map(|eq| eq.res)
        .sum()
}

#[derive(Debug, PartialEq)]
struct Equation {
    res: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Concatenate => format!("{}{}", lhs, rhs).parse::<u64>().unwrap(),
        }
    }
}

impl Equation {
    fn is_possible_with_2ops(&self) -> bool {
        self.is_possible_with_ops(&[Operator::Add, Operator::Multiply])
    }

    fn is_possible_with_3ops(&self) -> bool {
        self.is_possible_with_ops(&[Operator::Add, Operator::Multiply, Operator::Concatenate])
    }

    fn is_possible_with_ops(&self, operators: &[Operator]) -> bool {
        let n = self.operands.len();

        if n == 1 {
            return self.operands[0] == self.res;
        }

        let base = operators.len() as u64;
        let max_mask = base.pow((n - 1) as u32);

        for mask in 0..max_mask {
            let mut total = self.operands[0];
            let mut current_mask = mask;

            for i in 0..(n - 1) {
                let op_index = (current_mask % base) as usize;
                current_mask /= base;

                total = operators[op_index].apply(total, self.operands[i + 1]);
            }

            if total == self.res {
                return true;
            }
        }

        false
    }
}

fn parse_file(file_path: &str) -> io::Result<Vec<Equation>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| {
            let parts: Vec<_> = line.split(':').collect();
            assert_eq!(parts.len(), 2);
            let res = parts[0].parse::<u64>().unwrap();
            let operands: Vec<u64> = parts[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            Equation { res, operands }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();

        assert_eq!(
            input,
            vec![
                Equation {
                    res: 190,
                    operands: vec![10, 19]
                },
                Equation {
                    res: 3267,
                    operands: vec![81, 40, 27]
                },
                Equation {
                    res: 83,
                    operands: vec![17, 5]
                },
                Equation {
                    res: 156,
                    operands: vec![15, 6]
                },
                Equation {
                    res: 7290,
                    operands: vec![6, 8, 6, 15]
                },
                Equation {
                    res: 161011,
                    operands: vec![16, 10, 13]
                },
                Equation {
                    res: 192,
                    operands: vec![17, 8, 14]
                },
                Equation {
                    res: 21037,
                    operands: vec![9, 7, 18, 13]
                },
                Equation {
                    res: 292,
                    operands: vec![11, 6, 16, 20]
                },
            ]
        );

        let res1 = calculate_part1(&input);
        assert_eq!(res1, 3749);

        let res2 = calculate_part2(&input);
        assert_eq!(res2, 11387);
    }
}
