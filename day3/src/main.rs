use std::collections::HashSet;
use std::iter::FromIterator;

type Point = (i32, i32);

type Wire = Vec<Point>;

fn build_wire(input_str: &str) -> Wire {
    let mut wire: Vec<Point> = vec![];

    let mut x = 0;
    let mut y = 0;

    for dir in input_str.trim().split(',') {
        let (head, tail) = dir.split_at(1);
        let count = tail.parse::<i32>().unwrap();
        let (a_x, a_y) = match head {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => (0, 0),
        };

        for _ in 1..=count {
            x += a_x;
            y += a_y;

            wire.push((x, y));
        }
    }
    wire
}

/*
fn output(wires: Vec<Wire>) {
    let mut out: Vec<Vec<char>> = vec![vec!['.'; 1000]; 1000];
    out[300][300] = 'o';

    let mut count = 1;
    for w in wires {
        for (x, y) in w {
            out[(y + 300) as usize][(x + 300) as usize] = std::char::from_digit(count, 10).unwrap();
        }
        count += 1;
    }

    for s in out {
        println!("{:?}", s.into_iter().collect::<String>());
    }
}
*/

fn main() {
    let input = include_str!("../input");

    let wires: Vec<_> = input.lines().map(|s| build_wire(s)).collect();
    let sets: Vec<HashSet<_>> = wires.iter().map(HashSet::from_iter).collect();
    let intersections: Vec<_> = sets[0].intersection(&sets[1]).collect();
    let distances: Vec<_> = intersections
        .iter()
        .map(|(x, y)| x.abs() + y.abs())
        .collect();

    println!("Part 1: {}", distances.iter().min().unwrap());

    let min: usize = intersections
        .iter()
        .map(|&&i| {
            wires
                .iter()
                .map(|w| w.iter().position(|p| p == i).unwrap() + 1)
                .sum()
        })
        .min()
        .unwrap();
    println!("Part 2: {}", min);
}
