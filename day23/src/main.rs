use intcode::{parse_program, Computer, Program};

fn part1(base: Program) {
    let mut computers = vec![];
    for i in 0..50 {
        let mut c = Computer::new(base.clone());
        c.push(i);
        c.set_default_input(Some(-1));
        computers.push(c);
    }

    loop {
        for i in 0..computers.len() {
            if let Some(address) = computers[i].run() {
                if let Some(x) = computers[i].run() {
                    if let Some(y) = computers[i].run() {
                        println!("Sending X={}, Y={} to address: {}", x, y, address);

                        computers[address as usize].push(x);
                        computers[address as usize].push(y);
                    }
                }
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    part1(base);
}
