fn fuel_required(m: i32) -> i32 {
    (m / 3) - 2
}

fn extra_fuel_required(m: i32) -> i32 {
    println!("{}", m);
    let f = fuel_required(m);
    match f {
        f if f <= 0 => 0,
        _ => f + extra_fuel_required(f),
    }
}

fn main() {
    let input = include_str!("../input");

    let part1 = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(fuel_required)
        .sum::<i32>();

    println!("Day 1, Part 1: {:#?}", part1);

    let part2 = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .map(extra_fuel_required)
        .sum::<i32>();

    println!("Day 1, Part 2: {:#?}", part2);
}
