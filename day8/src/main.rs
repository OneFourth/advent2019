use std::ops;

#[derive(Debug, Clone)]
struct Layer
{
    pixels: Vec<u32>,
}

impl ops::Add<Layer> for Layer {
    type Output = Layer;

    fn add(self, _rhs: Layer) -> Layer {
        Layer {
            pixels: self.pixels.iter().zip(_rhs.pixels.iter()).map(|(a, b)| {
                match (a, b) {
                    (2, _) => *b,
                    (_, _) => *a,
                }
            }).collect(),
        }
    }
}

fn main() {
    let input = include_str!("../input").trim();
    let chars: Vec<_> = input.chars().collect();

    let width = 25;
    let height = 6;
    let layers: Vec<_> = chars.chunks(width*height).into_iter().map(|c| Layer{
        pixels: c.iter().map(|x| x.to_digit(10).unwrap()).collect(),
    }).collect();

    {
        let mut part1 = layers.clone();

        part1.sort_by(|a, b| a.pixels.iter().filter(|&&p| p == 0).count().partial_cmp(&b.pixels.iter().filter(|&&p| p == 0).count()).unwrap());

        println!("Part 1: {:?}", part1[0].pixels.iter().filter(|&&p| p == 1).count() * part1[0].pixels.iter().filter(|&&p| p == 2).count());
    }

    {
        let part2 = layers.clone();

        let mut layer = part2.first().unwrap().clone();
        for n in 1..part2.len() {
            layer = layer + part2[n].clone();
        }

        println!("Part 2:");
        for c in layer.pixels.chunks(25)
        {
            for p in c
            {
                print!("{}", p);
            }
            print!("\n");
        }
    }
}
