use shared::{parse_2d_map, print_2d_map, Dir, Vec2D};
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
    //let mut visited_cells = HashSet::new();
    //for dir in Dir::iter() {
    //    let mut visited = HashSet::new();
    //    let curr_dist = dists[&(input.target_pos, dir)];
    //    if curr_dist != min_dist {
    //        continue;
    //    }
    //    count_best_spots_rec(
    //        &parents,
    //        &(input.target_pos, dir),
    //        &mut visited,
    //        &mut visited_cells,
    //    );
    //}
    //let mut map = input.map.to_vec();
    //visited_cells.iter().for_each(|&pos| {
    //    map[pos.y][pos.x] = 'O';
    //});
    //print_2d_map(&map);
    //visited_cells.len()
}

fn min_from_all_dirs(dists: &HashMap<(Vec2, Dir), usize>, pos: Vec2) -> usize {
    let mut min = dists[&(pos, Dir::Right)];
    for dir in Dir::iter() {
        let curr_dist = dists[&(pos, dir)];
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

fn count_best_spots(
    parents: &HashMap<(Vec2, Dir), Vec<(Vec2, Dir)>>,
    end_pos: Vec2,
    dists: &HashMap<(Vec2, Dir), usize>,
    min_dist: usize,
) -> usize {
    let mut visited_cells = HashSet::new();
    for dir in Dir::iter() {
        let curr_dist = dists[&(end_pos, dir)];
        if curr_dist != min_dist {
            continue;
        }
        let mut visited = HashSet::new();
        count_best_spots_rec(parents, &(end_pos, dir), &mut visited, &mut visited_cells);
    }
    visited_cells.len()
}

fn count_best_spots_rec(
    parents: &HashMap<(Vec2, Dir), Vec<(Vec2, Dir)>>,
    (curr_pos, curr_dir): &(Vec2, Dir),
    visited: &mut HashSet<(Vec2, Dir)>,
    visited_cells: &mut HashSet<Vec2>,
) {
    visited_cells.insert(*curr_pos);

    if !visited.insert((*curr_pos, *curr_dir)) {
        return;
    }

    if let Some(curr_parents) = parents.get(&(*curr_pos, *curr_dir)) {
        for parent in curr_parents {
            count_best_spots_rec(parents, parent, visited, visited_cells);
        }
    }
}

fn calc_min_price_to_end(
    maze: &Maze,
) -> (
    HashMap<(Vec2, Dir), usize>,
    HashMap<(Vec2, Dir), Vec<(Vec2, Dir)>>,
) {
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();
    let mut heap = BinaryHeap::new();

    let mut debug_map = maze.map.to_vec();

    // push all non-wall cells
    for (y, line) in maze.map.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == '#' {
                continue;
            }
            for dir in Dir::iter() {
                distances.insert((Vec2::new(x, y), dir), usize::MAX);
            }
        }
    }

    // start 0 dist
    if let Some(dist) = distances.get_mut(&(maze.pos, Dir::Right)) {
        *dist = 0;
    }

    // push start in heap
    heap.push(MinPriorityPos {
        pos: maze.pos,
        dir: Dir::Right,
        dist: 0,
    });

    while let Some(curr) = heap.pop() {
        if curr.dist > distances[&(curr.pos, curr.dir)] {
            continue;
        }

        let neighbours = gen_neighbours(&maze.map, &curr);
        for neighbour in neighbours {
            let cmp_dist = neighbour
                .dist
                .cmp(&distances[&(neighbour.pos, neighbour.dir)]);
            let is_less = cmp_dist == std::cmp::Ordering::Less;
            let is_eq = cmp_dist == std::cmp::Ordering::Equal;

            //if neighbour.pos == Vec2::new(5, 7) {
            //    println!(
            //        "[5, 7] dist from {:?}: {}, cmp against {}",
            //        curr.pos,
            //        distances[&(neighbour.pos, neighbour.dir)],
            //        neighbour.dist
            //    );
            //}

            if is_less {
                distances.insert((neighbour.pos, neighbour.dir), neighbour.dist);
                parents.insert((neighbour.pos, neighbour.dir), vec![(curr.pos, curr.dir)]);
                heap.push(neighbour);
                debug_map[neighbour.pos.y][neighbour.pos.x] = 'O';
            }
            if is_eq {
                let neigh_parents = parents.entry((neighbour.pos, neighbour.dir)).or_default();
                neigh_parents.push((curr.pos, curr.dir));
                debug_map[neighbour.pos.y][neighbour.pos.x] = '=';
                //println!("eq {:?} -> parent: {:?}", neighbour.pos, neigh_parents);
            }
        }

        debug_map[curr.pos.y][curr.pos.x] = 'X';

        //print_2d_map(&debug_map);
        //press_enter_to_continue();
    }

    //let debug_parents: HashMap<Vec2, Vec<Vec2>> = parents
    //    .iter()
    //    .map(|(pos, parents)| (pos.0, parents.iter().map(|x| x.0).collect::<Vec<Vec2>>()))
    //    .collect();
    //
    //for (pos, parents) in &debug_parents {
    //    if parents.len() == 1 {
    //        continue;
    //    }
    //
    //    println!("{:?} <- {:?}", pos, parents);
    //}

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

//fn press_enter_to_continue() {
//    println!("Press Enter to continue...");
//    // Flush stdout to ensure the prompt is displayed immediately.
//
//    // Read a line from standard input, but ignore the result.
//    let mut input = String::new();
//    io::stdin().read_line(&mut input).unwrap();
//}
