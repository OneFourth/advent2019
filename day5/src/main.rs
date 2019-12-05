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
            '5' => {
                if get_parm(1, &digits, pointer, program) != 0 {
                    pointer = get_parm(2, &digits, pointer, program) as usize;
                } else {
                    pointer += 3;
                }
            }
            '6' => {
                if get_parm(1, &digits, pointer, program) == 0 {
                    pointer = get_parm(2, &digits, pointer, program) as usize;
                } else {
                    pointer += 3;
                }
            }
            '7' => {
                program[program[pointer + 3].get() as usize].set(
                    (get_parm(1, &digits, pointer, program)
                        < get_parm(2, &digits, pointer, program)) as i32,
                );
                pointer += 4;
            }
            '8' => {
                program[program[pointer + 3].get() as usize].set(
                    (get_parm(1, &digits, pointer, program)
                        == get_parm(2, &digits, pointer, program)) as i32,
                );
                pointer += 4;
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

    let part1 = base.clone();
    println!("Part 1");
    run_program(&part1, 1);

    println!("Part 2");
    let part2 = base.clone();
    run_program(&part2, 5);
}
