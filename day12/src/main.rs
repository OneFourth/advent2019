use regex::Regex;

type Point = [i64; 3];

#[derive(Debug)]
struct Planet {
    position: Point,
    velocity: Point,
}

impl Planet {
    fn apply_vel(&mut self) {
        self.position
            .iter_mut()
            .zip(self.velocity.iter())
            .for_each(|(a, b)| *a += b);
    }

    fn get_total_energy(&self) -> i64 {
        self.position.iter().map(|p| p.abs()).sum::<i64>()
            * self.velocity.iter().map(|v| v.abs()).sum::<i64>()
    }
}

fn apply_grav(p1: &mut Planet, p2: &mut Planet) {
    for i in 0..3 {
        match (p1.position[i], p2.position[i]) {
            (a, b) if a < b => {
                p1.velocity[i] += 1;
                p2.velocity[i] -= 1;
            }
            (a, b) if a > b => {
                p1.velocity[i] -= 1;
                p2.velocity[i] += 1;
            }
            _ => (),
        }
    }
}

fn main() {
    let xyz_reg = Regex::new(r"<x=([^,]+), y=([^,]+), z=([^,]+)>").unwrap();
    let input = include_str!("../input");

    let mut planets: Vec<_> = xyz_reg
        .captures_iter(input)
        .map(|cap| Planet {
            position: [
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
            ],
            velocity: [0, 0, 0],
        })
        .collect();

    let planet_count = planets.len();

    for _ in 1..=1000 {
        for i in 0..planet_count {
            let (left, right) = planets.split_at_mut(i + 1);
            let mut p1 = &mut left[i];

            right
                .iter_mut()
                .for_each(|mut p2| apply_grav(&mut p1, &mut p2));

            p1.apply_vel();
        }
    }

    println!(
        "Part 1: {:?}",
        planets.iter().map(|p| p.get_total_energy()).sum::<i64>()
    );
}
