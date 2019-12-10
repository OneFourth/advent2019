use std::collections::HashSet;

fn main() {
    let input = include_str!("../input");

    let coords: Vec<_> = input
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
        .collect();

    let angleish = |(ax, ay): (f64, f64), (bx, by): (f64, f64)| {
        ((ay - by).atan2(ax - bx).to_degrees() * 1000.0).trunc() as i32
    };

    let angles = coords.iter().map(|a| {
            coords
                .iter()
                .filter_map(move |b| if a == b { None } else { Some(angleish(*a, *b)) })
                .collect::<HashSet<_>>()
                .len()
    }).collect::<Vec<_>>();

    println!("{:?}", angles.iter().max());
}
