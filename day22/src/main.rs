use std::convert::TryInto;
use regex::Regex;

fn stack(cards: &[usize]) -> Vec<usize> {
    cards.iter().copied().rev().collect()
}

fn cut(cards: &[usize], n: isize) -> Vec<usize> {
    let m: isize;
    if n > 0 {
       m = n;
     }
     else {
        m = cards.len() as isize + n;
     }
     let real_n: usize = m.try_into().unwrap();
     cards[real_n..].iter().copied().chain(cards[..real_n].iter().copied()).collect()
}

fn inc(cards: &[usize], n: usize) -> Vec<usize> {
    let mut result = vec![0; cards.len()];
    for (i, &v) in cards.iter().enumerate() {
        result[(i * n) % cards.len()] = v;
    }
    result
}

#[derive(Debug)]
enum Shuffle {
    Cut(isize),
    Inc(usize),
    Stack,
}

fn part1(commands: Vec<Shuffle>) -> usize {
    let mut cards: Vec<_> = (0..10007).collect();

    for n in commands {
        use Shuffle::*;
        cards = match n {
            Cut(c) => cut(&cards, c),
            Inc(c) => inc(&cards, c),
            Stack => stack(&cards),
        }
    }

    cards.iter().position(|&v| v == 2019).unwrap()
}

fn main() {
    let input = include_str!("../input");
    let reg = Regex::new(r"cut (?P<cut>-?\d+)|deal with increment (?P<inc>\d+)|deal into new stack(?P<stack>)").unwrap();
    let commands: Vec<_> = input.lines().map(|s| {
        use Shuffle::*;
        let caps = reg.captures(s.trim()).unwrap();
        if let Some(c) = caps.name("cut") {
            Cut(c.as_str().parse().unwrap())
        }
        else if let Some(c) = caps.name("inc") {
            Inc(c.as_str().parse().unwrap())
        }
        else {
            Stack
        }
    }).collect();

    println!("Part 1: {:?}", part1(commands));
}
