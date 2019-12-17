use crossterm::{cursor, terminal, ExecutableCommand};
use std::cell::Cell;
use std::collections::HashMap;
use std::io::{stdout, Read};
use std::{thread, time};
use std::convert::TryFrom;

#[derive(Debug)]
struct Thruster {
    pointer: usize,
    done: bool,
    relative: i64,
    input: Vec<i64>,
    program: Vec<Cell<i64>>,
}

impl Thruster {
    fn access_program(&mut self, a: usize) -> &Cell<i64> {
        if self.program.len() <= a {
            self.program.resize(a + 1, Cell::new(0));
        }
        &self.program[a]
    }

    fn get_cell(&mut self, a: usize) -> &Cell<i64> {
        let address = self.access_program(self.pointer + a).get() as usize;
        &self.access_program(address)
    }

    fn get_parm(&mut self, pos: usize, digits: &[char]) -> &Cell<i64> {
        if digits.len() > pos + 1 {
            match digits[pos + 1] {
                '0' => self.get_cell(pos),
                '1' => self.access_program(self.pointer + pos), // value
                '2' => {
                    let address =
                        (self.relative + self.access_program(self.pointer + pos).get()) as usize;
                    self.access_program(address)
                } // relative
                _ => panic!("help get_parm"),
            }
        } else {
            self.get_cell(pos) // address
        }
    }

    fn run_program(&mut self) -> Option<i64> {
        let mut result = None;
        while !(self.done || result != None) {
            let mut digits = self
                .access_program(self.pointer)
                .get()
                .to_string()
                .chars()
                .rev()
                .collect::<Vec<_>>();

            let before_op = self.pointer;
            if digits.len() == 1 {
                digits.resize(2, '0');
            }
            match (digits[1], digits[0]) {
                ('0', '1') => {
                    let parm1 = self.get_parm(1, &digits).get();
                    let parm2 = self.get_parm(2, &digits).get();
                    self.get_parm(3, &digits).set(parm1 + parm2);
                }
                ('0', '2') => {
                    let parm1 = self.get_parm(1, &digits).get();
                    let parm2 = self.get_parm(2, &digits).get();
                    self.get_parm(3, &digits).set(parm1 * parm2);
                }
                ('0', '3') => {
                    let i = self.input.pop();
                    self.get_parm(1, &digits).set(i.unwrap());
                }
                ('0', '4') => result = Some(self.get_parm(1, &digits).get()),
                ('0', '5') => {
                    if self.get_parm(1, &digits).get() != 0 {
                        self.pointer = self.get_parm(2, &digits).get() as usize
                    }
                }
                ('0', '6') => {
                    if self.get_parm(1, &digits).get() == 0 {
                        self.pointer = self.get_parm(2, &digits).get() as usize
                    }
                }
                ('0', '7') => {
                    let parm1 = self.get_parm(1, &digits).get();
                    let parm2 = self.get_parm(2, &digits).get();
                    self.get_parm(3, &digits).set((parm1 < parm2) as i64);
                }
                ('0', '8') => {
                    let parm1 = self.get_parm(1, &digits).get();
                    let parm2 = self.get_parm(2, &digits).get();
                    self.get_parm(3, &digits).set((parm1 == parm2) as i64);
                }
                ('0', '9') => self.relative += self.get_parm(1, &digits).get(),
                ('9', '9') => self.done = true,
                _ => panic!("help"),
            };

            if self.pointer == before_op {
                self.pointer += match digits[0] {
                    '3' | '4' | '9' => 2,
                    '5' | '6' => 3,
                    '1' | '2' | '7' | '8' => 4,
                    _ => 1,
                };
            }
        }
        result
    }
}

fn get_map(base: &[Cell<i64>]) -> (Thruster, HashMap<(usize, usize), char>, (usize, usize)) {
    let mut t = Thruster {
        pointer: 0,
        done: false,
        relative: 0,
        input: vec![],
        program: base.to_owned(),
    };

    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut robot_pos = pos;

    while let Some(out) = t.run_program() {
        let c = u8::try_from(out).unwrap() as char;
        if out != 10 {
            map.insert(pos, c);
            if c == '^' {
                robot_pos = pos;
            }
            pos.0 += 1;

        }
        else {
            pos.1 += 1;
            pos.0 = 0;
        }

        print!("{}", c);
    }

    (t, map, robot_pos)
}

fn part1(base: &[Cell<i64>]) -> usize {
    let (_, map, _) = get_map(base);

    let x_min = map.keys().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = map.keys().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = map.keys().max_by_key(|(_, y)| y).unwrap().1;

    let mut count = 0;

    for y in y_min+1..y_max {
        for x in x_min+1..x_max {
            if map.get(&(x, y)) == Some(&'#') {
                let adj = [
                    map.get(&(x - 1, y)),
                    map.get(&(x + 1, y)),
                    map.get(&(x, y - 1)),
                    map.get(&(x, y + 1)),
                ];

                if adj.into_iter().all(|v| *v == Some(&'#')) {
                    count += x as usize * y as usize;
                }

            }
        }
    }

    count
}

fn turn((x, y): (i32, i32), turnLeft: bool) -> (i32, i32) {
    if turnLeft {
        match (x, y) {
            (-1, 0) => (0, 1),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (0, 1) => (1, 0),
            _ => panic!(),
        }
    }
    else {
        match (x, y) {
            (-1, 0) => (0, -1),
            (1, 0) => (0, 1),
            (0, -1) => (1, 0),
            (0, 1) => (-1, 0),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
enum Move {
    TurnRight,
    TurnLeft,
    Move(usize),
}

fn part2(base: &[Cell<i64>]) -> i64 {
    use crate::Move::*;

    let (_, map, (mut x, mut y)) = get_map(base);
    let mut movements = vec![];

    let mut currDir = (0, -1);

    let add_dir = |(x, y), (dx, dy)| {
        ((x as i32 + dx) as usize, (y as i32 + dy) as usize)
    };

    loop {
        let right = turn(currDir, false);
        let left = turn(currDir, true);
        let (new_x, new_y) = add_dir((x, y), currDir);
        if map.get(&(new_x, new_y)) == Some(&'#') {
            x = new_x;
            y = new_y;
            let mut last = movements.last_mut();
            match last {
                Some(Move(v)) => *v += 1,
                _ => movements.push(Move(1)),
            }
        }
        else if map.get(&add_dir((x, y), right)) == Some(&'#') {
            currDir = right;
            movements.push(TurnRight);
        }
        else if map.get(&add_dir((x, y), left)) == Some(&'#') {
            currDir = left;
            movements.push(TurnLeft);
        }
        else {
            break;
        }
    }

    println!("{:?}", movements);

    let mut t = Thruster {
        pointer: 0,
        done: false,
        relative: 0,
        input: vec![],
        program: base.to_owned(),
    };

    t.program[0].set(2);
    let all = "A,B,A,C,B,C,B,C,A,C\nR,12,L,10,R,12\nL,8,R,10,R,6\nR,12,L,10,R,10,L,8\nn\n";
    for c in all.bytes().rev() {
        print!("{} ", c);
        t.input.push(c as i64);
    }
    println!();

    while let Some(v) = t.run_program() {
        if let Ok(c) = u8::try_from(v) {
            print!("{}", c as char);
        }
        else {
            return v;
        }
    }
    0
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i64>().unwrap()))
        .collect();
    
    println!("Part 1: {}", part1(&base));
    println!("Part 2: {}", part2(&base));
}
