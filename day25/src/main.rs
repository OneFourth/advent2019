use intcode::{parse_program, Computer, Program};
use std::convert::TryFrom;
use std::io::{self, BufRead};
use itertools::Itertools;

fn part1(base: Program) {
    let mut inputs = vec!["south",
    "take fixed point",
    "north",
    "north",
    "take spool of cat6",
    "north",
    "take monolith",
    "north",
    "take hypercube",
    "south",
    "west",
    "take planetoid",
    "east",
    "south",
    "east",
    "north",
    "take candy cane",
    "south",
    "east",
    "take easter egg",
    "east",
    "south",
    "take ornament",
    "west",
    "south"];

    let items = ["planetoid",
        "candy cane",
        "spool of cat6",
        "ornament",
        "easter egg",
        "fixed point",
        "hypercube",
        "monolith"];
    let mut combos = (1..=items.len()).flat_map(|l| items.iter().combinations(l));

    let mut c = Computer::new(base.clone());

    for i in inputs {
        i.bytes().for_each(|b| c.push(b as i64));
        c.push('\n' as i64);
    }

    loop {
        match c.run() {
            Some(out) => {
                if let Ok(s) = u8::try_from(out) {
                    print!("{}", s as char);
                }
            }
            None => {
                if !c.done {
                    if let Some(combo) = combos.next() {
                        items.iter().for_each(|i| {
                            let line = format!("drop {}", i);
                            line.bytes().for_each(|b| c.push(b as i64));
                            c.push('\n' as i64);
                        });

                        combo.iter().for_each(|i| {
                            let line = format!("take {}", i);
                            line.bytes().for_each(|b| c.push(b as i64));
                            c.push('\n' as i64);
                        });

                        let line = "inv\nwest";
                        line.bytes().for_each(|b| c.push(b as i64));
                        c.push('\n' as i64);
                    }
                    else {
                        break;
                    }
                }
                else {
                    break;
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    part1(base);
}
