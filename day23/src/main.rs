use intcode::{parse_program, Computer, Program};

fn part1(base: Program) -> i64 {
    let mut computers = vec![];
    for i in 0..50 {
        let mut c = Computer::new(base.clone());
        c.push(i);
        computers.push(c);
    }

    let result;

    'outer: loop {
        for i in 0..computers.len() {
            while !computers[i].done {
                if computers[i].empty() {
                    computers[i].push(-1);
                }
                if let Some(address) = computers[i].run() {
                    if let Some(x) = computers[i].run() {
                        if let Some(y) = computers[i].run() {
                            if address == 255 {
                                result = y;
                                break 'outer;
                            }

                            computers[address as usize].push(x);
                            computers[address as usize].push(y);
                        }
                    }
                }
                else {
                    break;
                }
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    println!("{}", part1(base));
}
