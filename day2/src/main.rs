use std::cell::Cell;

fn main() {
    let input = include_str!("../input");

    let program: Vec<_> = input
        .trim()
        .split(",")
        .map(|s| Cell::new(s.parse::<usize>().unwrap()))
        .collect();

    program[1].set(12);
    program[2].set(2);

    for n in program.chunks(4)
    {
        println!("{:?}", n);
        match n[0].get()
        {
            1 => program[n[3].get()].set(program[n[1].get()].get() + program[n[2].get()].get()),
            2 => program[n[3].get()].set(program[n[1].get()].get() * program[n[2].get()].get()),
            99 => break,
            _ => (),
        }
    }

    println!("Part 1: {}", program[0].get());
}
