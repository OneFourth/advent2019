use std::collections::HashSet;

fn parse(input: &str) -> Vec<(f64, f64)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.trim()
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '#' => Some((x as f64, y as f64)),
                    _ => None,
                })
        })
        .collect()
}

fn angleish((ax, ay): (f64, f64), (bx, by): (f64, f64)) -> i32 {
    let degrees = ((by - ay).atan2(bx - ax).to_degrees()) + 360.0 + 90.0;
    ((degrees % 360.0) * 1000.0).trunc() as i32
}

fn part1(input: &str) -> ((f64, f64), usize) {
    let coords = parse(input);

    coords
        .iter()
        .map(|a| {
            (
                *a,
                coords
                    .iter()
                    .filter_map(move |b| if a == b { None } else { Some(angleish(*a, *b)) })
                    .collect::<HashSet<_>>()
                    .len(),
            )
        })
        .max_by(|(_, count1), (_, count2)| count1.cmp(count2))
        .unwrap()
}

fn sqdistance((ax, ay): (f64, f64), (bx, by): (f64, f64)) -> f64 {
    (ax - bx).powf(2.0) + (ay - by).powf(2.0)
}

fn part2(input: &str) -> i32 {
    let coords = parse(input);

    let part1_data = part1(input);

    let mut all = coords
        .iter()
        .filter_map(|b| {
            if part1_data.0 == *b {
                None
            } else {
                Some((sqdistance(part1_data.0, *b), angleish(part1_data.0, *b), *b))
            }
        })
        .collect::<Vec<_>>();

    all.sort_by(|(d1, a1, _), (d2, a2, _)| a1.cmp(a2).then(d1.partial_cmp(d2).unwrap()));
    let twohdth = all.iter().nth(200).unwrap().2;

    (twohdth.0 * 100.0 + twohdth.1) as i32
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {:?}", part1(input).1);
    println!("Part 2: {:?}", part2(input));
}
