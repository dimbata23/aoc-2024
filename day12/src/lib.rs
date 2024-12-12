use shared::{dir_in, hashset_dirs_to_vec, parse_2d_map, Dir, Pos2D};
use std::collections::HashSet;
use std::io;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec<char>]) -> usize {
    let mut garden = Garden::new(input);
    garden.fence_up();
    garden.calc_price()
}

fn calculate_part2(input: &[Vec<char>]) -> usize {
    let mut garden = Garden::new(input);
    garden.fence_up();
    garden.calc_discount_price()
}

struct Plot {
    ch: char,
    fences: HashSet<Dir>,
    region: Option<Region>,
}

type Pos2 = Pos2D<usize>;

#[derive(Copy, Clone, Debug)]
struct Region {
    ch: char,
    pos: Pos2,
}

struct Garden {
    matrix: Vec<Vec<Plot>>,
    regions: Vec<Region>,
}

#[derive(Copy, Clone)]
struct Neighbour {
    pos: Pos2,
    dir: Dir,
}

impl Garden {
    fn new(input: &[Vec<char>]) -> Garden {
        let matrix = input
            .iter()
            .map(|line| line.iter().map(|&ch| Plot::from_char(ch)).collect())
            .collect();
        Garden {
            matrix,
            regions: vec![],
        }
    }

    fn fence_up(&mut self) {
        for row in 0..self.matrix.len() {
            for col in 0..self.matrix[row].len() {
                self.fence_region_at_pos(Pos2::new(row, col));
            }
        }
    }

    fn calc_price(&self) -> usize {
        self.regions
            .iter()
            .map(|&region| self.calc_region_price(region))
            .sum()
    }

    fn calc_discount_price(&self) -> usize {
        self.regions
            .iter()
            .map(|&region| self.calc_region_discount_price(region))
            .sum()
    }

    fn calc_region_price(&self, region: Region) -> usize {
        self.calc_area(region) * self.calc_perim(region)
    }

    fn calc_region_discount_price(&self, region: Region) -> usize {
        let area = self.calc_area(region);
        let sides = self.calc_sides(region.pos);
        println!(
            "A region of {} plants with price {} * {} = {}",
            region.ch,
            area,
            sides,
            area * sides
        );
        area * sides
    }

    fn calc_area(&self, region: Region) -> usize {
        let mut area = 0;
        let mut visited = HashSet::new();
        self.calc_area_rec(region.pos, region.ch, &mut area, &mut visited);
        area
    }

    fn calc_perim(&self, region: Region) -> usize {
        let mut perim = 0;
        let mut visited = HashSet::new();
        self.calc_perim_rec(region.pos, region.ch, &mut perim, &mut visited);
        perim
    }

    fn calc_sides(&self, pos: Pos2) -> usize {
        let mut corners = 0_usize;
        let mut visited = HashSet::new();
        self.get_corners_doubled_rec(pos, &mut corners, &mut visited);
        corners / 2 // Note: #corners == #sides
    }

    fn fence_region_at_pos(&mut self, pos: Pos2) {
        let plot = self.get_plot(pos);
        if plot.region.is_none() {
            let region = Region { ch: plot.ch, pos };
            self.regions.push(region);
            self.fence_up_region_rec(pos, None, region);
        }
    }

    fn fence_up_region_rec(&mut self, pos: Pos2, from: Option<Dir>, region: Region) {
        if self.get_plot_ch(pos) != region.ch {
            return;
        }

        if let Some(dir) = from {
            let from_pos = pos.moved(dir);
            self.get_plot_mut(from_pos).fences.remove(&dir.opposite());
            self.get_plot_mut(pos).fences.remove(&dir);
        }

        if self.plot_has_region(pos) {
            return;
        }

        self.get_plot_mut(pos).region = Some(region);
        for neighbour in self.get_neighbours(pos) {
            self.fence_up_region_rec(neighbour.pos, Some(neighbour.dir), region)
        }
    }

    fn get_region_neighbours(&self, pos: Pos2, ch: char) -> Vec<Pos2> {
        self.get_neighbours(pos)
            .iter()
            .map(|&neigh| neigh.pos)
            .filter(|&pos| self.get_plot_ch(pos) == ch)
            .collect()
    }
}

