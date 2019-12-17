use crossterm::{cursor, terminal, ExecutableCommand};
use std::convert::TryFrom;
use std::io::stdout;

use std::collections::HashMap;
use std::{thread, time};

use intcode::{parse_program, Computer, Program};

fn part1(base: Program) -> usize {
    let mut c = Computer::new(base);
    let mut map = HashMap::new();

    while !c.done {
        let x = c.run();
        let y = c.run();
        let id = c.run();

        if let (Some(x_), Some(y_), Some(id_)) = (x, y, id) {
            *map.entry((x_, y_)).or_insert(id_) = id_;
        }
    }

    map.iter().filter(|(_, v)| **v == 2).count()
}

fn part2(base: Program, part1_result: usize) {
    base[0].set(2);
    let mut c = Computer::new(base);

    let mut input = 0;
    let mut px = 0;
    let mut bx = 0;

    let mut stdout = stdout();

    let mut fast_it = 900;

    stdout.execute(cursor::Hide).ok();
    stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();
    print!(r"Part1: {}
 _______  _______  ______    _______    _______  ___  
|       ||   _   ||    _ |  |       |  |       ||   | 
|    _  ||  |_|  ||   | ||  |_     _|  |____   ||___| 
|   |_| ||       ||   |_||_   |   |     ____|  | ___  
|    ___||       ||    __  |  |   |    | ______||   | 
|   |    |   _   ||   |  | |  |   |    | |_____ |___| 
|___|    |__| |__||___|  |_|  |___|    |_______|     ", part1_result);

    while !c.done {
        c.clear_input();
        c.push(input);
        let x = c.run();
        let y = c.run();
        let id = c.run();

        if let (Some(x_), Some(y_), Some(id_)) = (x, y, id) {
            if x_ == -1 && y_ == 0 {
                stdout.execute(cursor::MoveTo(0, 31)).ok();
                print!("Score: {}", id_);
            } else {
                stdout.execute(cursor::MoveTo(
                    u16::try_from(x_).unwrap() * 2,
                    u16::try_from(y_).unwrap() + 10,
                )).ok();
            }

            if id_ == 3 {
                px = x_;
            } else if id_ == 4 {
                bx = x_;
            }

            print!(
                "{}",
                match id_ {
                    1 => "██",
                    2 => "[]",
                    3 => "▀▀",
                    4 => "()",
                    _ => "  ",
                }
            );
        }

        if px > bx {
            input = -1;
        } else if px < bx {
            input = 1;
        } else {
            input = 0;
        }

        if fast_it > 0 {
            fast_it -= 1;
        } else {
            thread::sleep(time::Duration::from_millis(16));
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let base = parse_program(input);

    let part1_result = part1(base.clone());

    part2(base.clone(), part1_result);
}
