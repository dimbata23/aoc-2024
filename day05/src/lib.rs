use std::collections::{HashMap, HashSet};
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

fn calculate_part1(input: &Input) -> u64 {
    input
        .updates
        .iter()
        .filter(|update| is_corrrectly_ordered(update, &input.order))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn calculate_part2(input: &Input) -> u64 {
    let mut incorrect_updates: Vec<_> = input
        .updates
        .iter()
        .filter(|update| !is_corrrectly_ordered(update, &input.order))
        .cloned()
        .collect();

    for update in incorrect_updates.iter_mut() {
        update.sort_by(|n1, n2| match (input.order.get(n1), input.order.get(n2)) {
            (Some(set), _) if set.contains(n2) => std::cmp::Ordering::Less,
            (_, Some(set)) if set.contains(n1) => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Equal,
        });
    }

    incorrect_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn is_corrrectly_ordered(update: &[u64], order: &Order) -> bool {
    for (i, val) in update.iter().enumerate().skip(1) {
        if let Some(set_afters) = order.get(val) {
            if update.iter().take(i).any(|prev| set_afters.contains(prev)) {
                return false;
            }
        }
    }

    true
}

fn parse_file(file_path: &str) -> io::Result<Input> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut order = HashMap::new();
    let mut updates = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            if let (Some(a), Some(b)) = (parts.get(0), parts.get(1)) {
                let a: u64 = a.trim().parse().unwrap();
                let b: u64 = b.trim().parse().unwrap();
                order.entry(a).or_insert_with(HashSet::new).insert(b);
            }
        } else if line.contains(',') {
            let numbers: Vec<u64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            updates.push(numbers);
        }
    }

    Ok(Input { order, updates })
}

type Order = HashMap<u64, HashSet<u64>>;
struct Input {
    order: HashMap<u64, HashSet<u64>>,
    updates: Vec<Vec<u64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        let input = parse_file("sample_input");
        assert!(input.is_ok());
        let input = input.unwrap();

        let res: Vec<_> = input
            .updates
            .iter()
            .map(|update| is_corrrectly_ordered(update, &input.order))
            .collect();
        assert_eq!(res, vec![true, true, true, false, false, false]);

        let calc = calculate_part1(&input);
        assert_eq!(calc, 143);

        let calc2 = calculate_part2(&input);
        assert_eq!(calc2, 123);
    }
}
