fn is_valid(s: usize) -> bool {
    let val: Vec<_> = s.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    (val[0] <= val[1]
        && val[1] <= val[2]
        && val[2] <= val[3]
        && val[3] <= val[4]
        && val[4] <= val[5])
        && (val[0] == val[1]
            || val[1] == val[2]
            || val[2] == val[3]
            || val[3] == val[4]
            || val[4] == val[5])
}

fn main() {
    let input = include_str!("../input");
    let split: Vec<_> = input
        .split("-")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect();
    let (low, high) = (split[0], split[1]);

    println!("Part 1: {}", (low..=high).filter(|&n| is_valid(n)).count());
}
