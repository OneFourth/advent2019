use std::cell::Cell;

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

    fn get_parm(&mut self, pos: usize, digits: &Vec<char>) -> &Cell<i64> {
        if digits.len() > pos + 1 {
            match digits[pos + 1] {
                '0' => self.get_cell(pos),
                '1' => self.access_program(self.pointer + pos), // value
                '2' => {
                    let address = (self.relative + self.access_program(self.pointer + pos).get()) as usize;
                    self.access_program(address)
                }, // relative
                _ => panic!("help get_parm"),
            }
        } else {
            self.get_cell(pos) // address
        }
    }

    fn run_program(&mut self) -> Option<i64> {
        let mut result = None;
        while !(self.done || result != None) {
            let mut digits = self.access_program(self.pointer)
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
                },
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

fn run(base: &Vec<Cell<i64>>, input: Vec<i64>) {
    let mut t = Thruster {
        pointer: 0,
        done: false,
        relative: 0,
        input: input,
        program: base.clone(),
    };

    while !t.done {
        match t.run_program()
        {
            Some(a) => println!("Output: {}", a),
            None => println!("Done!"),
        }
    }
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i64>().unwrap()))
        .collect();

    run(&base, vec![1]);
    run(&base, vec![2]);
}
