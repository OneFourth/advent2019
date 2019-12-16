use std::iter;

fn get_pattern(pos: usize, size: usize) -> Vec<isize> {
    const BASE: [isize; 4] = [0, 1, 0, -1];

    BASE.iter()
        .cycle()
        .flat_map(|&v| iter::repeat(v).take(pos + 1))
        .skip(1)
        .take(size)
        .collect()
}

fn phase(numbers: Vec<isize>) -> Vec<isize> {
    let mut vals = vec![];
    for i in 0..=numbers.len() {
        let pat = get_pattern(i, numbers.len());
        vals.push(numbers.iter().zip(pat.iter()).map(|(n, p)| n * p).sum::<isize>().abs() % 10);
    }
    vals
}

fn part1(numbers: &Vec<isize>) -> Vec<isize> {
    let mut current = numbers.clone();
    for _ in 0..100 {
        current = phase(current);
    }
    current.into_iter().take(8).collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("../input");
    let numbers: Vec<_> = input
        .trim()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as isize)
        .collect();

    println!("{:?}", part1(&numbers));
}
