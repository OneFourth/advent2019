use std::convert::TryInto;

fn stack(cards: &[usize]) -> Vec<usize> {
    cards.iter().copied().rev().collect()
}

fn cut(cards: &[usize], n: isize) -> Vec<usize> {
    let m: isize;
    if n > 0 {
       m = n;
     }
     else {
        m = cards.len() as isize + n;
     }
     let real_n: usize = m.try_into().unwrap();
     cards[real_n..].iter().copied().chain(cards[..real_n].iter().copied()).collect()
}

fn inc(cards: &[usize], n: usize) -> Vec<usize> {
    let mut result = vec![0; cards.len()];
    for (i, &v) in cards.iter().enumerate() {
        result[(i * n) % cards.len()] = v;
    }
    result
}

fn main() {
    let mut cards: Vec<_> = (0..10007).collect();
    {{
    {
    cards = cut(&cards, 3334);
    cards = stack(&cards);
    cards = inc(&cards, 4);
    cards = cut(&cards, -342);
    cards = inc(&cards, 30);
    cards = cut(&cards, -980);
    cards = stack(&cards);
    cards = cut(&cards, -8829);
    cards = inc(&cards, 10);
    cards = cut(&cards, -7351);
    }
    {
    cards = inc(&cards, 60);
    cards = cut(&cards, -3766);
    cards = inc(&cards, 52);
    cards = cut(&cards, 8530);
    cards = inc(&cards, 35);
    cards = cut(&cards, -6979);
    cards = inc(&cards, 52);
    cards = cut(&cards, -8287);
    cards = inc(&cards, 34);
    cards = cut(&cards, -6400);
    cards = inc(&cards, 24);
    }
    {
    cards = stack(&cards);
    cards = inc(&cards, 28);
    cards = cut(&cards, 7385);
    cards = inc(&cards, 32);
    cards = cut(&cards, -1655);
    cards = inc(&cards, 66);
    cards = cut(&cards, -2235);
    cards = inc(&cards, 40);
    cards = cut(&cards, 8121);
    cards = inc(&cards, 71);
    cards = cut(&cards, -2061);
    }
    {
    cards = inc(&cards, 73);
    cards = cut(&cards, 7267);
    cards = inc(&cards, 19);
    cards = cut(&cards, 2821);
    cards = inc(&cards, 16);
    cards = cut(&cards, 7143);
    cards = stack(&cards);
    cards = inc(&cards, 31);
    cards = cut(&cards, 695);
    cards = inc(&cards, 26);
    }
    {
    cards = cut(&cards, 9140);
    cards = inc(&cards, 73);
    cards = cut(&cards, -4459);
    cards = inc(&cards, 17);
    cards = cut(&cards, 9476);
    cards = inc(&cards, 70);
    cards = cut(&cards, -9832);
    cards = inc(&cards, 46);
    cards = stack(&cards);
    }
    cards = inc(&cards, 62);
    cards = cut(&cards, 6490);
    cards = inc(&cards, 29);
    cards = cut(&cards, 3276);
    cards = stack(&cards);
    }
    {
    cards = cut(&cards, 6212);
    cards = inc(&cards, 9);
    cards = cut(&cards, -2826);
    cards = stack(&cards);
    cards = cut(&cards, -1018);
    cards = stack(&cards);
    cards = cut(&cards, -9257);
    cards = inc(&cards, 39);
    cards = cut(&cards, 4023);
    }
    {
    cards = inc(&cards, 69);
    cards = cut(&cards, -8818);
    cards = inc(&cards, 74);
    cards = cut(&cards, -373);
    cards = inc(&cards, 51);
    cards = cut(&cards, 3274);
    cards = inc(&cards, 38);
    cards = cut(&cards, 1940);
    cards = stack(&cards);
    cards = cut(&cards, -3921);
    cards = inc(&cards, 3);
    }
    {
    cards = cut(&cards, -8033);
    cards = inc(&cards, 38);
    cards = cut(&cards, 6568);
    cards = stack(&cards);
    cards = inc(&cards, 68);
    cards = stack(&cards);
    cards = inc(&cards, 70);
    cards = cut(&cards, -9);
    }
    }
    {cards = inc(&cards, 32);
    cards = cut(&cards, -9688);
    cards = inc(&cards, 4);
    cards = stack(&cards);
    cards = cut(&cards, -1197);
    cards = inc(&cards, 54);
    cards = cut(&cards, -582);
    cards = stack(&cards);
    cards = cut(&cards, -404);
    cards = stack(&cards);
    cards = cut(&cards, -8556);
    cards = inc(&cards, 47);
    cards = cut(&cards, 7318);
    cards = inc(&cards, 38);
    cards = cut(&cards, -8758);
    cards = inc(&cards, 48);
    
    }
    
    println!("{:?}", cards.iter().position(|&v| v == 2019));
}
