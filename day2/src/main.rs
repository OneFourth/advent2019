use intcode::{parse_program, Computer, Program};

fn part1(base: Program) -> i64 {
    base[1].set(12);
    base[2].set(2);

    let mut computer = Computer::new(base.clone());
    computer.run();

    computer.data[0].get()
}

fn part2(base: Program) -> Option<i64> {
    const TARGET: i64 = 19_690_720;

    for verb in 0..99 {
        for noun in 0..99 {
            let clone = base.clone();
            clone[1].set(noun);
            clone[2].set(verb);

            let mut c = Computer::new(clone);
            c.run();
            if c.data[0].get() == TARGET {
                return Some(100 * noun + verb);
            }
        }
    }

    None
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    println!("Part 1: {}", part1(base.clone()));
    println!("Part 2: {}", part2(base.clone()).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let tests = [
            ("1,9,10,3,2,3,11,0,99,30,40,50", 3500),
            ("1,0,0,0,99", 2),
            ("2,0,0,0,99", 4),
            ("1,1,1,4,99,5,6,0,99", 30),
        ];
        for (s, result) in tests.iter() {
            let program = parse_program(s);
            let mut c = Computer::new(program);
            c.run();
            assert!(c.data[0].get() == *result);
        }
    }

    #[test]
    fn input_test() {
        let program = parse_program(include_str!("../input"));
        assert!(part1(program) == 3166704);
    }
}
