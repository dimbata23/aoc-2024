use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &InputData) -> usize {
    let mut tree = gen_towel_tree(&input.towels);
    input
        .patterns
        .iter()
        .map(|pattern| tree.ways_to_be_done(pattern))
        .filter(|&x| x > 0)
        .count()
}

fn calculate_part2(input: &InputData) -> usize {
    let mut tree = gen_towel_tree(&input.towels);
    input
        .patterns
        .iter()
        .map(|pattern| tree.ways_to_be_done(pattern))
        .sum()
}

fn gen_towel_tree(towels: &[Vec<char>]) -> TowelTree {
    let mut tree = TowelTree::new();
    for towel in towels {
        tree.add_towel(towel);
    }
    tree
}

#[derive(Debug)]
struct TreeNode {
    prefix: char,
    children: BTreeMap<char, Box<TreeNode>>,
    is_word_end_node: bool,
}

impl TreeNode {
    fn new(prefix: char) -> Self {
        Self {
            prefix,
            children: BTreeMap::new(),
            is_word_end_node: false,
        }
    }

    fn try_add_suffix(&mut self, towel: &[char]) -> bool {
        if towel.is_empty() {
            return true;
        }

        if towel[0] != self.prefix {
            return false;
        }

        if towel.len() == 1 {
            self.is_word_end_node = true;
            return true;
        }

        let node = self
            .children
            .entry(towel[1])
            .or_insert(Box::new(TreeNode::new(towel[1])));
        node.try_add_suffix(&towel[1..])
    }

    fn read_word<'a>(&self, word: &'a [char]) -> Vec<&'a [char]> {
        if word.is_empty() {
            return vec![word];
        }

        let suffix = &word[1..];

        let mut vec = vec![];
        if self.is_word_end_node {
            vec.push(suffix);
        }

        if suffix.is_empty() {
            return vec;
        }

        if let Some(node) = self.children.get(&suffix[0]) {
            let mut other_vec = node.read_word(suffix);
            vec.append(&mut other_vec)
        }

        vec
    }
}

#[derive(Debug)]
struct TowelTree {
    starting_prefixes: BTreeMap<char, TreeNode>,
    cached: HashMap<Vec<char>, usize>,
}

impl TowelTree {
    fn new() -> Self {
        Self {
            starting_prefixes: BTreeMap::new(),
            cached: HashMap::new(),
        }
    }

    fn add_towel(&mut self, towel: &[char]) {
        if towel.is_empty() {
            return;
        }

        let node = self
            .starting_prefixes
            .entry(towel[0])
            .or_insert(TreeNode::new(towel[0]));
        node.try_add_suffix(towel);
    }

    fn ways_to_be_done(&mut self, pattern: &[char]) -> usize {
        if let Some(&cache_res) = self.cached.get(pattern) {
            return cache_res;
        }

        if pattern.is_empty() {
            self.cached.insert(pattern.to_vec(), 1);
            return 1;
        }

        if let Some(node) = self.starting_prefixes.get(&pattern[0]) {
            let vec = node.read_word(pattern);
            let total_ways = vec.iter().map(|suffix| self.ways_to_be_done(suffix)).sum();
            self.cached.insert(pattern.to_vec(), total_ways);
            total_ways
        } else {
            self.cached.insert(pattern.to_vec(), 0);
            0
        }
    }
}

#[derive(Debug)]
struct InputData {
    towels: Vec<Vec<char>>,
    patterns: Vec<Vec<char>>,
}

fn parse_file(file_path: &str) -> io::Result<InputData> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines_iter = reader.lines();

    let towels_line = lines_iter.next().unwrap()?;
    let towels: Vec<Vec<_>> = towels_line
        .split(", ")
        .map(|s| s.chars().collect())
        .collect();

    // Skip the empty line
    lines_iter.next();

    let patterns: Vec<Vec<_>> = lines_iter
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(InputData { towels, patterns })
}
