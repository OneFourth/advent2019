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
                let adj: Vec<_> = [(x-1, y), (x+1, y), (x, y-1), (x, y+1)].into_iter().filter_map(|p| old_state.get(&p)).collect();
                let bugs = adj.iter().filter(|&&a| *a).count();
                let curr = *old_state.get(&(x, y)).unwrap();

                if bugs != 1 && curr {
                    new_state.insert((x, y), false);
                }
                else if (bugs == 1 || bugs == 2) && !curr {
                    new_state.insert((x, y), true);
                }
                else {
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

fn main() {
    let input = include_str!("../input");

    let state: HashMap<_, _> = input.lines().enumerate().flat_map(|(y, l)| {
        l.trim().chars().enumerate().map(move |(x, c)| {
            ((x as isize, y as isize), c == '#')
        })
    }).collect();

    println!("{}", part1(state));
}
