use shared::Vec2D;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn calculate_part1(input: &[Robot]) -> usize {
    const STEPS: DataType = 100;
    const WIDTH: DataType = 101;
    const HEIGHT: DataType = 103;
    const ROOM_SIZE: RoomSize = RoomSize::new(WIDTH as DataType, HEIGHT as DataType);
    let new_spots = calc_new_spots(&input, ROOM_SIZE, STEPS);
    calc_safety_factor(&new_spots, ROOM_SIZE)
}

fn calculate_part2(input: &[Robot]) -> usize {
    const WIDTH: DataType = 101;
    const HEIGHT: DataType = 103;
    const ROOM_SIZE: RoomSize = RoomSize::new(WIDTH as DataType, HEIGHT as DataType);
    commence_manual_labour(input, ROOM_SIZE)
}

fn calc_new_spots(robots: &[Robot], room_size: Vec2, steps: DataType) -> Vec<Vec2> {
    robots
        .iter()
        .map(|robot| robot.step(room_size, steps))
        .collect()
}

fn calc_safety_factor(robots: &[Vec2], room_size: Vec2) -> usize {
    quadrants_cnt(robots, room_size)
        .iter()
        .fold(1, |lhs, rhs| lhs * rhs)
}

fn quadrants_cnt(robots: &[Vec2], room_size: Vec2) -> Vec<usize> {
    let mid = room_size / 2;
    let mut quadrants = vec![0; 4];
    for robot in robots {
        if robot.x < mid.x {
            if robot.y < mid.y {
                quadrants[0] += 1;
            } else if robot.y > mid.y {
                quadrants[1] += 1;
            }
        } else if robot.x > mid.x {
            if robot.y < mid.y {
                quadrants[2] += 1;
            } else if robot.y > mid.y {
                quadrants[3] += 1;
            }
        }
    }
    quadrants
}

type RoomSize = Vec2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Robot {
    p: Vec2,
    v: Vec2,
}

impl Robot {
    fn step(self, room_size: RoomSize, steps: DataType) -> Vec2 {
        let mut new_pos = self.p + (self.v * steps);
        new_pos %= room_size;
        if new_pos.x < 0 {
            new_pos.x += room_size.x;
        }
        if new_pos.y < 0 {
            new_pos.y += room_size.y;
        }
        new_pos
    }
}

fn commence_manual_labour(input: &[Robot], room_size: Vec2) -> usize {
    let mut sec = 1_usize;
    loop {
        let positions = calc_new_spots(&input, room_size, sec as DataType);
        let may_be_tree = print_matrix(&positions, room_size);
        println!("{}", '-'.to_string().repeat(room_size.x as usize));
        if !may_be_tree {
            sec += 1;
            continue;
        }

        println!("Do you see a christmas tree [y(es), n(ext) = default]?");
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        if buf.starts_with('y') {
            println!("Yay a christmas tree!");
            break;
        }

        sec += 1;
    }
    sec
}

fn parse_file(file_path: &str) -> io::Result<Vec<Robot>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut robots = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(robot) = parse_line(&line) {
            robots.push(robot);
        }
    }

    Ok(robots)
}

fn parse_line(line: &str) -> Option<Robot> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }

    let p = parse_vec2(parts[0])?;
    let v = parse_vec2(parts[1])?;

    Some(Robot { p, v })
}

fn parse_vec2(part: &str) -> Option<Vec2> {
    let values: Vec<&str> = part[2..].split(',').collect(); // Skips "p=" and "v="
    if values.len() != 2 {
        return None;
    }

    let x = values[0].parse::<DataType>().ok()?;
    let y = values[1].parse::<DataType>().ok()?;

    Some(Vec2::new(x, y))
}

fn print_matrix(positions: &[Vec2], room_size: Vec2) -> bool {
    let mut tree_pos = None;
    let mut matrix = vec![vec![' '; room_size.x as usize]; room_size.y as usize];
    for pos in positions {
        matrix[pos.y as usize][pos.x as usize] = 'X';
        if may_contain_christmas_tree(&matrix, *pos) {
            tree_pos = Some(pos);
            matrix[pos.y as usize][pos.x as usize] = 'O';
        }
    }

    for line in matrix {
        for ch in line {
            print!("{ch}");
        }
        println!();
    }

    match tree_pos {
        Some(pos) => {
            println!("The christmas tree might be at position: {:?}", pos);
            true
        }
        None => false,
    }
}

fn may_contain_christmas_tree(matrix: &[Vec<char>], pos: Vec2) -> bool {
    const TREE_HEIGHT_MIN: DataType = 10;
    if pos.y > TREE_HEIGHT_MIN {
        (0..TREE_HEIGHT_MIN)
            .into_iter()
            .all(|i| matrix[(pos.y - i) as usize][pos.x as usize] != ' ')
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let expected_output = vec![
            Robot {
                p: Vec2 { x: 0, y: 4 },
                v: Vec2 { x: 3, y: -3 },
            },
            Robot {
                p: Vec2 { x: 6, y: 3 },
                v: Vec2 { x: -1, y: -3 },
            },
            Robot {
                p: Vec2 { x: 10, y: 3 },
                v: Vec2 { x: -1, y: 2 },
            },
            Robot {
                p: Vec2 { x: 2, y: 0 },
                v: Vec2 { x: 2, y: -1 },
            },
            Robot {
                p: Vec2 { x: 0, y: 0 },
                v: Vec2 { x: 1, y: 3 },
            },
            Robot {
                p: Vec2 { x: 3, y: 0 },
                v: Vec2 { x: -2, y: -2 },
            },
            Robot {
                p: Vec2 { x: 7, y: 6 },
                v: Vec2 { x: -1, y: -3 },
            },
            Robot {
                p: Vec2 { x: 3, y: 0 },
                v: Vec2 { x: -1, y: -2 },
            },
            Robot {
                p: Vec2 { x: 9, y: 3 },
                v: Vec2 { x: 2, y: 3 },
            },
            Robot {
                p: Vec2 { x: 7, y: 3 },
                v: Vec2 { x: -1, y: 2 },
            },
            Robot {
                p: Vec2 { x: 2, y: 4 },
                v: Vec2 { x: 2, y: -3 },
            },
            Robot {
                p: Vec2 { x: 9, y: 5 },
                v: Vec2 { x: -3, y: -3 },
            },
        ];

        let result = parse_file("sample_input").expect("Failed to parse file");
        assert_eq!(result, expected_output);
    }
}
