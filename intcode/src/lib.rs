use std::cell::Cell;

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct Computer {
    pointer: usize,
    done: bool,
    rel_pointer: i64,
    input: VecDeque<i64>,
    data: Vec<Cell<i64>>,
    extra_memory: HashMap<usize, Cell<i64>>,
}

impl Computer {
    fn new(s: &str) -> Self {
        let data: Vec<_> = s
            .trim()
            .split(',')
            .map(|s| Cell::new(s.parse::<i64>().unwrap()))
            .collect();

        Computer {
            data,
            ..Default::default()
        }
    }
}

enum Mode {
    Position,
    Value,
    Relative,
}

enum OpCode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Inp(Mode),
    Out(Mode),
    Jne(Mode, Mode),
    Jeq(Mode, Mode),
    Tlt(Mode, Mode, Mode),
    Teq(Mode, Mode, Mode),
    Rel(Mode),
    End,
}

impl OpCode {
    fn new(s: &str) -> Self {
        use OpCode::*;
        let number = s.parse::<u8>()?;
        let digits = [number % 100, number % 1_000, number % 10_000, number % 100_000];

        let mode = |pos| {
            use Mode::*;
            match digits[pos] {
                0 => Position,
                1 => Value,
                2 => Relative,
                _ => panic!("invalid mode"),
            }
        };

        match code {
            1 => Add(mode(1), mode(2), mode(3)),
            2 => Mul(mode(1), mode(2), mode(3)),
            3 => Inp(mode(1)),
            4 => Out(mode(1)),
            5 => Jne(mode(1), mode(2)),
            6 => Jeq(mode(1), mode(2)),
            7 => Tlt(mode(1), mode(2), mode(3)),
            8 => Teq(mode(1), mode(2), mode(3)),
            9 => Rel(mode(1)),
            99 => End,
            _ => panic!("invalid opcode"),
        }
    }
}

impl Computer {
    pub fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn run(&mut self) -> Option<i64> {
        while !self.done && result == None {
            match OpCode::new(self.read_current()) {
                Add(m1, m2, m3) => self.read_mode(m3).set(self.read_mode(m1).get() + self.read_mode(m2).get()),
                Mul(m1, m2, m3) => self.read_mode(m3).set(self.read_mode(m1).get() * self.read_mode(m2).get()),
                Inp(m1) => self.read_mode(m1).set(input.pop_front()),
                Out(m1) => result = self.read_mode(m1).get(),
                Jne(m1, m2) => {
                    if self.read_mode(m1).get() != 0 {
                        self.pointer = self.read_mode(m2).get();
                    }
                },
                Jeq(m1, m2) => {
                    if self.read_mode(m1).get() == 0 {
                        self.pointer = self.read_mode(m2).get();
                    }
                },
                Tlt(m1, m2, m3) => self.read_mode(m3).set((self.read_mode(m1) < self.read_mode(m2)) as i64),
                Teq(m1, m2, m3) => self.read_mode(m3).set((self.read_mode(m1) == self.read_mode(m2)) as i64),
                Rel(Mode) => ,
                End => self.done = true,
            }
        }

        None
    }

    fn read_mode(&mut self, mode: Mode) -> i64 {
        use Mode::*;
        let address = self.pointer;
        self.pointer += 1;

        let address_to_read = match mode {
            Position => checked_read(checked_read(address).get()).get(),
            Value => checked_read(address).get(),
            Relative => checked_read(address + self.rel_pointer).get(),
        };
    }

    fn read_current(&mut self) -> &Cell<i64> {
        let address = self.pointer;
        self.pointer += 1;
        self.checked_read(address);
    }

    fn checked_read(&mut self, address: usize) -> &Cell<i64> {
        if self.data.len() > address {
            &self.data[address]
        } else {
            self.extra_memory.entry(address).or_insert(Cell::new(0))
        }
    }
}
