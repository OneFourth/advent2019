use intcode::{parse_program, Computer, Program};

use std::collections::HashMap;

fn part1(program: Program) -> usize {
    let mut grid = HashMap::new();

    for y in 0..=49 {
        for x in 0..=49 {
            let mut computer = Computer::new(program.clone());

            computer.push(x);
            computer.push(y);
            if let Some(c) = computer.run() {
                grid.insert((x, y), c);
            }
        }
    }

    grid.iter().filter(|&(_, v)| *v == 1).count()
}

fn main() {
    let input = include_str!("../input");
    let program = parse_program(input);

    println!("{}", part1(program.clone()));
}