impl Plot {
    fn from_char(ch: char) -> Self {
        Self {
            ch,
            fences: vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right]
                .into_iter()
                .collect(),
            region: None,
        }
    }
}

fn set_not_contains(set: Option<&HashSet<Dir>>, dir: Dir) -> bool {
    if let Some(set) = set {
        !set.contains(&dir)
    } else {
        false
    }
}

fn parse_file(file_path: &str) -> std::io::Result<Vec<Vec<char>>> {
    parse_2d_map(file_path)
}

/**************************************************************/
/******** Despicable code ahead. Proceed with caution! ********/
/**************************************************************/

impl Garden {
    fn add_corners(&self, pos: Pos2, corners: &mut usize) {
        let fences = &self.get_plot(pos).fences;
        if fences.is_empty() {
            // case 1
            return;
        }

        if fences.len() == 4 {
            // case 2
            *corners += 8;
            return;
        }

        let fences_arr = hashset_dirs_to_vec(fences);
        if dir_in(&fences_arr, Dir::Left) {
            if dir_in(&fences_arr, Dir::Up) {
                // U_L_  |----
                //       |^     case 3
                //       |
                println!("case 03: Adding top left of {:?}", pos);
                *corners += 2;
                let right_plot_fences = self.right_plot_fences(pos);
                if set_not_contains(right_plot_fences, Dir::Up) {
                    println!("case 03: Adding top right of {:?}", pos);
                    *corners += 1;
                }
                let bot_plot_fences = self.bot_plot_fences(pos);
                if set_not_contains(bot_plot_fences, Dir::Left) {
                    println!("case 03: Adding bot left of {:?}", pos);
                    *corners += 1;
                }
                if dir_in(&fences_arr, Dir::Down) {
                    // UDL_ |----
                    //      |v      case 4
                    //      |----
                    println!("case 04: Adding bot left of {:?}", pos);
                    *corners += 2;
                    if set_not_contains(right_plot_fences, Dir::Down) {
                        println!("case 04: Adding bot right of {:?}", pos);
                        *corners += 1;
                    }
                } else if dir_in(&fences_arr, Dir::Right) {
                    // U_LR  |----|
                    //       |   >|  case 5
                    //       |    |
                    println!("case 05: Adding top right of {:?}", pos);
                    *corners += 2;
                    if set_not_contains(bot_plot_fences, Dir::Right) {
                        println!("case 05: Adding bot right of {:?}", pos);
                        *corners += 1;
                    }
                }
            } else if dir_in(&fences_arr, Dir::Down) {
                // _DL_  |
                //       |v     case 6
                //       |----
                println!("case 06: Adding bot left of {:?}", pos);
                *corners += 2;
                let top_plot_fences = self.top_plot_fences(pos);
                if set_not_contains(top_plot_fences, Dir::Left) {
                    println!("case 06: Adding top left of {:?}", pos);
                    *corners += 1;
                }
                let right_plot_fences = self.right_plot_fences(pos);
                if set_not_contains(right_plot_fences, Dir::Down) {
                    println!("case 06: Adding bot right of {:?}", pos);
                    *corners += 1;
                }
                if dir_in(&fences_arr, Dir::Right) {
                    // _DLR  |    |
                    //       |   >|  case 7
                    //       |----|
                    *corners += 2;
                    println!("case 07: Adding bot right of {:?}", pos);
                    if set_not_contains(top_plot_fences, Dir::Right) {
                        println!("case 07: Adding top right of {:?}", pos);
                        *corners += 1;
                    }
                }
            } else if dir_in(&fences_arr, Dir::Left) {
                // __L_  |
                //       |<     case 8
                //       |
                let top_plot_fences = self.top_plot_fences(pos);
                if set_not_contains(top_plot_fences, Dir::Left) {
                    println!("case 08: Adding top left of {:?}", pos);
                    *corners += 1;
                }
                let bot_plot_fences = self.bot_plot_fences(pos);
                if set_not_contains(bot_plot_fences, Dir::Left) {
                    println!("case 08: Adding bot left of {:?}", pos);
                    *corners += 1;
                }
                if dir_in(&fences_arr, Dir::Right) {
                    // __LR  |    |
                    //       |   >|  case 9
                    //       |    |
                    if set_not_contains(top_plot_fences, Dir::Right) {
                        println!("case 09: Adding top right of {:?}", pos);
                        *corners += 1;
                    }
                    if set_not_contains(bot_plot_fences, Dir::Right) {
                        println!("case 09: Adding bot right of {:?}", pos);
                        *corners += 1;
                    }
                }
            }
        } else if dir_in(&fences_arr, Dir::Right) {
            if dir_in(&fences_arr, Dir::Up) {
                // U__R  -----|
                //           ^|  case 10
                //            |
                println!("case 10: Adding top right of {:?}", pos);
                *corners += 2;
                let left_plot_fences = self.left_plot_fences(pos);
                if set_not_contains(left_plot_fences, Dir::Up) {
                    println!("case 10: Adding top left of {:?}", pos);
                    *corners += 1;
                }
                let bot_plot_fences = self.bot_plot_fences(pos);
                if set_not_contains(bot_plot_fences, Dir::Right) {
                    println!("case 10: Adding bot right of {:?}", pos);
                    *corners += 1;
                }
                if dir_in(&fences_arr, Dir::Down) {
                    // UD_R  -----|
                    //           v|  case 11
                    //       -----|
                    println!("case 11: Adding bot right of {:?}", pos);
                    *corners += 2;
                    if set_not_contains(left_plot_fences, Dir::Down) {
                        println!("case 11: Adding bot left of {:?}", pos);
                        *corners += 1;
                    }
                }
            } else if dir_in(&fences_arr, Dir::Down) {
                // _D_R      |
                //          v|  case 12
                //       ----|
                println!("case 12: Adding bot right of {:?}", pos);
                *corners += 2;
                let left_plot_fences = self.left_plot_fences(pos);
                if set_not_contains(left_plot_fences, Dir::Down) {
                    println!("case 12: Adding bot left of {:?}", pos);
                    *corners += 1;
                }
                let top_plot_fences = self.top_plot_fences(pos);
                if set_not_contains(top_plot_fences, Dir::Right) {
                    println!("case 12: Adding top right of {:?}", pos);
                    *corners += 1;
                }
            } else if fences.len() == 1 {
                // ___R      |
                //          >|  case 13
                //           |
                let top_plot_fences = self.top_plot_fences(pos);
                if set_not_contains(top_plot_fences, Dir::Right) {
                    println!("case 13: Adding top right of {:?}", pos);
                    *corners += 1;
                }
                let bot_plot_fences = self.bot_plot_fences(pos);
                if set_not_contains(bot_plot_fences, Dir::Right) {
                    println!("case 13: Adding bot right of {:?}", pos);
                    *corners += 1;
                }
            }
        } else if dir_in(&fences_arr, Dir::Up) {
            // U___  -------
            //          ^     case 14
            //
            let left_plot_fences = self.left_plot_fences(pos);
            if set_not_contains(left_plot_fences, Dir::Up) {
                println!("case 14: Adding top left of {:?}", pos);
                *corners += 1;
            }
            let right_plot_fences = self.right_plot_fences(pos);
            if set_not_contains(right_plot_fences, Dir::Up) {
                println!("case 14: Adding top right of {:?}", pos);
                *corners += 1;
            }
            if dir_in(&fences_arr, Dir::Down) {
                // UD__  -------
                //          v     case 15
                //       -------
                if set_not_contains(left_plot_fences, Dir::Down) {
                    println!("case 15: Adding bot left of {:?}", pos);
                    *corners += 1;
                }
                if set_not_contains(right_plot_fences, Dir::Down) {
                    println!("case 15: Adding bot right of {:?}", pos);
                    *corners += 1;
                }
            }
        } else if dir_in(&fences_arr, Dir::Down) {
            // _D__
            //          v     case 16
            //       -------
            let left_plot_fences = self.left_plot_fences(pos);
            if set_not_contains(left_plot_fences, Dir::Down) {
                println!("case 16: Adding bot left of {:?}", pos);
                *corners += 1;
            }
            let right_plot_fences = self.right_plot_fences(pos);
            if set_not_contains(right_plot_fences, Dir::Down) {
                println!("case 16: Adding bot right of {:?}", pos);
                *corners += 1;
            }
        }
    }

