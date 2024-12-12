use shared::{parse_2d_map, Pos2D, Vec2D};
use std::collections::HashSet;
use std::io;

type CoordType = i64;
type Pos2 = Pos2D<CoordType>;
type Vec2 = Vec2D<CoordType>;
type Limit = Pos2;

#[derive(Copy, Clone)]
enum Extended {
    True,
    False,
}

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[Vec<char>]) -> usize {
    let limit = Limit::new(input.len() as CoordType, input[0].len() as CoordType);
    let antennas = get_antennas(&input);
    let antinodes = get_antinodes(&antennas, limit, Extended::False);
    print_antinodes(input, &antinodes);
    antinodes.len()
}

fn calculate_part2(input: &[Vec<char>]) -> usize {
    let limit = Limit::new(input.len() as CoordType, input[0].len() as CoordType);
    let antennas = get_antennas(&input);
    let antinodes = get_antinodes(&antennas, limit, Extended::True);
    print_antinodes(input, &antinodes);
    antinodes.len()
}

fn get_antinodes(antennas: &[Antenna], limit: Limit, extended: Extended) -> HashSet<Antinode> {
    let mut set = HashSet::new();

    for antenna1 in antennas {
        for antenna2 in antennas {
            if antenna1 == antenna2 {
                continue;
            }

            let antinodes = antenna1.get_antinodes(antenna2, limit, extended);
            for antinode in antinodes {
                set.insert(antinode);
            }

            if let Extended::True = extended {
                set.insert(Antinode::from_pos(antenna1.pos));
            }
        }
    }

    set
}

fn get_antennas(input: &[Vec<char>]) -> Vec<Antenna> {
    let mut res = vec![];
    for (row, line) in input.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch != '.' {
                res.push(Antenna::new(row, col, ch));
            }
        }
    }
    res
}

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    parse_2d_map(file_path)
}

#[derive(PartialEq)]
struct Antenna {
    pos: Pos2,
    ch: char,
}

impl Antenna {
    fn new(row: usize, col: usize, ch: char) -> Self {
        Self {
            pos: Pos2::new(row as CoordType, col as CoordType),
            ch,
        }
    }

    fn get_antinodes(&self, other: &Self, limit: Limit, extended: Extended) -> Vec<Antinode> {
        if self.ch != other.ch {
            return vec![];
        }

        let vec1 = self.pos.make_vec_to(other.pos);
        let vec2 = other.pos.make_vec_to(self.pos);

        let antinode1 = Antinode::from_pos_limited((vec1 + other.pos).to_pos(), limit);
        let antinode2 = Antinode::from_pos_limited((vec2 + self.pos).to_pos(), limit);

        let mut res = vec![];

        if let Some(node1) = antinode1 {
            res.push(node1);
        }

        if let Some(node2) = antinode2 {
            res.push(node2);
        }

        if let Extended::True = extended {
            let mut add_antinode_sequence = |mut node_opt: Option<Antinode>, vec: Vec2| {
                while let Some(node) = node_opt {
                    res.push(node);
                    node_opt = Antinode::from_pos_limited((vec + node.pos).to_pos(), limit);
                }
            };

            add_antinode_sequence(antinode1, vec1);
            add_antinode_sequence(antinode2, vec2);
        }

        res
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Antinode {
    pos: Pos2,
}

impl Antinode {
    fn new(row: usize, col: usize) -> Self {
        Self {
            pos: Pos2::new(row as CoordType, col as CoordType),
        }
    }

    fn from_pos(pos: Pos2) -> Self {
        Self { pos }
    }

    fn from_pos_limited(pos: Pos2, limit: Limit) -> Option<Self> {
        if pos.row >= 0 && pos.col >= 0 && pos.row < limit.row && pos.col < limit.col {
            Some(Antinode::from_pos(pos))
        } else {
            None
        }
    }
}

fn print_antinodes(input: &[Vec<char>], antinodes: &HashSet<Antinode>) {
    for (row, line) in input.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if antinodes.contains(&Antinode::new(row, col)) {
                print!("#");
            } else {
                print!("{ch}");
            }
        }
        println!();
    }
}
