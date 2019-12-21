use intcode::{parse_program, Computer, Program};
use std::convert::TryFrom;

fn part1(base: Program) {
    let mut c = Computer::new(base);

    let command = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";
    command.bytes().for_each(|b| c.push(b as i64));

    while let Some(out) = c.run() {
        if let Ok(c) = u8::try_from(out) {
            print!("{}", c as char);
        } else {
            println!("Damage: {}", out);
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    part1(base);
}
