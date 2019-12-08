#[derive(Debug)]
struct Layer
{
    width: usize,
    height: usize,

    pixels: Vec<u32>,
}


fn main() {
    let input = include_str!("../input").trim();
    let chars: Vec<_> = input.chars().collect();

    let width = 25;
    let height = 6;
    let mut layers: Vec<_> = chars.chunks(width*height).into_iter().map(|c| Layer{
        width: width,
        height: height,
        pixels: c.iter().map(|x| x.to_digit(10).unwrap()).collect(),
    }).collect();

    layers.sort_by(|a, b| a.pixels.iter().filter(|&&p| p == 0).count().partial_cmp(&b.pixels.iter().filter(|&&p| p == 0).count()).unwrap());

    println!("Part 1: {:?}", layers[0].pixels.iter().filter(|&&p| p == 1).count() * layers[0].pixels.iter().filter(|&&p| p == 2).count());
}
