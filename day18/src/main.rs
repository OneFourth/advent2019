use pathfinding::directed::astar::astar;
use pathfinding::grid::Grid;

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug, Default)]
struct KeyDoor {
    key: (usize, usize),
    door: (usize, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EqGrid {
    grid_key: String,
    player: (usize, usize),
}

fn main() {
    let input = include_str!("../input");
    let width = input.find('\n').unwrap() - 1;
    let height = input.trim().lines().count();

    let grid_cache = RefCell::new(HashMap::new());

    let mut original = EqGrid {
        grid_key: "".to_string(),
        player: (0, 0),
    };

    let mut key_doors: HashMap<_, KeyDoor> = HashMap::new();
    let mut original_grid = Grid::new(width, height);

    input.lines().enumerate().for_each(|(y, l)| {
        l.trim().chars().enumerate().for_each(|(x, c)| match c {
            '.' => {
                original_grid.add_vertex((x, y));
            }
            '#' => (),
            '@' => {
                original_grid.add_vertex((x, y));
                original.player = (x, y);
            }
            'a'..='z' => {
                original_grid.add_vertex((x, y));
                key_doors.entry(c).or_insert(Default::default()).key = (x, y);
            }
            'A'..='Z' => {
                key_doors
                    .entry(c.to_lowercase().next().unwrap())
                    .or_insert(Default::default())
                    .door = (x, y);
            }
            _ => panic!("Unhandled character"),
        })
    });

    grid_cache.borrow_mut().insert("".to_string(), original_grid);

    let path = astar(&original, |g| {
        key_doors.iter().filter_map(|(k, v)| {
            if !grid_cache.borrow()[&g.grid_key].has_vertex(&v.door) {
                let path = astar(
                    &g.player,
                    |&p| grid_cache.borrow()[&g.grid_key].neighbours(&p).into_iter().map(|n| (n, 1)),
                    |&p| grid_cache.borrow()[&g.grid_key].distance(&p, &v.key),
                    |&p| p == v.key,
                    );
                if let Some((_, cost)) = path {
                    let mut grid = grid_cache.borrow()[&g.grid_key].clone();
                    grid.add_vertex(v.door);

                    let mut chars: Vec<_> = g.grid_key.chars().collect();
                    chars.push(*k);
                    chars.sort();

                    let grid_key: String = chars.into_iter().collect();
                    grid_cache.borrow_mut().insert(grid_key.clone(), grid);

                    return Some((EqGrid {
                        player: v.key,
                        grid_key,
                    }, cost));
                }
            }

            None
        }).collect::<Vec<_>>()
    }, |g| {
        let g_grid = &grid_cache.borrow()[&g.grid_key];
        key_doors.iter().map(|(_, v)| {
            if !g_grid.has_vertex(&v.door) {
                g_grid.distance(&g.player, &v.key)
            }
            else {
                0
            }
        }).sum::<usize>()
    }, |g| {
        println!("{:?}", g.grid_key);
        let g_grid = &grid_cache.borrow()[&g.grid_key];
        //g_grid.has_vertex(&key_doors[&'c'].door)
        key_doors.iter().all(|(_, v)| g_grid.has_vertex(&v.door))
    });

    println!("{:?}", path);
    /*
    for (k, v) in &key_doors {
        let ast = astar(
            &player,
            |&p| grid.neighbours(&p).into_iter().map(|n| (n, 1)),
            |&p| grid.distance(&p, &v.key),
            |&p| p == v.key,
        );
        if let Some(path) = ast {
            println!("{}: {}", k, path.1);
        }
        //println!("{}, {:?}, {}", k, grid.neighbours(&player), grid.distance(&player, &v.key));
    }
    */

    //println!("{:?}", grid);
    //println!("{:?}", key_doors);
    //println!("{:?}", player);
}
