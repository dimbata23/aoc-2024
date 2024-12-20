use shared::{parse_2d_map, Vec2D};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

type Vec2 = Vec2D<usize>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec<char>]) -> usize {
    let dists = dists_from_start(input);
    find_cheats(&dists, 2)
}

fn calculate_part2(input: &[Vec<char>]) -> usize {
    let dists = dists_from_start(input);
    find_cheats(&dists, 20)
}

fn find_cheats(dists: &HashMap<Vec2, usize>, no_clip_time: usize) -> usize {
    let mut cnt = 0;
    for (&pos1, &dist1) in dists {
        for (&pos2, &dist2) in dists {
            if pos1 == pos2 {
                continue;
            }

            let dist = pos1.manhattan_distance(pos2);
            if dist <= no_clip_time {
                let saves = dist1.abs_diff(dist2) - dist;
                if saves >= 100 {
                    cnt += 1;
                }
            }
        }
    }

    cnt / 2 // each cheat was counted twice
}

fn dists_from_start(map: &[Vec<char>]) -> HashMap<Vec2, usize> {
    let start = get_start(map);
    let end = get_end(map);
    let mut distances = HashMap::new();

    let height = map.len();
    let width = map[0].len();
    let limit = Vec2::new(width, height);
    let mut visited = vec![vec![false; width]; height];
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited[start.y][start.x] = true;

    while let Some((pos, distance)) = queue.pop_front() {
        distances.insert(pos, distance);

        if pos == end {
            continue;
        }

        for new_pos in pos.gen_neighbours_constrained(limit) {
            if !visited[new_pos.y][new_pos.x] && map[new_pos.y][new_pos.x] != '#' {
                visited[new_pos.y][new_pos.x] = true;
                queue.push_back((new_pos, distance + 1));
            }
        }
    }

    distances
}

fn search_for(map: &[Vec<char>], target: char) -> Option<Vec2> {
    for (y, line) in map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == target {
                return Some(Vec2::new(x, y));
            }
        }
    }

    None
}

fn get_start(map: &[Vec<char>]) -> Vec2 {
    search_for(map, 'S').unwrap()
}

fn get_end(map: &[Vec<char>]) -> Vec2 {
    search_for(map, 'E').unwrap()
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    parse_2d_map(file_path)
}
