use std::cell::Cell;

fn get_parm(pos: usize, digits: &[char], pointer: usize, program: &[Cell<i32>]) -> i32 {
    if digits.len() > pos + 1 && digits[pos + 1] == '1' {
        program[pointer + pos].get() // value
    } else {
        program[program[pointer + pos].get() as usize].get() // address
    }
}

fn run_program(program: &[Cell<i32>], input: i32) {
    let mut pointer = 0;
    while pointer < program.len() {
        let digits = program[pointer]
            .get()
            .to_string()
            .chars()
            .rev()
            .collect::<Vec<_>>();
        match digits[0] {
            '1' => {
                program[program[pointer + 3].get() as usize].set(
                    get_parm(1, &digits, pointer, program) + get_parm(2, &digits, pointer, program),
                );
                pointer += 4;
            }
            '2' => {
                program[program[pointer + 3].get() as usize].set(
                    get_parm(1, &digits, pointer, program) * get_parm(2, &digits, pointer, program),
                );
                pointer += 4;
            }
            '3' => {
                program[program[pointer + 1].get() as usize].set(input);
                pointer += 2;
            }
            '4' => {
                println!(
                    "Output: {}",
                    program[program[pointer + 1].get() as usize].get()
                );
                pointer += 2;
            }
            '9' => break,
            _ => panic!("help"),
        };
    }
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| Cell::new(s.parse::<i32>().unwrap()))
        .collect();

    run_program(&base, 1);
}
