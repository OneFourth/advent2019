fn fuel_required(m: i32) -> i32 {
    (m / 3) - 2
}

fn extra_fuel_required(m: i32) -> i32 {
    let f = fuel_required(m);
    match f {
        f if f <= 0 => 0,
        _ => f + extra_fuel_required(f),
    }
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

fn part1(input: &str) -> i32 {
    parse(input).into_iter().map(fuel_required).sum()
}

fn part2(input: &str) -> i32 {
    parse(input).into_iter().map(extra_fuel_required).sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("12"), 2);
        assert_eq!(part1("14"), 2);
        assert_eq!(part1("1969"), 654);
        assert_eq!(part1("100756"), 33583);

        assert_eq!(part1(include_str!("../input")), 3305115);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("14"), 2);
        assert_eq!(part2("1969"), 966);
        assert_eq!(part2("100756"), 50346);

        assert_eq!(part2(include_str!("../input")), 4954799);
    }
}
