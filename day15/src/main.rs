use std::cell::Cell;
use std::collections::HashMap;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::{stdout, Read};
use rand::{Rng, thread_rng};
use rand::distributions::{Distribution, Uniform};
use std::{thread, time};

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

fn add_dir(curr: (i64, i64), dir: i64) -> (i64, i64) {
    match dir {
        1 => (curr.0, curr.1 - 1),
        2 => (curr.0, curr.1 + 1),
        3 => (curr.0 - 1, curr.1),
        4 => (curr.0 + 1, curr.1),
        _ => panic!("Wrong movement"),
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

    let mut map = HashMap::new();
    let mut pos = (0, 0);

    let mut found = None;

    let mut stdout = stdout();

    let mut rng = rand::thread_rng();

    let inp = Uniform::new(1, 5);

    let mut count = 0;

    loop {
        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|c| c as char);

        let actual_input: i64 = match input {
            Some('w') => 1,
            Some('s') => 2,
            Some('a') => 3,
            Some('d') => 4,
            _ => continue,
        };

        /*
        let mut actual_input = 0;

        for i in 1..=4 {
            let got = map.get(&add_dir(pos, i));
            if got == None || got == Some(&3) {
                actual_input = i;
            }
        }

        if actual_input == 0 {
            actual_input = inp.sample(&mut rng);
        }
        */

        let new_pos = add_dir(pos, actual_input);

        t.input.push(actual_input);

        match t.run_program() {
            Some(0) => {
                *map.entry(add_dir(pos, actual_input)).or_insert(0) = 0;
            },
            Some(1) => {
                *map.entry(new_pos).or_insert(1) = 1;
                pos = new_pos;
                count += 1;
            }
            Some(2) => {
                *map.entry(new_pos).or_insert(2) = 2;
                found = Some(new_pos);
                count += 1;
                break;
            }
            _ => panic!(),
        }

        stdout.execute(cursor::Hide).ok();
        stdout.execute(terminal::Clear(terminal::ClearType::All)).ok();

        let x_min = map.keys().min_by_key(|(x, _)| x).unwrap().0;
        let x_max = map.keys().max_by_key(|(x, _)| x).unwrap().0;
        let y_min = map.keys().min_by_key(|(_, y)| y).unwrap().1;
        let y_max = map.keys().max_by_key(|(_, y)| y).unwrap().1;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let entry = map.entry((x, y)).or_insert(3);
                if (x, y) == pos {
                    print!("X");
                }
                else {
                    print!(
                        "{}",
                        match *entry {
                            0 => "#",
                            1 => ".",
                            2 => "O",
                            _ => " ",
                        }
                        )
                };
            }
            println!();
        }

        thread::sleep(time::Duration::from_millis(32));
    }

    println!("{:?}", count);
    println!("{:?}", found);
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i64>().unwrap()))
        .collect();

    part1(&base);
}
