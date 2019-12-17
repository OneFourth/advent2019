use intcode::{parse_program, Computer, Program};
use permutohedron::Heap;

fn thrusters_run(base: Program, phase: &[i64], loop_once: bool) -> i64 {
    let mut computers: Vec<_> = phase
        .iter()
        .map(|p| {
            let mut c = Computer::new(base.clone());
            c.push(*p);
            c
        })
        .collect();

    let mut feedback = 0;
    while !&computers.last().unwrap().done {
        for c in &mut computers {
            c.push(feedback);
            if let Some(output) = c.run() {
                feedback = output
            }
        }
        if loop_once {
            break;
        }
    }

    feedback
}

fn find_max_signal(base: Program, phase: [i64; 5], loop_once: bool) -> i64 {
    let mut values = phase.to_vec();
    let heap = Heap::new(&mut values);

    heap.map(|v| thrusters_run(base.clone(), &v, loop_once))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input");
    let base = parse_program(input);

    println!(
        "Part 1: {}",
        find_max_signal(base.clone(), [0, 1, 2, 3, 4], true)
    );
    println!(
        "Part 2: {}",
        find_max_signal(base.clone(), [5, 6, 7, 8, 9], false)
    );
}
