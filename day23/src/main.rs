use intcode::{parse_program, Computer, Program};

use std::collections::HashSet;

fn part1(base: Program) -> i64 {
    let mut computers = vec![];
    for i in 0..50 {
        let mut c = Computer::new(base.clone());
        c.push(i);
        computers.push(c);
    }

    let result;

    'outer: loop {
        for i in 0..computers.len() {
            while !computers[i].done {
                if computers[i].empty() {
                    computers[i].push(-1);
                }
                if let Some(address) = computers[i].run() {
                    if let Some(x) = computers[i].run() {
                        if let Some(y) = computers[i].run() {
                            if address == 255 {
                                result = y;
                                break 'outer;
                            }

                            computers[address as usize].push(x);
                            computers[address as usize].push(y);
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    result
}

fn part2(base: Program) -> i64 {
    let mut computers = vec![];
    for i in 0..50 {
        let mut c = Computer::new(base.clone());
        c.push(i);
        computers.push(c);
    }

    let mut nat = (0, 0);
    let mut nats = HashSet::new();

    loop {
        if computers.iter().all(|c| c.empty()) {
            if !nats.insert(nat) {
                break;
            };
            computers[0].push(nat.0);
            computers[0].push(nat.1);
        }
        for i in 0..computers.len() {
            while !computers[i].done {
                if computers[i].empty() {
                    computers[i].push(-1);
                }
                if let Some(address) = computers[i].run() {
                    if let Some(x) = computers[i].run() {
                        if let Some(y) = computers[i].run() {
                            if address == 255 {
                                nat = (x, y);
                            } else {
                                computers[address as usize].push(x);
                                computers[address as usize].push(y);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    nat.1
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    println!("Part 1: {}", part1(base.clone()));
    println!("Part 2: {}", part2(base));
}
