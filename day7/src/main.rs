use std::cell::Cell;
use permutohedron::Heap;

#[derive(Debug)]
struct Thruster
{
    pointer: usize,
    done: bool,
    input: Vec<i32>,
    program: Vec<Cell<i32>>,
}

impl Thruster {
    fn get_cell(&self, a: usize) -> &Cell<i32> {
        &self.program[self.program[self.pointer + a].get() as usize]
    }

    fn run_program(&mut self) -> Option<i32> {
        let mut result = None;
        while !(self.done || result != None) {
            let digits = self.program[self.pointer]
                .get()
                .to_string()
                .chars()
                .rev()
                .collect::<Vec<_>>();

            let get_parm = |pos: usize| -> i32 {
                if digits.len() > pos + 1 && digits[pos + 1] == '1' {
                    self.program[self.pointer + pos].get() // value
                } else {
                    self.get_cell(pos).get() // address
                }
            };

            let before_op = self.pointer;
            match digits[0] {
                '1' => self.get_cell(3).set(get_parm(1) + get_parm(2)),
                '2' => self.get_cell(3).set(get_parm(1) * get_parm(2)),
                '3' => {
                    let i = self.input.pop();
                    self.get_cell(1).set(i.unwrap());
                }
                '4' => result = Some(self.get_cell(1).get()),
                '5' => {
                    if get_parm(1) != 0 {
                        self.pointer = get_parm(2) as usize
                    }
                }
                '6' => {
                    if get_parm(1) == 0 {
                        self.pointer = get_parm(2) as usize
                    }
                }
                '7' => self.get_cell(3).set((get_parm(1) < get_parm(2)) as i32),
                '8' => self.get_cell(3).set((get_parm(1) == get_parm(2)) as i32),
                '9' => self.done = true,
                _ => panic!("help"),
            };

            if self.pointer == before_op {
                self.pointer += match digits[0] {
                    '3' | '4' => 2,
                    '5' | '6' => 3,
                    '1' | '2' | '7' | '8' => 4,
                    _ => 1,
                };
            }
        }
        result
    }
}

fn thrusters_run(base: &Vec<Cell<i32>>, phase: Vec<i32>, loop_once: bool) -> i32 {
    let mut thrusters: Vec<_> = phase.iter().map(
        |&v| Thruster {
            pointer: 0,
            done: false,
            input: vec![v],
            program: base.clone(),
        }).collect();

    let mut feedback = 0;
    while !&thrusters[4].done {
        for t in &mut thrusters {
            t.input.insert(0, feedback);
            match t.run_program()
            {
                Some(output) => feedback = output,
                None => (),
            }
        }
        if loop_once
        {
            break
        }
    }

    feedback
}

fn find_max_signal(base: &Vec<Cell<i32>>, phase: [i32; 5], loop_once: bool) -> i32
{
    let mut values = phase.to_vec();
    let heap = Heap::new(&mut values);

    heap.map(|v| thrusters_run(&base, v, loop_once)).max().unwrap()
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i32>().unwrap()))
        .collect();

    println!("Part 1: {}", find_max_signal(&base, [0, 1, 2, 3, 4], true));
    println!("Part 2: {}", find_max_signal(&base, [5, 6, 7, 8, 9], false));
}
