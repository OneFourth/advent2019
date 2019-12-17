use std::collections::HashMap;
use intcode::{parse_program, Computer, Program};

fn turn_to(dir: i64, turn: i64) -> i64 {
    if turn == 0 {
        (dir + 3) % 4
    } else {
        (dir + 1) % 4
    }
}

fn part1(base: Program) -> usize {
    let mut c = Computer::new(base);

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut painted = HashMap::new();
    let mut position = (0, 0);
    let mut direction = 0;

    let mut count = 0;

    while !c.done {
        let entry = painted.entry(position).or_insert_with(|| {
            count += 1;
            0
        });
        c.push(*entry);
        if let Some(colour) = c.run() {
            if let Some(turn) = c.run() {
                *entry = colour;
                direction = turn_to(direction, turn);
                let move_dir = directions[direction as usize];
                position = (position.0 + move_dir.0, position.1 + move_dir.1);
            };
        }
    }

    count
}

fn part2(base: Program) {
    let mut c = Computer::new(base);

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut painted = HashMap::new();
    let mut position = (0, 0);
    painted.insert(position, 1);
    let mut direction = 0;

    while !c.done {
        let entry = painted.entry(position).or_insert(0);
        c.push(*entry);
        if let Some(colour) = c.run() {
            if let Some(turn) = c.run() {
                *entry = colour;
                direction = turn_to(direction, turn);
                let move_dir = directions[direction as usize];
                position = (position.0 + move_dir.0, position.1 + move_dir.1);
            };
        }
    }

    let x_min = painted.keys().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = painted.keys().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = painted.keys().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = painted.keys().max_by_key(|(_, y)| y).unwrap().1;

    println!("Part 2:");
    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            let entry = painted.entry((x, y)).or_insert(0);
            print!(
                "{}",
                match *entry {
                    0 => "  ",
                    1 => "##",
                    _ => "  ",
                }
            );
        }
        println!();
    }
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input); 

    println!("Part 1: {}", part1(base.clone()));
    part2(base.clone());
}
