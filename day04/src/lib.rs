use shared::parse_2d_map;
use std::io;
use strum_macros::EnumIter;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(lines: &[Vec<char>]) -> usize {
    (0..lines.len())
        .into_iter()
        .map(|row| {
            (0..lines[row].len())
                .into_iter()
                .map(|col| count_xmas_from(lines, row, col))
                .sum::<usize>()
        })
        .sum()
}

fn calculate_part2(lines: &[Vec<char>]) -> usize {
    (1..lines.len() - 1)
        .into_iter()
        .map(|row| {
            (1..lines[row].len() - 1)
                .into_iter()
                .map(|col| is_cross_mass(lines, row, col))
                .filter(|&is_cross| is_cross)
                .count()
        })
        .sum()
}

fn is_cross_mass(lines: &[Vec<char>], row: usize, col: usize) -> bool {
    if lines[row][col] != 'A' {
        return false;
    }

    let top_left = lines[row - 1][col - 1];
    let top_right = lines[row - 1][col + 1];
    let bot_left = lines[row + 1][col - 1];
    let bot_right = lines[row + 1][col + 1];

    is_m_or_s(top_left)
        && is_m_or_s(top_right)
        && are_opposite(top_left, bot_right)
        && are_opposite(top_right, bot_left)
}

fn is_m_or_s(ch: char) -> bool {
    ch == 'M' || ch == 'S'
}

fn are_opposite(ch1: char, ch2: char) -> bool {
    match ch1 {
        'M' => ch2 == 'S',
        'S' => ch2 == 'M',
        _ => false,
    }
}

fn is_xmas_rec(lines: &[Vec<char>], row: usize, col: usize, dir: Direction, letter: char) -> bool {
    if row >= lines.len() || col >= lines[row].len() {
        return false;
    }

    if lines[row][col] != letter {
        return false;
    }

    match get_next_letter(letter) {
        None => true,
        Some(next_letter) => match get_next_coord(row, col, dir) {
            None => false,
            Some((next_row, next_col)) => is_xmas_rec(lines, next_row, next_col, dir, next_letter),
        },
    }
}

fn count_xmas_from(lines: &[Vec<char>], row: usize, col: usize) -> usize {
    use strum::IntoEnumIterator;
    Direction::iter()
        .map(|dir| is_xmas_rec(&lines, row, col, dir, 'X'))
        .filter(|&is_xmas| is_xmas)
        .count()
}

fn get_next_letter(letter: char) -> Option<char> {
    match letter {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        _ => None,
    }
}

fn get_next_coord(row: usize, col: usize, dir: Direction) -> Option<(usize, usize)> {
    match dir {
        Direction::East => Some((row, col + 1)),
        Direction::South => Some((row + 1, col)),
        Direction::West => Some((row, col.checked_sub(1)?)),
        Direction::North => Some((row.checked_sub(1)?, col)),
        Direction::SE => Some((row + 1, col + 1)),
        Direction::SW => Some((row + 1, col.checked_sub(1)?)),
        Direction::NW => Some((row.checked_sub(1)?, col.checked_sub(1)?)),
        Direction::NE => Some((row.checked_sub(1)?, col + 1)),
    }
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    parse_2d_map(file_path)
}

#[derive(Copy, Clone, EnumIter)]
enum Direction {
    East,
    South,
    West,
    North,
    SE,
    SW,
    NW,
    NE,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();
        let cnt = calculate_part1(&input);
        assert_eq!(cnt, 18);
    }

    #[test]
    fn test_is_xmas() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();
        assert_eq!(1, count_xmas_from(&input, 0, 4));
        assert_eq!(1, count_xmas_from(&input, 0, 5));
    }
}
