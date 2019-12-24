use std::collections::HashMap;
use std::collections::HashSet;

fn state_to_str(state: &HashMap<(isize, isize), bool>) -> String {
    let mut s = "".to_string();
    for y in 0..5 {
        for x in 0..5 {
            match state.get(&(x, y)).unwrap() {
                true => s += "#",
                false => s += ".",
            };
        }
        s += "\n";
    }
    s
}

fn bio_rating(state: HashMap<(isize, isize), bool>) -> u32 {
    let mut total = 0;

    for ((x, y), b) in state {
        if b {
            total += 2_u32.pow(x as u32 + y as u32 * 5);
        }
    }

    total
}

fn part1(state: HashMap<(isize, isize), bool>) -> u32 {
    let mut states = HashSet::new();
    states.insert(state_to_str(&state));

    let mut old_state = state;
    loop {
        let mut new_state = HashMap::new();
        for y in 0..5 {
            for x in 0..5 {
                let adj: Vec<_> = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .iter()
                    .filter_map(|p| old_state.get(&p))
                    .collect();
                let bugs = adj.iter().filter(|&&a| *a).count();
                let curr = *old_state.get(&(x, y)).unwrap();

                if bugs != 1 && curr {
                    new_state.insert((x, y), false);
                } else if (bugs == 1 || bugs == 2) && !curr {
                    new_state.insert((x, y), true);
                } else {
                    new_state.insert((x, y), curr);
                }
            }
        }
        let s = state_to_str(&new_state);
        old_state = new_state;
        if !states.insert(s) {
            break;
        };
    }
    bio_rating(old_state)
}

fn part2(state: HashMap<(isize, isize), bool>) -> usize {
    let mut current_state = state
        .iter()
        .filter_map(|(&(x, y), &b)| {
            if !(x == 2 && y == 2) && b {
                Some((x, y, 0))
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();

    for _ in 0..200 {
        let mut counts = HashMap::new();

        for &(x, y, d) in &current_state {
            match (x, y) {
                (0, 0) => {
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;

                    *counts.entry((2, 1, d - 1)).or_insert(0) += 1;
                    *counts.entry((1, 2, d - 1)).or_insert(0) += 1;
                }
                (4, 0) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;

                    *counts.entry((3, 2, d - 1)).or_insert(0) += 1;
                    *counts.entry((2, 1, d - 1)).or_insert(0) += 1;
                }
                (0, 4) => {
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;

                    *counts.entry((1, 2, d - 1)).or_insert(0) += 1;
                    *counts.entry((2, 3, d - 1)).or_insert(0) += 1;
                }
                (4, 4) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;

                    *counts.entry((3, 2, d - 1)).or_insert(0) += 1;
                    *counts.entry((2, 3, d - 1)).or_insert(0) += 1;
                }
                (0, _) => {
                    *counts.entry((1, 2, d - 1)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;
                }
                (4, _) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((3, 2, d - 1)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;
                }
                (_, 0) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((2, 1, d - 1)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;
                }
                (_, 4) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((2, 3, d - 1)).or_insert(0) += 1;
                }
                (1, 2) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;

                    for y_ in 0..5 {
                        *counts.entry((0, y_, d + 1)).or_insert(0) += 1;
                    }
                }
                (3, 2) => {
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;

                    for y_ in 0..5 {
                        *counts.entry((4, y_, d + 1)).or_insert(0) += 1;
                    }
                }
                (2, 1) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;

                    for x_ in 0..5 {
                        *counts.entry((x_, 0, d + 1)).or_insert(0) += 1;
                    }
                }
                (2, 3) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;

                    for x_ in 0..5 {
                        *counts.entry((x_, 4, d + 1)).or_insert(0) += 1;
                    }
                }
                (x, y) => {
                    *counts.entry((x - 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x + 1, y, d)).or_insert(0) += 1;
                    *counts.entry((x, y - 1, d)).or_insert(0) += 1;
                    *counts.entry((x, y + 1, d)).or_insert(0) += 1;
                }
            }
        }

        current_state = counts
            .iter()
            .filter_map(|(&p, &c)| {
                if current_state.contains(&p) {
                    if c == 1 {
                        return Some(p);
                    }
                } else if c == 1 || c == 2 {
                    return Some(p);
                }
                None
            })
            .collect();
    }

    current_state.len()
}

fn main() {
    let input = include_str!("../input");

    let state: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), c == '#'))
        })
        .collect();

    println!("Part 1: {}", part1(state.clone()));
    println!("Part 2: {}", part2(state));
}
