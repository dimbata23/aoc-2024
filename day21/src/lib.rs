use lazy_static::lazy_static;
use shared::{parse_2d_map, Vec2D};
use std::collections::{HashMap, VecDeque};
use std::{io, usize};

type Vec2 = Vec2D<usize>;
type PathsMap = HashMap<FromTo, Vec<CharPath>>;
type Code = [char; 4];
type CharPath = Vec<char>;

static DIR_PAD: &[&[char]] = &[&['#', '^', 'A'], &['<', 'v', '>']];
static NUM_PAD: &[&[char]] = &[
    &['7', '8', '9'],
    &['4', '5', '6'],
    &['1', '2', '3'],
    &['#', '0', 'A'],
];

lazy_static! {
    static ref DIR_PAD_PATHS: PathsMap = {
        let mut map = shortest_paths(DIR_PAD);
        let entry = map.get_mut(&FromTo::new('<', 'A')).unwrap();
        *entry = vec![vec!['>', '>', '^', 'A']];
        let entry = map.get_mut(&FromTo::new('A', '<')).unwrap();
        *entry = vec![vec!['v', '<', '<', 'A']];
        map
    };
    static ref NUM_PAD_PATHS: PathsMap = shortest_paths(NUM_PAD);
}

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    println!("Part one result: {res_part1}");

    let res_part2 = calculate_part2(&input);
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Code]) -> usize {
    input.iter().map(|code| Pads::new(2).do_code(code)).sum()
}

fn calculate_part2(input: &[Code]) -> usize {
    // TODO: Memoisation
    input.iter().map(|code| Pads::new(25).do_code(code)).sum()
}

struct Pads {
    dir_pads_cnt: usize,
}

fn code_val(code: &Code) -> usize {
    let mut number = 0;

    for &ch in &code[0..3] {
        number = number * 10 + (ch as usize - '0' as usize);
    }

    number
}

impl Pads {
    fn new(dir_pads_cnt: usize) -> Self {
        Self { dir_pads_cnt }
    }

    fn do_code(&self, code: &Code) -> usize {
        let mut curr_from = 'A';
        let mut price = 0;
        let mut path = vec![];
        for &ch in code {
            let (ch_price, ch_path) = self.do_one_char(curr_from, ch);
            price += ch_price;
            path.extend(ch_path);
            curr_from = ch;
        }

        price * code_val(code)
    }

    fn do_one_char(&self, from: char, to: char) -> (usize, Vec<char>) {
        let from_to = FromTo::new(from, to);
        let num_paths = &NUM_PAD_PATHS[&from_to];
        let mut least_len = usize::MAX;
        let mut least_path = vec![];

        for path in num_paths {
            let dir_paths = self.do_dir_paths(&path, self.dir_pads_cnt);
            for dir_path in dir_paths {
                if dir_path.len() < least_len {
                    least_len = dir_path.len();
                    least_path = dir_path;
                }
            }
        }

        (least_len, least_path)
    }

    fn do_dir_paths(&self, path: &[char], depth: usize) -> Vec<Vec<char>> {
        if depth == 0 {
            return vec![path.to_vec()];
        }
        let mut curr_from = 'A';
        let mut all_paths = vec![];
        for &ch in path {
            let from_to = FromTo::new(curr_from, ch);
            let child_paths = &DIR_PAD_PATHS[&from_to];
            if all_paths.is_empty() {
                all_paths = child_paths.to_vec();
            } else {
                all_paths = Self::do_comb_paths(&all_paths, &child_paths);
            }
            curr_from = ch;
        }

        let mut res_paths = vec![];
        //let all_paths = vec![all_paths[0].clone()];
        //assert!(all_paths
        //    .windows(2)
        //    .all(|window| window[0].len() == window[1].len()));
        for path in all_paths {
            let child_paths = self.do_dir_paths(&path, depth - 1);
            res_paths.extend(child_paths.into_iter());
        }

        //assert!(res_paths
        //    .windows(2)
        //    .all(|window| window[0].len() == window[1].len()));
        //vec![res_paths[0].clone()]

        res_paths
    }

    fn do_comb_paths(prefixes: &[Vec<char>], suffixes: &[Vec<char>]) -> Vec<Vec<char>> {
        let mut vec = vec![];
        for prefix in prefixes {
            for suffix in suffixes {
                let mut comb = prefix.to_vec();
                comb.extend_from_slice(suffix);
                vec.push(comb);
            }
        }
        vec
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct FromTo {
    from: char,
    to: char,
}

impl FromTo {
    fn new(from: char, to: char) -> Self {
        Self { from, to }
    }
}

/// Blatantly obvious AI generated bfs_pahs and shrotest_paths functions

/// Performs a BFS from a given start position and returns all shortest paths as sequences of `Dir`.
fn bfs_paths(map: &[&[char]], start: Vec2) -> HashMap<Vec2, Vec<CharPath>> {
    let height = map.len();
    let width = map[0].len();
    let limit = Vec2::new(width, height);

    // Track the shortest distance and the paths to each node
    let mut visited = vec![vec![None; width]; height];
    let mut queue = VecDeque::new();
    let mut paths = HashMap::new();

    queue.push_back((start, Vec::new()));
    visited[start.y][start.x] = Some(0);
    paths.insert(start, vec![Vec::new()]);

    while let Some((pos, current_path)) = queue.pop_front() {
        let current_distance = visited[pos.y][pos.x].unwrap();

        // Generate neighbors
        let ns = pos.gen_neighbours_dirs_constrained(limit);
        for (new_pos, dir) in ns {
            if map[new_pos.y][new_pos.x] == '#' {
                continue;
            }

            let new_distance = current_distance + 1;
            let mut new_path = current_path.clone();
            new_path.push(dir.to_char());

            if visited[new_pos.y][new_pos.x].is_none()
                || visited[new_pos.y][new_pos.x] == Some(new_distance)
            {
                // If it's not visited or if visited at the same distance, add the new path
                visited[new_pos.y][new_pos.x] = Some(new_distance);

                // Enqueue the position for further exploration if it's the shortest path
                if visited[new_pos.y][new_pos.x] == Some(new_distance) {
                    queue.push_back((new_pos, new_path.clone()));
                    paths.entry(new_pos).or_insert_with(Vec::new).push(new_path);
                }
            }
        }
    }

    paths
        .values_mut()
        .for_each(|paths| paths.iter_mut().for_each(|path| path.push('A')));

    paths
}

/// Computes the shortest paths from all positions to all other positions as sequences of `Dir`.
fn shortest_paths(map: &[&[char]]) -> PathsMap {
    let height = map.len();
    let width = map[0].len();

    let mut paths_map = HashMap::new();

    for start_y in 0..height {
        for start_x in 0..width {
            let from = map[start_y][start_x];
            if from == '#' {
                continue;
            }

            let start = Vec2::new(start_x, start_y);
            let local_paths = bfs_paths(map, start);

            for (to_pos, paths) in local_paths {
                let to = map[to_pos.y][to_pos.x];
                let key = FromTo::new(from, to);
                paths_map.entry(key).or_insert_with(Vec::new).extend(paths);
            }
        }
    }

    paths_map
}

fn parse_file(file_path: &str) -> io::Result<Vec<Code>> {
    Ok(parse_2d_map(file_path)?
        .into_iter()
        .map(|x| x.try_into().unwrap())
        .collect())
}
