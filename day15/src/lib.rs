use shared::{print_2d_map, Dir, Vec2D};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

type DataType = usize;
type Vec2 = Vec2D<DataType>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &Warehouse) -> DataType {
    let mut warehouse = input.clone();
    print_2d_map(&warehouse.map);
    while warehouse.simulate_step() {}
    print_2d_map(&warehouse.map);
    warehouse.sum_box_gps_coords()
}

fn calculate_part2(input: &Warehouse) -> DataType {
    let mut warehouse = input.make_doubled();
    print_2d_map(&warehouse.map);
    while warehouse.simulate_step() {}
    print_2d_map(&warehouse.map);
    warehouse.sum_box_gps_coords()
}

#[derive(Clone)]
struct Warehouse {
    map: Vec<Vec<char>>,
    robot: Vec2,
    robot_moves: Vec<Dir>,
    curr_step: usize,
}

impl Warehouse {
    fn simulate_step(&mut self) -> bool {
        if self.curr_step >= self.robot_moves.len() {
            return false;
        }

        let dir = self.robot_moves[self.curr_step];
        let wanted_robot_pos = self.robot.moved(dir);

        match self.map[wanted_robot_pos.y][wanted_robot_pos.x] {
            '.' => self.move_robot_free(wanted_robot_pos),
            'O' => {
                if self.try_move_box(wanted_robot_pos, dir) {
                    self.move_robot_free(wanted_robot_pos)
                }
            }
            '[' | ']' => {
                if self.try_move_box_doubled(wanted_robot_pos, dir) {
                    self.move_robot_free(wanted_robot_pos)
                }
            }
            _ => (),
        }

        //print_map(&self.map);
        //wait_for_keypress();

        self.curr_step += 1;
        true
    }

