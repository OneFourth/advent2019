fn main()
{
    let input = include_str!("../input");

    let day1 = input.lines()
                    .map(|l| l.parse::<i32>().unwrap())
                    .map(|n| (n/3) - 2)
                    .sum::<i32>();

    println!("Day 1, Part 1: {:#?}", day1);
}
