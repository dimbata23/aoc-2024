use shared::{parse_2d_map, Pos2D};
use std::collections::HashSet;
use std::io;

type DataType = usize;
type Pos2 = Pos2D<usize>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec<DataType>]) -> usize {
    find_zeros(input)
        .into_iter()
        .map(|pos| count_reachable_nines(input, pos))
        .sum()
}

fn calculate_part2(input: &[Vec<DataType>]) -> usize {
    find_zeros(input)
        .into_iter()
        .map(|pos| rate_hike(input, pos))
        .sum()
}

fn count_reachable_nines(matrix: &[Vec<DataType>], start_pos: Pos2) -> usize {
    let mut visited = HashSet::new();
    let mut res_cnt = 0_usize;
    count_reachable_nines_rec(matrix, start_pos, &mut res_cnt, &mut visited);
    res_cnt
}

fn count_reachable_nines_rec(
    matrix: &[Vec<DataType>],
    pos: Pos2,
    cnt: &mut usize,
    visited: &mut HashSet<Pos2>,
) {
    if !(*visited).insert(pos) {
        return;
    }

    let curr_val = matrix[pos.row][pos.col];
    if curr_val == 9 {
        *cnt += 1;
        return;
    }

    for neighbour in get_neighbours(matrix, pos) {
        count_reachable_nines_rec(matrix, neighbour, cnt, visited);
    }
}

fn rate_hike(matrix: &[Vec<DataType>], start_pos: Pos2) -> usize {
    let mut res_cnt = 0_usize;
    rate_hike_rec(matrix, start_pos, &mut res_cnt, HashSet::new());
    res_cnt
}

fn rate_hike_rec(matrix: &[Vec<DataType>], pos: Pos2, cnt: &mut usize, mut visited: HashSet<Pos2>) {
    if !visited.insert(pos) {
        return;
    }

    let curr_val = matrix[pos.row][pos.col];
    if curr_val == 9 {
        *cnt += 1;
        return;
    }

    let neighbours = get_neighbours(matrix, pos);
    if neighbours.len() == 1 {
        let neighbour = neighbours[0];
        rate_hike_rec(matrix, neighbour, cnt, visited);
    } else {
        for neighbour in neighbours {
            rate_hike_rec(matrix, neighbour, cnt, visited.clone());
        }
    }
}

fn find_zeros(matrix: &[Vec<DataType>]) -> Vec<Pos2> {
    matrix
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &value)| value == 0)
                .map(move |(col, _)| Pos2 { row, col })
        })
        .collect()
}

fn get_neighbours(matrix: &[Vec<DataType>], pos: Pos2) -> Vec<Pos2> {
    let mut res = vec![];
    let curr_val = matrix[pos.row][pos.col];

    if pos.row > 0 {
        res.push(Pos2 {
            row: pos.row - 1,
            col: pos.col,
        });
    }

    if pos.col > 0 {
        res.push(Pos2 {
            row: pos.row,
            col: pos.col - 1,
        });
    }

    if pos.row < matrix.len() - 1 {
        res.push(Pos2 {
            row: pos.row + 1,
            col: pos.col,
        });
    }

    if pos.col < matrix[pos.row].len() - 1 {
        res.push(Pos2 {
            row: pos.row,
            col: pos.col + 1,
        });
    }

    res.into_iter()
        .filter(|cpos| matrix[cpos.row][cpos.col] == curr_val + 1)
        .collect()
}

fn parse_file(file_path: &str) -> std::io::Result<Vec<Vec<DataType>>> {
    Ok(parse_2d_map(file_path)?
        .iter()
        .map(|line| {
            line.iter()
                .filter_map(|&c| c.to_digit(10))
                .map(|d| d as DataType)
                .collect()
        })
        .collect())
}
