use std::collections::HashMap;
use std::convert::TryFrom;

use intcode::{parse_program, Computer, Program};

type MapData = HashMap<(usize, usize), char>;

fn get_map(base: Program) -> (Computer, MapData, (usize, usize)) {
    let mut c = Computer::new(base);

    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut robot_pos = pos;

    while let Some(out) = c.run() {
        let c = u8::try_from(out).unwrap() as char;
        if out != 10 {
            map.insert(pos, c);
            if c == '^' {
                robot_pos = pos;
            }
            pos.0 += 1;
        } else {
            pos.1 += 1;
            pos.0 = 0;
        }

        print!("{}", c);
    }

    (c, map, robot_pos)
}

fn part1(base: Program) -> usize {
    let (_, map, _) = get_map(base);

    let x_min = map.keys().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = map.keys().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = map.keys().max_by_key(|(_, y)| y).unwrap().1;

    let mut count = 0;

    for y in y_min + 1..y_max {
        for x in x_min + 1..x_max {
            if map.get(&(x, y)) == Some(&'#') {
                let adj = [
                    map.get(&(x - 1, y)),
                    map.get(&(x + 1, y)),
                    map.get(&(x, y - 1)),
                    map.get(&(x, y + 1)),
                ];

                if adj.iter().all(|v| *v == Some(&'#')) {
                    count += x as usize * y as usize;
                }
            }
        }
    }

    count
}

fn turn((x, y): (i32, i32), turn_left: bool) -> (i32, i32) {
    if turn_left {
        match (x, y) {
            (-1, 0) => (0, 1),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (0, 1) => (1, 0),
            _ => panic!(),
        }
    } else {
        match (x, y) {
            (-1, 0) => (0, -1),
            (1, 0) => (0, 1),
            (0, -1) => (1, 0),
            (0, 1) => (-1, 0),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Move {
    TurnRight,
    TurnLeft,
    Move(usize),
}

fn part2(base: Program) -> i64 {
    use crate::Move::*;

    let (_, map, (mut x, mut y)) = get_map(base.clone());
    let mut movements = vec![];

    let mut curr_dir = (0, -1);

    let add_dir = |(x, y), (dx, dy)| ((x as i32 + dx) as usize, (y as i32 + dy) as usize);

    loop {
        let right = turn(curr_dir, false);
        let left = turn(curr_dir, true);
        let (new_x, new_y) = add_dir((x, y), curr_dir);
        if map.get(&(new_x, new_y)) == Some(&'#') {
            x = new_x;
            y = new_y;
            let last = movements.last_mut();
            match last {
                Some(Move(v)) => *v += 1,
                _ => movements.push(Move(1)),
            }
        } else if map.get(&add_dir((x, y), right)) == Some(&'#') {
            curr_dir = right;
            movements.push(TurnRight);
        } else if map.get(&add_dir((x, y), left)) == Some(&'#') {
            curr_dir = left;
            movements.push(TurnLeft);
        } else {
            break;
        }
    }

    println!("{:?}", movements);

    let mut c = Computer::new(base);

    c.data[0].set(2);
    let all = "A,B,A,C,B,C,B,C,A,C\nR,12,L,10,R,12\nL,8,R,10,R,6\nR,12,L,10,R,10,L,8\nn\n";
    for b in all.bytes() {
        print!("{} ", b);
        c.push(b as i64);
    }
    println!();

    while let Some(v) = c.run() {
        if let Ok(c) = u8::try_from(v) {
            print!("{}", c as char);
        } else {
            return v;
        }
    }
    0
}

fn main() {
    let input = include_str!("../input");

    let base = parse_program(input);

    println!("Part 1: {}", part1(base.clone()));
    println!("Part 2: {}", part2(base));
}
