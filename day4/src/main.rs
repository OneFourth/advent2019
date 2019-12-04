use std::collections::HashMap;

fn is_valid_part1(s: usize) -> bool {
    let val: Vec<_> = s
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    val.windows(2).all(|n| n[0] <= n[1]) && val.windows(2).any(|n| n[0] == n[1])
}

fn is_valid_part2(s: usize) -> bool {
    let val: Vec<_> = s
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut counts = HashMap::new();

    println!(
        "{:?}",
        val.windows(2).filter(|n| n[0] == n[1]).collect::<Vec<_>>()
    );
    for n in val.windows(2) {
        if n[0] == n[1] {
            *counts.entry(n[0]).or_insert(0) += 1;
        }
    }

    val.windows(2).all(|n| n[0] <= n[1]) && counts.values().any(|&x| x == 1)
}

fn main() {
    let input = include_str!("../input");
    let split: Vec<_> = input
        .split('-')
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    let (low, high) = (split[0], split[1]);

    println!(
        "Part 1: {}",
        (low..=high).filter(|&n| is_valid_part1(n)).count()
    );
    println!(
        "Part 2: {}",
        (low..=high).filter(|&n| is_valid_part2(n)).count()
    );
}
