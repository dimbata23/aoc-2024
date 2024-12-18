use shared::Vec2D;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

type Vec2 = Vec2D<usize>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec2]) -> usize {
    const WIDTH: usize = 71_usize;
    const HEIGHT: usize = 71_usize;
    const BYTES: usize = 1024_usize;
    let map = gen_2d_map(input, WIDTH, HEIGHT, BYTES);
    shortest_path(&map).unwrap()
}

fn calculate_part2(input: &[Vec2]) -> String {
    const WIDTH: usize = 71_usize;
    const HEIGHT: usize = 71_usize;
    const BYTES: usize = 1024_usize;
    let mut map = gen_2d_map(input, WIDTH, HEIGHT, BYTES);
    let mut bytes = BYTES;
    while shortest_path(&map).is_some() {
        let byte = input[bytes];
        map[byte.y][byte.x] = '#';
        bytes += 1;
    }
    let byte = input[bytes - 1];
    format!("{},{}", byte.x, byte.y)
}

fn shortest_path(map: &[Vec<char>]) -> Option<usize> {
    let height = map.len();
    let width = map[0].len();
    let limit = Vec2::new(width, height);
    let target = Vec2::new(width - 1, height - 1);
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    queue.push_back((Vec2::new(0, 0), 0));
    visited[0][0] = true;

    while let Some((pos, distance)) = queue.pop_front() {
        if pos == target {
            return Some(distance);
        }

        for new_pos in pos.gen_neighbours_constrained(limit) {
            if !visited[new_pos.y][new_pos.x] && map[new_pos.y][new_pos.x] == '.' {
                visited[new_pos.y][new_pos.x] = true;
                queue.push_back((new_pos, distance + 1));
            }
        }
    }

    None
}

fn gen_2d_map(input: &[Vec2], width: usize, height: usize, bytes_cnt: usize) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; width]; height];
    for byte in input.iter().take(bytes_cnt) {
        map[byte.y][byte.x] = '#';
    }
    map
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec2>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let pairs = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(',');
            let x = parts.next().unwrap().trim().parse().unwrap();
            let y = parts.next().unwrap().trim().parse().unwrap();
            Vec2::new(x, y)
        })
        .collect();

    Ok(pairs)
}
