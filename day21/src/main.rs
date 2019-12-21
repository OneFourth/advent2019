use intcode::{parse_program, Computer, Program};

fn part1(base: Program) {
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    println!("{:?}", part1(base));
}
