use std::cell::Cell;

use std::collections::HashMap;

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

fn turn_to(dir: i64, turn: i64) -> i64 {
    if turn == 0 {
        (dir + 3) % 4
    } else {
        (dir + 1) % 4
    }
}

fn part1(base: &[Cell<i64>]) {
    let mut t = Thruster {
        pointer: 0,
        done: false,
        relative: 0,
        input: vec![],
        program: base.to_owned(),
    };

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut painted = HashMap::new();
    let mut position = (0, 0);
    let mut direction = 0;

    let mut count = 0;

    while !t.done {
        let entry = painted.entry(position).or_insert_with(|| {
            count += 1;
            0
        });
        t.input.push(*entry);
        if let Some(colour) = t.run_program() {
            if let Some(turn) = t.run_program() {
                *entry = colour;
                direction = turn_to(direction, turn);
                let move_dir = directions[direction as usize];
                position = (position.0 + move_dir.0, position.1 + move_dir.1);
            };
        }
    }

    println!("{}", count);
}

fn part2(base: &[Cell<i64>]) {
    let mut t = Thruster {
        pointer: 0,
        done: false,
        relative: 0,
        input: vec![],
        program: base.to_owned(),
    };

    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut painted = HashMap::new();
    let mut position = (0, 0);
    painted.insert(position, 1);
    let mut direction = 0;

    while !t.done {
        let entry = painted.entry(position).or_insert(0);
        t.input.push(*entry);
        if let Some(colour) = t.run_program() {
            if let Some(turn) = t.run_program() {
                *entry = colour;
                direction = turn_to(direction, turn);
                let move_dir = directions[direction as usize];
                position = (position.0 + move_dir.0, position.1 + move_dir.1);
            };
        }
    }

    let x_min = painted.keys().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = painted.keys().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = painted.keys().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = painted.keys().max_by_key(|(_, y)| y).unwrap().1;

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            let entry = painted.entry((x, y)).or_insert(0);
            print!(
                "{}",
                match *entry {
                    0 => "  ",
                    1 => "##",
                    _ => "  ",
                }
            );
        }
        println!();
    }
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i64>().unwrap()))
        .collect();

    part1(&base);
    part2(&base);
}
