use std::collections::HashSet;
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

fn calculate_part1(input: &Vec<Vec<char>>) -> usize {
    let mut guard = Guard::from_input(input);
    guard.patrol();
    guard.count_xs()
}

fn calculate_part2(input: &Vec<Vec<char>>) -> usize {
    let guard = Guard::from_input(input);
    guard.count_possible_loops()
}

fn find_starting_pos(matrix: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row, line) in matrix.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '^' {
                return Some((row, col));
            }
        }
    }

    None
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().chars().collect())
        .collect())
}

#[derive(Clone)]
struct Guard {
    matrix: Vec<Vec<char>>,
    curr_row: usize,
    curr_col: usize,
    pos_and_dirs: HashSet<(usize, usize, char)>,
}

impl Guard {
    fn from_input(input: &Vec<Vec<char>>) -> Self {
        let (row, col) = find_starting_pos(input).unwrap();
        Guard {
            matrix: input.clone(),
            curr_row: row,
            curr_col: col,
            pos_and_dirs: HashSet::new(),
        }
    }

    /// # Returns
    /// - `true`: if the patrol completes successfully.
    /// - `false`: if the patrol enters an infinite loop.
    fn patrol(&mut self) -> bool {
        loop {
            match self.move_once() {
                None => return true,
                Some(false) => return false,
                _ => continue,
            }
        }
    }

    fn count_possible_loops(&self) -> usize {
        let mut taken_steps = HashSet::new();
        let mut current_state = self.clone();

        while let Some((next_row, next_col)) = current_state.get_next_move() {
            let mut new_state = self.clone();
            new_state.matrix[next_row][next_col] = '#';

            if !new_state.patrol() {
                taken_steps.insert((next_row, next_col));
            }

            current_state.move_once();
        }

        taken_steps.len()
    }

    /// # Returns
    /// - `None` if the guard moved out of the board
    /// - `Some(true)` if the guard moved into a new position on the board
    /// - `Some(false)` if the guard moved to a position of an infinite loop
    fn move_once(&mut self) -> Option<bool> {
        let curr_pos_dir = (self.curr_row, self.curr_col, self.get_dir());
        let been_there = !self.pos_and_dirs.insert(curr_pos_dir);
        if been_there {
            return Some(false);
        }

        if let Some(next_pos) = self.get_next_move() {
            self.move_to(next_pos);
            Some(true)
        } else {
            None
        }
    }

    fn get_dir(&self) -> char {
        self.matrix[self.curr_row][self.curr_col]
    }

    fn set_dir(&mut self, dir: char) {
        self.matrix[self.curr_row][self.curr_col] = dir;
    }

    fn rotate_90(&mut self) {
        let ch = match self.get_dir() {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            c => c,
        };
        self.set_dir(ch);
    }

    fn move_to(&mut self, (row, col): (usize, usize)) {
        self.matrix[row][col] = self.get_dir();
        self.set_dir('X');
        self.curr_row = row;
        self.curr_col = col;
    }

    fn get_next_move(&mut self) -> Option<(usize, usize)> {
        if let Some((next_row, next_col)) = self.get_pos_infront() {
            if self.matrix[next_row][next_col] == '#' {
                self.rotate_90();
                return self.get_next_move(); // TODO: Potential infinite loop
            }
            Some((next_row, next_col))
        } else {
            self.matrix[self.curr_row][self.curr_col] = 'X';
            None
        }
    }

    fn bound_move(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        (row < self.matrix.len() && col < self.matrix[row].len()).then_some((row, col))
    }

    fn get_pos_infront(&self) -> Option<(usize, usize)> {
        let coord = match self.matrix[self.curr_row][self.curr_col] {
            '^' => (self.curr_row.checked_sub(1)?, self.curr_col),
            '>' => self.bound_move(self.curr_row, self.curr_col + 1)?,
            'v' => self.bound_move(self.curr_row + 1, self.curr_col)?,
            '<' => (self.curr_row, self.curr_col.checked_sub(1)?),
            _ => return None,
        };
        Some(coord)
    }

    fn count_xs(&self) -> usize {
        self.matrix
            .iter()
            .map(|line| line.iter().filter(|&&ch| ch == 'X').count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();
        assert_eq!(
            input,
            vec![
                vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '#', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '#', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '#', '.', '.', '^', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.'],
                vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            ]
        );

        let res_xs = calculate_part1(&input);
        assert_eq!(res_xs, 41);

        let guard = Guard::from_input(&input);
        let res_loops = guard.count_possible_loops();
        assert_eq!(res_loops, 6);
    }

    #[test]
    fn test_loop() {
        let input = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '.'],
            vec!['.', '^', '#', '.'],
        ];

        let mut guard = Guard::from_input(&input);
        let no_loop = guard.patrol();
        assert_eq!(no_loop, false);

        let input = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['.', '.', '.', '.'],
            vec!['#', '.', '.', '.'],
            vec!['.', '^', '.', '.'],
        ];

        let mut guard = Guard::from_input(&input);
        let no_loop = guard.patrol();
        assert_eq!(no_loop, true);
    }
}
