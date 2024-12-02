use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec<i16>]) -> usize {
    input
        .iter()
        .map(|report| is_safe(report))
        .filter(|&safe| safe)
        .count()
}

fn calculate_part2(input: &[Vec<i16>]) -> usize {
    input
        .iter()
        .map(|report| is_safe_damped(report))
        .filter(|&safe| safe)
        .count()
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<i16>>> {
    File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .map(|line| {
            line.map(|l| {
                l.split_whitespace()
                    .filter_map(|num| num.parse::<i16>().ok())
                    .collect()
            })
        })
        .collect()
}

fn is_safe(report: &[i16]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let mut incr = true;
    let mut decr = true;

    for window in report.windows(2) {
        let (a, b) = (window[0], window[1]);
        let diff = a - b;
        let dist = diff.abs();
        if dist < 1 || dist > 3 {
            return false;
        }

        if diff < 0 {
            decr = false;
            if !incr {
                return false;
            }
        } else {
            incr = false;
            if !decr {
                return false;
            }
        }
    }

    true
}

fn is_safe_damped(report: &[i16]) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let damped = {
            let mut res: Vec<_> = report.into();
            res.remove(i);
            res
        };
        if is_safe(&damped) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample_input() {
        let result = parse_file("sample_input");
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result,
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        );
    }

    #[test]
    fn is_safe_sampe_test() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();
        let result: Vec<_> = input.iter().map(|report| is_safe(report)).collect();
        assert_eq!(result, vec![true, false, false, false, false, true]);
    }

    #[test]
    fn is_safe_damped_sampe_test() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();
        let result: Vec<_> = input.iter().map(|report| is_safe_damped(report)).collect();
        assert_eq!(result, vec![true, false, false, true, true, true]);
    }
}
