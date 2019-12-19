use intcode::{parse_program, Computer, Program}; 
use std::collections::HashMap;

fn debug_printing(program: Program) {
    let mut grid = HashMap::new();

    for y in 800..=1200 {
        for x in 913..=1020 {
            let entry = grid.entry((x, y)).or_insert_with(|| {
                let mut computer = Computer::new(program.clone());
                

                computer.push(x);
                computer.push(y);
                computer.run().unwrap()
            });

            if (x, y) == (914, 1018) {
                print!("{}", '+');
            }
            else {
                print!("{}", match *entry {
                    1 => '#',
                    _ => '.',
                });
            }
        }
        println!();
    }
}


fn part1(program: Program) -> usize {
    let mut grid = HashMap::new();

    for y in 0..=49 {
        for x in 0..=49 {
            let mut computer = Computer::new(program.clone());

            computer.push(x);
            computer.push(y);
            if let Some(c) = computer.run() {
                grid.insert((x, y), c);
            }
        }
    }

    grid.iter().filter(|&(_, v)| *v == 1).count()
}

fn part2 (program: Program) -> i64 {
    let mut grid = HashMap::new();

    let mut result = (0, 0);

    for y in 0..=10000 {
        let mut found = false;
        let mut countx = 0;
        let mut endx = None;

        for x in 0..=10000 {
            let entry = grid.entry((x, y)).or_insert_with(|| {
                let mut computer = Computer::new(program.clone());

                computer.push(x);
                computer.push(y);
                computer.run().unwrap()
            });

            if *entry == 1 {
                if !found {
                    found = true;
                }
                countx += 1;
            }
            else {
                if found {
                    endx = Some(x-1);
                    break;
                }
            }
        }

        if countx > 100 {
            if let Some(mut x_) = endx {
                x_ -= 99;
                let mut county = 1;

                for y_ in y+1..=y+100 {
                    let entry = grid.entry((x_, y_)).or_insert_with(|| {
                        let mut computer = Computer::new(program.clone());

                        computer.push(x_);
                        computer.push(y_);
                        computer.run().unwrap()
                    });

                    if *entry == 1 {
                        county += 1;
                    }
                    else {
                        break;
                    }
                }

                if county >= 100 {
                    result = (x_, y);
                    break;
                }
            }
        }
    }

    result.0 * 10000 + result.1
}

fn main() {
    let input = include_str!("../input");
    let program = parse_program(input);

    //println!("{}", part1(program.clone()));
    println!("{}", part2(program.clone()));
    //debug_printing(program.clone());
}