    fn left_plot_fences(&self, pos: Pos2) -> Option<&HashSet<Dir>> {
        if pos.col > 0 {
            if self.get_plot_ch(pos) != self.get_plot_ch(pos.left()) {
                None
            } else {
                Some(&self.get_plot(pos.left()).fences)
            }
        } else {
            None
        }
    }

    fn right_plot_fences(&self, pos: Pos2) -> Option<&HashSet<Dir>> {
        if pos.col < self.matrix[pos.row].len() - 1 {
            if self.get_plot_ch(pos) != self.get_plot_ch(pos.right()) {
                None
            } else {
                Some(&self.get_plot(pos.right()).fences)
            }
        } else {
            None
        }
    }

    fn top_plot_fences(&self, pos: Pos2) -> Option<&HashSet<Dir>> {
        if pos.row > 0 {
            if self.get_plot_ch(pos) != self.get_plot_ch(pos.up()) {
                None
            } else {
                Some(&self.get_plot(pos.up()).fences)
            }
        } else {
            None
        }
    }

    fn bot_plot_fences(&self, pos: Pos2) -> Option<&HashSet<Dir>> {
        if pos.row < self.matrix.len() - 1 {
            if self.get_plot_ch(pos) != self.get_plot_ch(pos.down()) {
                None
            } else {
                Some(&self.get_plot(pos.down()).fences)
            }
        } else {
            None
        }
    }

