use std::cell::Cell;

fn run_program(program: &[Cell<i32>], input: i32) {
    let mut pointer = 0;
    while pointer < program.len() {
        let digits = program[pointer]
            .get()
            .to_string()
            .chars()
            .rev()
            .collect::<Vec<_>>();

        let get_cell = |a: usize| &program[program[pointer + a].get() as usize];
        let get_parm = |pos: usize| -> i32 {
            if digits.len() > pos + 1 && digits[pos + 1] == '1' {
                program[pointer + pos].get() // value
            } else {
                get_cell(pos).get() // address
            }
        };

        let before_op = pointer;
        match digits[0] {
            '1' => get_cell(3).set(get_parm(1) + get_parm(2)),
            '2' => get_cell(3).set(get_parm(1) * get_parm(2)),
            '3' => get_cell(1).set(input),
            '4' => println!("Output: {}", get_cell(1).get()),
            '5' => {
                if get_parm(1) != 0 {
                    pointer = get_parm(2) as usize
                }
            }
            '6' => {
                if get_parm(1) == 0 {
                    pointer = get_parm(2) as usize
                }
            }
            '7' => get_cell(3).set((get_parm(1) < get_parm(2)) as i32),
            '8' => get_cell(3).set((get_parm(1) == get_parm(2)) as i32),
            '9' => break,
            _ => panic!("help"),
        };

        if pointer == before_op {
            pointer += match digits[0] {
                '3' | '4' => 2,
                '5' | '6' => 3,
                '1' | '2' | '7' | '8' => 4,
                _ => 1,
            };
        }
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
