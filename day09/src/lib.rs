use std::fs;
use std::io;

enum Part {
    One,
    Two,
}

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[char]) -> usize {
    calculate_part(input, Part::One)
}

fn calculate_part2(input: &[char]) -> usize {
    calculate_part(input, Part::Two)
}

fn calculate_part(input: &[char], part: Part) -> usize {
    let mut vec = Vec::new();
    let mut free_space = false;
    let mut file_id = 0;

    for &ch in input {
        let block = if free_space {
            FileId::EmptySpace
        } else {
            file_id += 1;
            FileId::Id(file_id - 1)
        };

        let digit = ch.to_digit(10).unwrap_or(0) as usize;
        vec.extend(vec![block; digit]);
        free_space = !free_space;
    }

    match part {
        Part::One => rearrange_part1(&mut vec),
        Part::Two => rearrange_part2(&mut vec),
    };

    calculate_checksum(&vec)
}

fn rearrange_part1(vec: &mut Vec<FileId>) {
    while let Some(free_space_idx) = vec.iter().position(|&id| id == FileId::EmptySpace) {
        if let Some(block_idx) = vec.iter().rposition(|&id| id != FileId::EmptySpace) {
            if free_space_idx > block_idx {
                break;
            }

            vec.swap(free_space_idx, block_idx);
        }
    }
}

fn rearrange_part2(vec: &mut Vec<FileId>) {
    let mut next_file_block = find_next_file(vec, 0);
    loop {
        if let Some(empty_pos) = find_empty_space(vec, next_file_block.len()) {
            let id = vec[next_file_block.start_pos];
            if empty_pos < next_file_block.start_pos {
                vec[next_file_block.start_pos..next_file_block.end_pos].fill(FileId::EmptySpace);
                vec[empty_pos..empty_pos + next_file_block.len()].fill(id);
            }
        }

        if next_file_block.start_pos == 0 {
            break;
        }
        next_file_block = find_next_file(vec, vec.len() - next_file_block.start_pos);
    }
}

fn calculate_checksum(vec: &[FileId]) -> usize {
    vec.iter().enumerate().fold(0, |acc, (i, &id)| match id {
        FileId::EmptySpace => acc,
        FileId::Id(id) => acc + id * i,
    })
}

fn find_next_file(vec: &[FileId], skip: usize) -> FileBlock {
    let start_pos = vec
        .iter()
        .rev()
        .skip(skip)
        .position(|&id| id != FileId::EmptySpace)
        .unwrap();
    let id = vec.iter().rev().skip(skip).skip(start_pos).next().unwrap();
    let end_pos = vec
        .iter()
        .rev()
        .skip(skip)
        .enumerate()
        .skip(start_pos)
        .find(|(_, &curr_id)| curr_id != *id)
        .map_or(vec.len() - skip, |(pos, _)| pos);

    FileBlock {
        start_pos: skip + start_pos,
        end_pos: skip + end_pos,
    }
    .unrev(vec.len())
}

fn find_empty_space(vec: &[FileId], size: usize) -> Option<usize> {
    vec.iter()
        .enumerate()
        .take(vec.len() - size)
        .find(|(i, &id)| {
            id == FileId::EmptySpace
                && vec
                    .iter()
                    .skip(*i)
                    .take(size)
                    .all(|&id| id == FileId::EmptySpace)
        })
        .map(|(i, _)| i)
}

fn parse_file(file_path: &str) -> io::Result<Vec<char>> {
    fs::read_to_string(file_path).map(|contents| contents.trim_end().chars().collect())
}

#[derive(Copy, Clone)]
struct FileBlock {
    start_pos: usize,
    end_pos: usize,
}

impl FileBlock {
    fn len(&self) -> usize {
        self.end_pos - self.start_pos
    }

    fn unrev(&self, container_size: usize) -> FileBlock {
        let end_pos = container_size - 1 - self.start_pos + 1;
        if self.end_pos == container_size {
            FileBlock {
                start_pos: 0,
                end_pos,
            }
        } else {
            let start_pos = container_size - 1 - self.end_pos + 1;
            FileBlock { start_pos, end_pos }
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum FileId {
    EmptySpace,
    Id(usize),
}