    fn calc_area_rec(&self, pos: Pos2, ch: char, area: &mut usize, visited: &mut HashSet<Pos2>) {
        if !visited.insert(pos) {
            return;
        }

        *area += 1;

        for neighbour in self.get_region_neighbours(pos, ch) {
            self.calc_area_rec(neighbour, ch, area, visited);
        }
    }

    fn calc_perim_rec(&self, pos: Pos2, ch: char, perim: &mut usize, visited: &mut HashSet<Pos2>) {
        if !visited.insert(pos) {
            return;
        }

        *perim += self.get_plot_fences_cnt(pos);

        for neighbour in self.get_region_neighbours(pos, ch) {
            self.calc_perim_rec(neighbour, ch, perim, visited);
        }
    }

    fn get_corners_doubled_rec(&self, pos: Pos2, corners: &mut usize, visited: &mut HashSet<Pos2>) {
        if !visited.insert(pos) {
            return;
        }

        self.add_corners(pos, corners);

        let neighbours = self.get_region_neighbours(pos, self.get_plot_ch(pos));
        for neighbour in neighbours {
            self.get_corners_doubled_rec(neighbour, corners, visited);
        }
    }

    fn get_plot_mut(&mut self, pos: Pos2) -> &mut Plot {
        &mut self.matrix[pos.row][pos.col]
    }

    fn get_plot(&self, pos: Pos2) -> &Plot {
        &self.matrix[pos.row][pos.col]
    }

    fn get_plot_fences_cnt(&self, pos: Pos2) -> usize {
        self.get_plot(pos).fences.len()
    }

    fn get_plot_ch(&self, pos: Pos2) -> char {
        self.get_plot(pos).ch
    }

    fn plot_has_region(&self, pos: Pos2) -> bool {
        self.get_plot(pos).region.is_some()
    }

    fn get_neighbours(&self, pos: Pos2) -> Vec<Neighbour> {
        let mut res = vec![];

        if pos.row > 0 {
            res.push(Neighbour {
                pos: pos.moved(Dir::Up),
                dir: Dir::Down,
            });
        }

        if pos.col > 0 {
            res.push(Neighbour {
                pos: pos.moved(Dir::Left),
                dir: Dir::Right,
            });
        }

        if pos.row < self.matrix.len() - 1 {
            res.push(Neighbour {
                pos: pos.moved(Dir::Down),
                dir: Dir::Up,
            });
        }

        if pos.col < self.matrix[pos.row].len() - 1 {
            res.push(Neighbour {
                pos: pos.moved(Dir::Right),
                dir: Dir::Left,
            });
        }

        res
    }
}
