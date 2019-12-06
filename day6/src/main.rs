use std::collections::HashMap;

fn counter(mapping: &HashMap<String, String>, k: &String) -> usize {
    let mut count = 0;
    let mut search = k;

    loop {
        let found = mapping.get(search);
        match &found {
            None => break,
            Some(v) => {
                count += 1;
                search = &v;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("../input");

    let mapping = input
        .lines()
        .map(|s| (s[4..7].to_string(), s[0..3].to_string()))
        .collect::<HashMap<_, _>>();

    println!(
        "Part 1: {}",
        mapping.keys().map(|k| counter(&mapping, &k)).sum::<usize>()
    );
}
