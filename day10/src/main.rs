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

    let all = coords
        .iter()
        .filter_map(|b| {
            if part1_data.0 == *b {
                None
            } else {
                Some((angleish(part1_data.0, *b), sqdistance(part1_data.0, *b), *b))
            }
        })
        .collect::<Vec<_>>();

    let unique_angles: HashSet<_> = all.iter().map(|(a, _, _)| a).collect();

    let mut vec_of_vec: Vec<_> = unique_angles
        .iter()
        .map(|&a| {
            let mut vals = all
                .iter()
                .filter(|(ang, _, _)| a == ang)
                .collect::<Vec<_>>();
            vals.sort_by(|(_, d1, _), (_, d2, _)| d1.partial_cmp(d2).unwrap());
            (a, vals)
        })
        .collect();
    vec_of_vec.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));

    let twohdth = (vec_of_vec.get(199).unwrap().1)[0].2;
    (twohdth.0 * 100.0 + twohdth.1) as i32
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {:?}", part1(input).1);
    println!("Part 2: {:?}", part2(input));
}