    fn sum_box_gps_coords(&self) -> usize {
        let mut sum = 0_usize;
        for (y, line) in self.map.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch == 'O' || ch == '[' {
                    sum += self.gps_coord(Vec2::new(x, y));
                }
            }
        }
        sum
    }

    fn make_doubled(&self) -> Warehouse {
        let mut map = vec![];
        let robot = Vec2::new(2 * self.robot.x, self.robot.y);
        let robot_moves = self.robot_moves.clone();
        let curr_step = self.curr_step;

        for line in &self.map {
            map.push(vec![]);
            for &ch in line {
                match ch {
                    '#' => map.last_mut().unwrap().extend(vec!['#', '#']),
                    'O' => map.last_mut().unwrap().extend(vec!['[', ']']),
                    '.' => map.last_mut().unwrap().extend(vec!['.', '.']),
                    '@' => map.last_mut().unwrap().extend(vec!['@', '.']),
                    _ => (),
                }
            }
        }

        Warehouse {
            map,
            robot,
            robot_moves,
            curr_step,
        }
    }

    fn try_move_box(&mut self, box_pos: Vec2, dir: Dir) -> bool {
        let wanted_pos = box_pos.moved(dir);
        match self.map[wanted_pos.y][wanted_pos.x] {
            '.' => self.move_box_free(box_pos, wanted_pos),
            'O' => {
                if self.try_move_box(wanted_pos, dir) {
                    self.move_box_free(box_pos, wanted_pos)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn try_move_box_doubled(&mut self, box_pos: Vec2, dir: Dir) -> bool {
        let lhs = {
            let box_side = self.map[box_pos.y][box_pos.x];
            if box_side == '[' {
                box_pos
            } else {
                box_pos.left()
            }
        };

        if self.can_move_box_doubled(lhs, dir) {
            self.move_box_doubled(lhs, dir);
            true
        } else {
            false
        }
    }

    fn can_move_box_doubled(&mut self, lhs: Vec2, dir: Dir) -> bool {
        let rhs = lhs.right();
        let pos_to_check = match dir {
            Dir::Left => vec![lhs.moved(dir)],
            Dir::Right => vec![rhs.moved(dir)],
            Dir::Up | Dir::Down => vec![lhs.moved(dir), rhs.moved(dir)],
        };

        let can_move = pos_to_check
            .into_iter()
            .all(|pos| match self.map[pos.y][pos.x] {
                '.' => true,
                '[' => self.can_move_box_doubled(pos, dir),
                ']' => self.can_move_box_doubled(pos.left(), dir),
                _ => false,
            });

        can_move
    }

    fn move_box_doubled(&mut self, lhs: Vec2, dir: Dir) {
        let rhs = lhs.right();
        let pos_to_check = match dir {
            Dir::Left => vec![lhs.moved(dir)],
            Dir::Right => vec![rhs.moved(dir)],
            Dir::Up | Dir::Down => vec![lhs.moved(dir), rhs.moved(dir)],
        };

        for &pos in &pos_to_check {
            match self.map[pos.y][pos.x] {
                '[' => self.move_box_doubled(pos, dir),
                ']' => self.move_box_doubled(pos.left(), dir),
                _ => (),
            };
        }

        self.move_box_doubled_free(lhs, lhs.moved(dir));
    }

    fn move_box_doubled_free(&mut self, lhs: Vec2, new_lhs: Vec2) {
        let (rhs, new_rhs) = (lhs.right(), new_lhs.right());
        self.map[lhs.y][lhs.x] = '.';
        self.map[rhs.y][rhs.x] = '.';
        self.map[new_lhs.y][new_lhs.x] = '[';
        self.map[new_rhs.y][new_rhs.x] = ']';
    }

    fn move_robot_free(&mut self, new_pos: Vec2) {
        self.map[new_pos.y][new_pos.x] = '@';
        self.map[self.robot.y][self.robot.x] = '.';
        self.robot = new_pos;
    }

    fn move_box_free(&mut self, old_pos: Vec2, new_pos: Vec2) -> bool {
        self.map[new_pos.y][new_pos.x] = 'O';
        self.map[old_pos.y][old_pos.x] = '.';
        true
    }

    fn gps_coord(&self, pos: Vec2) -> usize {
        pos.y * 100 + pos.x
    }
}

fn parse_file(file_path: &str) -> io::Result<Warehouse> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    let mut robot_position = None;
    let mut robot_moves = Vec::new();

    let mut parsing_map = true;

    for line in reader.lines().filter_map(Result::ok) {
        if parsing_map && (line.is_empty() || !line.starts_with('#')) {
            parsing_map = false;
        }

        if parsing_map {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                if ch == '@' {
                    robot_position = Some(Vec2::new(x, map.len()));
                }
                row.push(ch);
            }
            map.push(row);
        } else {
            robot_moves.extend(line.chars().filter_map(Dir::from_char));
        }
    }

    if robot_position.is_none() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No robot position found",
        ));
    }

    Ok(Warehouse {
        map,
        robot: robot_position.unwrap(),
        robot_moves,
        curr_step: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let warehouse = parse_file("sample_input").expect("Failed to parse warehouse");

        assert_eq!(warehouse.robot, Vec2::new(4, 4));
        assert_eq!(warehouse.map[4][4], '@');
        assert_eq!(warehouse.map.len(), 10);
        assert_eq!(warehouse.map[0].len(), 10);

        assert_eq!(warehouse.robot_moves.len(), 700);

        let expected_moves_1 = vec![Dir::Left, Dir::Down, Dir::Down, Dir::Right];

        let expected_moves_2 = vec![
            Dir::Down,
            Dir::Down,
            Dir::Down,
            Dir::Left,
            Dir::Left,
            Dir::Up,
        ];

        let actual_moves_1 = &warehouse.robot_moves[0..expected_moves_1.len()];
        let actual_moves_2 = &warehouse.robot_moves[70..(70 + expected_moves_2.len())];

        assert_eq!(actual_moves_1, expected_moves_1.as_slice());
        assert_eq!(actual_moves_2, expected_moves_2.as_slice());
    }
}

// DEBUG
//use std::io::Read;

//fn wait_for_keypress() {
//    let mut buffer = [0; 1];
//    println!("Press any key to continue...");
//    io::stdin().read(&mut buffer).expect("Failed to read input");
//}
