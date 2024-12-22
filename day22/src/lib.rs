use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;

type QuadDeltas = [i64; 4];
const DEPTH: usize = 2000;

pub fn run() -> io::Result<()> {
    let input = parse_file("input")?;
    let res_part1 = calculate_part1(&input);
    println!("Part one result: {res_part1}");

    let res_part2 = calculate_part2(&input);
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &[u64]) -> u64 {
    let vec: Vec<_> = input
        .iter()
        .map(|&secret| gen_new_secret(secret, DEPTH))
        .collect();

    vec.iter().sum()
}

fn calculate_part2(input: &[u64]) -> usize {
    let deltas = gen_quad_deltas(input, DEPTH);
    let progress_bar = ProgressBar::new(deltas.len() as u64);

    // Brute-force that mf in parallel, but make it pretty :)
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    deltas
        .par_iter()
        .map(|&deltas| {
            progress_bar.inc(1);
            calc_bananas(input, DEPTH, &deltas)
        })
        .max()
        .unwrap_or(0)
}

fn calc_bananas(secrets: &[u64], depth: usize, deltas: &QuadDeltas) -> usize {
    secrets
        .iter()
        .map(|&secret| buy_bananas(secret, depth, deltas))
        .filter_map(|x| x)
        .sum()
}

fn buy_bananas(secret: u64, depth: usize, target_deltas: &QuadDeltas) -> Option<usize> {
    let prices = gen_prices(secret, depth);
    let buyer_deltas = gen_price_deltas(secret, depth);
    for (idx, window) in buyer_deltas.windows(4).enumerate() {
        if target_deltas == window {
            return Some(prices[idx + 4] as usize);
        }
    }

    None
}

fn gen_quad_deltas(secrets: &[u64], depth: usize) -> HashSet<QuadDeltas> {
    secrets
        .iter()
        .flat_map(|&secret| {
            gen_price_deltas(secret, depth)
                .windows(4)
                .filter_map(|window| window.try_into().ok())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn gen_prices(secret: u64, depth: usize) -> Vec<i64> {
    let mut prev_secret = secret;
    let mut vec = vec![0; depth + 1];
    vec[0] = price(secret);
    for i in 1..=depth {
        let new_secret = gen_new_secret(prev_secret, 1);
        vec[i] = price(new_secret);
        prev_secret = new_secret;
    }
    vec
}

fn gen_price_deltas(secret: u64, depth: usize) -> Vec<i64> {
    let prices = gen_prices(secret, depth);
    prices
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn price(secret: u64) -> i64 {
    secret as i64 % 10
}

fn gen_new_secret(num: u64, depth: usize) -> u64 {
    if depth == 0 {
        return num;
    }

    let mut secret = num;
    secret = mix(secret, secret * 64);
    secret = prune(secret);

    secret = mix(secret, secret / 32);
    secret = prune(secret);

    secret = mix(secret, secret * 2048);
    secret = prune(secret);

    //println!("{secret}");

    gen_new_secret(secret, depth - 1)
}

fn mix(num: u64, other: u64) -> u64 {
    num ^ other
}

fn prune(num: u64) -> u64 {
    num % 16777216
}

fn parse_file(file_path: &str) -> io::Result<Vec<u64>> {
    Ok(File::open(file_path)
        .map(io::BufReader::new)?
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().parse::<u64>().unwrap())
        .collect())
}
