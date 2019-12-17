use intcode::{parse_program, Computer, Program};

fn part1(base: Program) -> i64 {
    let mut c = Computer::new(base);

    c.push(1);

    let mut outputs = vec![];
    while let Some(out) = c.run() {
        outputs.push(out);
    }
    if outputs.iter().rev().skip(1).any(|v| *v != 0) {
        println!("{:?}", outputs);
        panic!("Diagnostic failed")
    }

    *outputs.last().unwrap()
}

fn part2(base: Program) -> i64 {
    let mut c = Computer::new(base);

    c.push(5);

    c.run().unwrap()
}

fn main() {
    let input = include_str!("../input");

    let base = parse_program(input);

    println!("Part 1: {}", part1(base.clone()));
    println!("Part 2: {}", part2(base.clone()));
}
