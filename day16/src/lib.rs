use shared::{parse_2d_map, Dir, Vec2D};
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io,
};
use strum::IntoEnumIterator;

type Vec2 = Vec2D<usize>;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &Maze) -> usize {
    let (dists, _) = calc_min_price_to_end(&input);
    min_from_all_dirs(&dists, input.target_pos)
}

fn calculate_part2(input: &Maze) -> usize {
    let (dists, parents) = calc_min_price_to_end(&input);
    let min_dist = min_from_all_dirs(&dists, input.target_pos);
    count_best_spots(&parents, input.target_pos, &dists, min_dist)
}

fn min_from_all_dirs(dists: &HashMap<CellDir, usize>, pos: Vec2) -> usize {
    let start = CellDir::new(pos, Dir::Right);
    let mut min = dists[&start];
    for dir in Dir::iter() {
        let cell_dir = CellDir::new(pos, dir);
        let curr_dist = dists[&cell_dir];
        if curr_dist < min {
            min = curr_dist;
        }
    }
    min
}

struct Maze {
    map: Vec<Vec<char>>,
    pos: Vec2,
    target_pos: Vec2,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct CellDir {
    pos: Vec2,
    dir: Dir,
}

fn count_best_spots(
    parents: &HashMap<CellDir, Vec<CellDir>>,
    end_pos: Vec2,
    dists: &HashMap<CellDir, usize>,
    min_dist: usize,
) -> usize {
    let mut visited_cells = HashSet::new();
    for dir in Dir::iter() {
        let cell_dir = CellDir::new(end_pos, dir);
        let curr_dist = dists[&cell_dir];
        if curr_dist != min_dist {
            continue;
        }
        let mut visited = HashSet::new();
        count_best_spots_rec(parents, cell_dir, &mut visited, &mut visited_cells);
    }
    visited_cells.len()
}

fn count_best_spots_rec(
    parents: &HashMap<CellDir, Vec<CellDir>>,
    cell_dir: CellDir,
    visited: &mut HashSet<CellDir>,
    visited_cells: &mut HashSet<Vec2>,
) {
    visited_cells.insert(cell_dir.pos);

    if !visited.insert(cell_dir) {
        return;
    }

    if let Some(curr_parents) = parents.get(&cell_dir) {
        for &parent in curr_parents {
            count_best_spots_rec(parents, parent, visited, visited_cells);
        }
    }
}

fn calc_min_price_to_end(maze: &Maze) -> (HashMap<CellDir, usize>, HashMap<CellDir, Vec<CellDir>>) {
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let mut heap = BinaryHeap::new();

    // push all non-wall cells
    for (y, line) in maze.map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == '#' {
                continue;
            }
            for dir in Dir::iter() {
                distances.insert(CellDir::new(Vec2::new(x, y), dir), usize::MAX);
            }
        }
    }

    // start 0 dist
    let start_cell_dir = CellDir::new(maze.pos, Dir::Right);
    if let Some(dist) = distances.get_mut(&start_cell_dir) {
        *dist = 0;
    }

    // push start in heap
    heap.push(MinPriorityPos {
        pos: maze.pos,
        dir: Dir::Right,
        dist: 0,
    });

    while let Some(curr) = heap.pop() {
        let curr_cell_dir = CellDir::new(curr.pos, curr.dir);
        if curr.dist > distances[&curr_cell_dir] {
            continue;
        }

        let neighbours = gen_neighbours(&maze.map, &curr);
        for neighbour in neighbours {
            let neighbour_cell_dir = CellDir::new(neighbour.pos, neighbour.dir);
            let this_dist = neighbour.dist;
            let other_dist = distances[&neighbour_cell_dir];

            if this_dist < other_dist {
                distances.insert(neighbour_cell_dir, neighbour.dist);
                parents.insert(neighbour_cell_dir, vec![curr_cell_dir]);
                heap.push(neighbour);
            }

            if this_dist == other_dist {
                let neigh_parents = parents.entry(neighbour_cell_dir).or_default();
                neigh_parents.push(curr_cell_dir);
            }
        }
    }

    (distances, parents)
}

fn gen_neighbours(map: &[Vec<char>], cell: &MinPriorityPos) -> Vec<MinPriorityPos> {
    let mut neighbours = vec![];

    if map[cell.pos.y][cell.pos.x] == 'E' {
        return neighbours;
    }

    let (forward_dir, left_dir, right_dir) = (
        cell.dir,
        cell.dir.rotated_90_ccw(),
        cell.dir.rotated_90_cw(),
    );

    let (forward_pos, left_pos, right_pos) = (
        cell.pos.moved(forward_dir),
        cell.pos.moved(left_dir),
        cell.pos.moved(right_dir),
    );

    let (forward_ch, left_ch, right_ch) = (
        map[forward_pos.y][forward_pos.x],
        map[left_pos.y][left_pos.x],
        map[right_pos.y][right_pos.x],
    );

    if forward_ch != '#' {
        neighbours.push(MinPriorityPos {
            pos: forward_pos,
            dir: forward_dir,
            dist: cell.dist + 1,
        });
    }

    if left_ch != '#' {
        neighbours.push(MinPriorityPos {
            pos: cell.pos,
            dir: left_dir,
            dist: cell.dist + 1000,
        });
    }

    if right_ch != '#' {
        neighbours.push(MinPriorityPos {
            pos: cell.pos,
            dir: right_dir,
            dist: cell.dist + 1000,
        });
    }

    neighbours
}

#[derive(Copy, Clone)]
struct MinPriorityPos {
    pos: Vec2,
    dist: usize,
    dir: Dir,
}

impl Eq for MinPriorityPos {}

impl PartialEq for MinPriorityPos {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
    }
}

impl Ord for MinPriorityPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for MinPriorityPos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl CellDir {
    fn new(pos: Vec2, dir: Dir) -> Self {
        Self { pos, dir }
    }
}

fn parse_file(file_path: &str) -> io::Result<Maze> {
    let map = parse_2d_map(file_path)?;
    let mut start = Vec2::new(1, 1);
    let mut target = Vec2::new(1, 1);

    for (y, line) in map.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            match ch {
                'S' => start = Vec2::new(x, y),
                'E' => target = Vec2::new(x, y),
                _ => (),
            }
        }
    }

    Ok(Maze {
        map,
        pos: start,
        target_pos: target,
    })
}
