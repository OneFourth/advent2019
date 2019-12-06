use std::collections::HashMap;
use std::collections::HashSet;

fn counter(mapping: &HashMap<String, String>, k: &str, target: &str) -> (usize, HashSet<String>) {
    let mut count = 0;
    let mut search = k;

    let mut route = HashSet::new();

    loop {
        let found = mapping.get(search);
        match &found {
            None => panic!("Shouldn't get here anymore"),
            Some(v) => {
                count += 1;
                route.insert(v.to_string());
                if *v == target
                {
                    break;
                }
                search = &v;
            }
        }
    }

    (count, route)
}

fn main() {
    let input = include_str!("../input");

    let mapping = input
        .lines()
        .map(|s| (s[4..7].to_string(), s[0..3].to_string()))
        .collect::<HashMap<_, _>>();

    let routes: HashMap<_, _> = mapping.keys().map(|k| (k, counter(&mapping, &k, "COM"))).collect();
    println!("Part 1: {}", routes.iter().map(|t| (t.1).0 ).sum::<usize>());

    let you = &routes[&mapping["YOU"]].1;
    let san = &routes[&mapping["SAN"]].1;

    let min: usize = you.intersection(&san).map(|s| counter(&mapping, &mapping["YOU"], &s).0 + counter(&mapping, &mapping["SAN"], &s).0).min().unwrap();
    println!("Part 2: {}", min);
}
