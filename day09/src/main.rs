use intcode::{parse_program, Computer, Program};

fn part1(base: Program) -> i64 {
    let mut c = Computer::new(base);
    c.push(1);
    c.run().unwrap()
}

fn part2(base: Program) -> i64 {
    let mut c = Computer::new(base);
    c.push(2);
    c.run().unwrap()
}

fn main() {
    let input = include_str!("../input");

    let base = parse_program(input);

    println!("{}", part1(base.clone()));
    println!("{}", part2(base));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let program = parse_program("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut c = Computer::new(program);

        let mut results = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        results.reverse();
        while let Some(v) = c.run() {
            let r = results.pop();
            assert!(r != None);
            assert!(v == r.unwrap());
        }
    }
}
