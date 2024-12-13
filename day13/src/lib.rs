use shared::{determinant, intersect_vecs, Vec2D};
use std::fs::File;
use std::io;
use std::io::Read;
use std::str::FromStr;

type DataType = i64;
type Vec2 = Vec2D<DataType>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[ClawMachine]) -> DataType {
    input
        .iter()
        .filter_map(|machine| machine.get_cheapest_solution())
        .map(|sol| cost(&sol))
        .sum()
}

fn calculate_part2(input: &[ClawMachine]) -> DataType {
    input
        .iter()
        .map(ClawMachine::adjusted_prize_position)
        .filter_map(|machine| machine.get_the_only_solution())
        .map(|sol| cost(&sol))
        .sum()
}

#[derive(Debug, Eq, PartialEq)]
struct ClawMachine {
    button_a: Vec2,
    button_b: Vec2,
    prize: Vec2,
}

impl ClawMachine {
    fn get_cheapest_solution(&self) -> Option<Vec2> {
        let sols = self.get_all_possible_solutions();
        sols.into_iter().min_by_key(cost)
    }

    fn get_the_only_solution(&self) -> Option<Vec2> {
        let det = match determinant(self.button_a, self.button_b) {
            0 => return None,
            det => det as f64,
        };

        let coeff_a = determinant(self.prize, self.button_b) as f64 / det;
        let coeff_b = determinant(self.button_a, self.prize) as f64 / det;
        if coeff_a.is_sign_negative()
            || coeff_b.is_sign_negative()
            || coeff_a.fract() != 0.0
            || coeff_b.fract() != 0.0
        {
            return None;
        }

        Some(Vec2::new(coeff_a as DataType, coeff_b as DataType))
    }

    fn get_all_possible_solutions(&self) -> Vec<Vec2> {
        let x_eq = Diophantine::new(self.button_a.x, self.button_b.x, self.prize.x);
        let y_eq = Diophantine::new(self.button_a.y, self.button_b.y, self.prize.y);
        let x_sols = x_eq.all_non_negative_solutions();
        let y_sols = y_eq.all_non_negative_solutions();
        intersect_vecs(&x_sols, &y_sols)
    }

    fn adjusted_prize_position(&self) -> ClawMachine {
        ClawMachine {
            button_a: self.button_a,
            button_b: self.button_b,
            prize: self.prize + Vec2::new(10000000000000, 10000000000000),
        }
    }
}

fn cost(sol: &Vec2) -> DataType {
    3 * sol.x + sol.y
}

fn parse_file(file_path: &str) -> io::Result<Vec<ClawMachine>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents
        .split("\n\n")
        .map(|entry| entry.parse().unwrap())
        .collect::<Vec<ClawMachine>>())
}

fn parse_vec2(s: &str) -> Vec2 {
    let parts: Vec<&str> = s.split(", ").collect();
    let x = parts[0]
        .trim_matches(|c| c == 'X' || c == '=')
        .parse()
        .unwrap();
    let y = parts[1]
        .trim_matches(|c| c == 'Y' || c == '=')
        .parse()
        .unwrap();
    Vec2 { x, y }
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let button_a = parse_vec2(lines[0].trim_start_matches("Button A: "));
        let button_b = parse_vec2(lines[1].trim_start_matches("Button B: "));
        let prize = parse_vec2(lines[2].trim_start_matches("Prize: "));
        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}

/**************************************************************/
/******** Despicable code ahead. Proceed with caution! ********/
/**************************************************************/

#[derive(Debug, Copy, Clone)]
struct Diophantine {
    u: DataType,
    v: DataType,
    w: DataType,
}

impl Diophantine {
    fn new(u: DataType, v: DataType, w: DataType) -> Self {
        Self { u, v, w }
    }

    fn all_non_negative_solutions(self) -> Vec<Vec2> {
        let mut solutions = vec![];

        if let Some((sol, gcd)) = self.solve() {
            let step_x = self.v / gcd;
            let step_y = self.u / gcd;

            let k_min = if step_x > 0 {
                (-sol.x as f64 / step_x as f64).ceil() as i64
            } else {
                (-sol.x as f64 / step_x as f64).floor() as i64
            };

            let k_max = if step_y > 0 {
                (sol.y as f64 / step_y as f64).floor() as i64
            } else {
                (sol.y as f64 / step_y as f64).ceil() as i64
            };

            // General solution
            // x = a + k * step_x
            // y = b - k * step_y

            for k in k_min..=k_max {
                let x_k = sol.x + k * step_x;
                let y_k = sol.y - k * step_y;

                solutions.push(Vec2::new(x_k, y_k));
            }
        }

        solutions
    }

    fn solve(self) -> Option<(Vec2, DataType)> {
        let (x, y, gcd) = gcd_extended(self.u, self.v);
        if self.w % gcd != 0 {
            return None;
        }
        let scale = self.w / gcd;
        let a = x * scale;
        let b = y * scale;
        Some((Vec2::new(a, b), gcd))
    }
}

fn gcd_extended(a: DataType, b: DataType) -> (DataType, DataType, DataType) {
    if b == 0 {
        return (1, 0, a);
    }
    let (x1, y1, gcd) = gcd_extended(b, a % b);
    let x = y1;
    let y = x1 - (a / b) * y1;
    (x, y, gcd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vec2() {
        let input = "X+94, Y+34";
        let expected = Vec2::new(94, 34);
        let parsed = parse_vec2(input);
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_claw_machine() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let expected = ClawMachine {
            button_a: Vec2 { x: 94, y: 34 },
            button_b: Vec2 { x: 22, y: 67 },
            prize: Vec2 { x: 8400, y: 5400 },
        };
        let parsed: ClawMachine = input.parse().unwrap();
        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_parse_multiple_claw_machines() -> io::Result<()> {
        let parsed_machines = parse_file("small_input")?;

        let expected = vec![
            ClawMachine {
                button_a: Vec2 { x: 94, y: 34 },
                button_b: Vec2 { x: 22, y: 67 },
                prize: Vec2 { x: 8400, y: 5400 },
            },
            ClawMachine {
                button_a: Vec2 { x: 26, y: 66 },
                button_b: Vec2 { x: 67, y: 21 },
                prize: Vec2 { x: 12748, y: 12176 },
            },
        ];

        assert_eq!(parsed_machines, expected);

        Ok(())
    }
}
