use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Material {
    amount: i64,
    name: String,
}

impl Material {
    fn new(s: &str) -> Self {
        let mut data = s.trim().split(' ');
        Material {
            amount: data.next().unwrap().parse().unwrap(),
            name: data.next().unwrap().to_string(),
        }
    }
}

type Reaction = (Vec<Material>, Material);

fn new_reaction(s: &str) -> (String, Reaction) {
    let mut data = s.trim().split("=>");
    let i = data.next().unwrap();
    let o = data.next().unwrap();

    let out = Material::new(o);

    (
        out.name.to_owned(),
        (
            i.split(',').map(|s| Material::new(s)).collect::<Vec<_>>(),
            out,
        ),
    )
}

#[derive(Debug)]
struct State<'a> {
    available: HashMap<String, i64>,
    reactions: &'a HashMap<String, Reaction>,
    used_ore: usize,
}

impl State<'_> {
    fn try_create(&mut self, mat: &str, need_amount: i64) {
        let reaction = self.reactions.get(mat).unwrap();
        if let Some(m) = reaction.0.get(0) {
            while *self.available.entry(mat.to_string()).or_insert(0) < need_amount {
                if m.name == "ORE" {
                    self.available
                        .entry(mat.to_string())
                        .and_modify(|e| *e += reaction.1.amount)
                        .or_insert(reaction.1.amount);
                    self.used_ore += m.amount as usize;
                } else {
                    for req_m in &reaction.0 {
                        self.try_create(&req_m.name, req_m.amount);
                        self.available
                            .entry(req_m.name.clone())
                            .and_modify(|e| *e -= req_m.amount);
                    }
                    self.available
                        .entry(mat.to_string())
                        .and_modify(|e| *e += reaction.1.amount);
                }
            }
        }
    }
}

fn part1(reactions: &HashMap<String, Reaction>) -> usize {
    let mut s = State {
        available: HashMap::new(),
        reactions: &reactions,
        used_ore: 0,
    };

    s.try_create("FUEL", 1);

    s.used_ore
}

fn part2(reactions: &HashMap<String, Reaction>) -> i64 {
    let mut s = State {
        available: HashMap::new(),
        reactions: &reactions,
        used_ore: 0,
    };

    while s.used_ore < 1_000_000_000_000 {
        let a = *s.available.entry("FUEL".to_string()).or_insert(0) + 1;
        s.try_create("FUEL", a);

        if s.available["FUEL"] % 10_000 == 0 {
            println!("{}, {}, {}", s.used_ore, s.available["FUEL"], (s.used_ore as f64) / 1_000_000_000_000.0);
        }
    }

    s.available["FUEL"] - 1
}

fn main() {
    let input = include_str!("../input");
    let reactions: HashMap<_, _> = input.trim().lines().map(new_reaction).collect();

    println!("Part 1: {}", part1(&reactions));
    println!("Part 2: {}", part2(&reactions));
}
