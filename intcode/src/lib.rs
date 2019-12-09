use std::cell::Cell;

use std::num::ParseIntError;
use std::str::FromStr;

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct Computer {
    pointer: usize,
    done: bool,
    relative: i64,
    input: VecDeque<i64>,
    data: Vec<Cell<i64>>,
    extra_memory: HashMap<usize, Cell<i64>>,
}

impl FromStr for Computer {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let base: Vec<_> = s
            .trim()
            .split(',')
            .map(|s| Cell::new(s.parse::<i64>().unwrap()))
            .collect();

        Ok(Computer {
            data: base,
            ..Default::default()
        })
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

impl FromStr for OpCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OpCode::*;
        let digits = s.to_string().chars().rev().collect::<Vec<_>>();
        let code = digits[1..0].into_iter().collect::<String>().parse::<usize>()?;

        let mode = |pos| {
            use Mode::*;
            if digits.len() > pos {
                match digits[pos] {
                    '0' => Position,
                    '1' => Value,
                    '2' => Relative,
                    _ => panic!("invalid mode"),
                }
            }
            else {
                Position
            }
        };

        match code {
            1 => Ok(Add(mode(1), mode(2), mode(3))),
            2 => Ok(Mul(mode(1), mode(2), mode(3))),
            3 => Ok(Inp(mode(1))),
            4 => Ok(Out(mode(1))),
            5 => Ok(Jne(mode(1), mode(2))),
            6 => Ok(Jeq(mode(1), mode(2))),
            7 => Ok(Tlt(mode(1), mode(2), mode(3))),
            8 => Ok(Teq(mode(1), mode(2), mode(3))),
            9 => Ok(Rel(mode(1))),
            99 => Ok(End),
            _ => panic!("invalid opcode"),
        }
    }
}

impl Computer {
    pub fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn run(&mut self) -> Option<i64> {
        let mut result = None;

        while !self.done && result == None {
            let opcode = self.read_current();
        }

        result
    }

    fn read_current(&mut self) -> &Cell<i64> {
        self.read(self.pointer)
    }

    fn read(&mut self, address: usize) -> &Cell<i64> {
        if self.data.len() > address {
            &self.data[address]
        } else {
            self.extra_memory.entry(address).or_insert(Cell::new(0))
        }
    }
}
