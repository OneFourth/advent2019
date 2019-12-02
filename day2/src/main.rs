use std::cell::Cell;

fn run_program(program: &Vec<Cell<usize>>) -> usize
{
    for n in program.chunks(4)
    {
        match n[0].get()
        {
            1 => program[n[3].get()].set(program[n[1].get()].get() + program[n[2].get()].get()),
            2 => program[n[3].get()].set(program[n[1].get()].get() * program[n[2].get()].get()),
            99 => break,
            _ => (),
        };
    }

    return program[0].get()
}

fn main() {
    let input = include_str!("../input");

    let base: Vec<_> = input
        .trim()
        .split(",")
        .map(|s| Cell::new(s.parse::<usize>().unwrap()))
        .collect();

    let program = base.clone();

    program[1].set(12);
    program[2].set(2);

    println!("Part 1: {}", run_program(&program));

    let target = 19690720;

    for verb in 0..99
    {
        for noun in 0..99
        {
            let clone = base.clone();
            clone[1].set(noun);
            clone[2].set(verb);
            if run_program(&clone) == target
            {
                println!("Part 2: {}", 100 * noun + verb);
            }
        }
    }
}
