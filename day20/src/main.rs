use pathfinding::directed::dijkstra::dijkstra;
use pathfinding::grid::Grid;
use pathfinding::matrix::Matrix;

use std::collections::HashMap;

#[derive(Debug)]
struct Portal {
    locations: Vec<(usize, usize)>,
}

impl Portal {
    fn new() -> Self {
        Portal {
            locations: vec![]
        }
    }
}

fn part1(input: &str) -> usize {
    let width = input.find('\n').unwrap() - 1;
    let height = input.trim().lines().count();

    let decon = input.chars().filter(|&c| c != '\n' && c != '\r').collect();
    let matrix = Matrix::from_vec(height, width, decon).unwrap();
    let transposed = matrix.transposed();

    let mut portals = HashMap::new();

    for y in 0..height {
        for x in 0..width - 2 {
            let comb = (matrix[&(y, x)], matrix[&(y, x+1)], matrix[&(y, x+2)]);
            match comb {
                ('A'..='Z', 'A'..='Z', '.') => {
                    let entry = portals.entry(format!("{}{}", comb.0, comb.1)).or_insert(Portal::new());
                    entry.locations.push((x+2, y));
                }
                ('.', 'A'..='Z', 'A'..='Z') => {
                    let entry = portals.entry(format!("{}{}", comb.1, comb.2)).or_insert(Portal::new());
                    entry.locations.push((x, y));
                }
                _ => (),
            }
        }
    }

    for x in 0..width {
        for y in 0..height - 2 {
            let comb = (transposed[&(x, y)], transposed[&(x, y+1)], transposed[&(x, y+2)]);
            match comb {
                ('A'..='Z', 'A'..='Z', '.') => {
                    let entry = portals.entry(format!("{}{}", comb.0, comb.1)).or_insert(Portal::new());
                    entry.locations.push((x, y+2))
                }
                ('.', 'A'..='Z', 'A'..='Z') => {
                    let entry = portals.entry(format!("{}{}", comb.1, comb.2)).or_insert(Portal::new());
                    entry.locations.push((x, y))
                }
                _ => (),
            }
        }
    }

    let mut neighbours_extra = HashMap::new();

    for p in portals.values() {
        if p.locations.len() > 1 {
            neighbours_extra.insert(p.locations[0], p.locations[1]);
            neighbours_extra.insert(p.locations[1], p.locations[0]);
        }
    }

    let mut grid = Grid::new(width, height);

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, c)| match c {
            '.' => { grid.add_vertex((x, y)); },
            '#' | ' ' => (),
            'A'..='Z' => (),
            _ => panic!("Unhandled character"),
        })
    });

    let start = portals["AA"].locations[0];
    let goal = portals["ZZ"].locations[0];

    let path = dijkstra(&start, |p| {
        let mut neighbours: Vec<_> = grid.neighbours(p).iter().map(|&n| (n, 1)).collect();
        if let Some(x) = neighbours_extra.get(p) {
            neighbours.push((*x, 1));
        }
        neighbours
    }, |&p| p == goal);

    path.unwrap().1
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
}
