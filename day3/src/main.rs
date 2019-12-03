use std::collections::HashSet;

type Point = (i32, i32);

type Wire = HashSet<Point>;

fn build_wire(input_str: &str) -> Wire {
    let mut wire = HashSet::new();

    let mut x = 0;
    let mut y = 0;

    for dir in input_str.trim().split(',')
    {
        let (head, tail) = dir.split_at(1);
        let count = tail.parse::<i32>().unwrap();
        let (a_x, a_y) = match head
            {
                "U" => ( 0,  1),
                "D" => ( 0, -1),
                "L" => (-1,  0),
                "R" => ( 1,  0),
                _ => (0, 0)
            };

        println!("{}, {}, {}", count, a_x, a_y);
        for _ in 1..count+1 {
            x += a_x;
            y += a_y;

            wire.insert((x,y));
        }
    }
    wire
}

fn output(wires: Vec<Wire>)
{
    let mut out: Vec<Vec<char>> = vec![vec!['.'; 1000]; 1000];
    out[300][300] = 'o';

    let mut count = 1;
    for w in wires
    {
        for (x, y) in w
        {
            out[(y+300) as usize][(x+300) as usize] = std::char::from_digit(count, 10).unwrap();
        }
        count += 1;
    }

    for s in out
    {
        println!("{:?}", s.into_iter().collect::<String>());
    }
}

fn main() {
    let input = include_str!("../input");

    let wires: Vec<_> = input.lines().map(|s| build_wire(s)).collect();
    let distances: Vec<_> = wires[0].intersection(&wires[1]).map(|(x, y)| x.abs() + y.abs()).collect();

    println!("{:?}", distances);
    println!("Part 1: {}", distances.iter().min().unwrap());

    //output(wires);
}
