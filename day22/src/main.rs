use regex::Regex;
use std::convert::TryInto;

#[derive(Debug)]
enum Shuffle {
    Cut(isize),
    Inc(usize),
    Stack,
}

fn part1(commands: &[Shuffle]) -> isize {
    const LEN: isize = 10007;
    let mut val = 2019;

    use Shuffle::*;
    for command in commands.iter().cycle().take(commands.len()) {
        val = match command {
            Cut(n) => (LEN + val - n) % LEN,
            Inc(n) => (val * *n as isize) % LEN,
            Stack => LEN - 1 - val,
        }
    }

    val
}

fn part2(commands: &[Shuffle]) -> isize {
    const LEN: isize = 10007;
    let mut val = 2019;

    use Shuffle::*;
    for command in commands.iter().cycle().take(commands.len()) {
        val = match command {
            Cut(n) => (LEN + val - n) % LEN,
            Inc(n) => (val * *n as isize) % LEN,
            Stack => LEN - 1 - val,
        }
    }

    val
}

fn main() {
    let input = include_str!("../input");
    let reg = Regex::new(
        r"cut (?P<cut>-?\d+)|deal with increment (?P<inc>\d+)|deal into new stack(?P<stack>)",
    )
    .unwrap();
    let commands: Vec<_> = input
        .lines()
        .map(|s| {
            use Shuffle::*;
            let caps = reg.captures(s.trim()).unwrap();
            if let Some(c) = caps.name("cut") {
                Cut(c.as_str().parse().unwrap())
            } else if let Some(c) = caps.name("inc") {
                Inc(c.as_str().parse().unwrap())
            } else {
                Stack
            }
        })
        .collect();

    println!("Part 1: {:?}", part1(&commands));
    println!("Part 2: {:?}", part2(&commands));
}
