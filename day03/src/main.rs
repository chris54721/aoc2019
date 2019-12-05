use std::{fs};
use std::collections::HashMap;

fn main() {
    let input: Vec<Vec<(char, isize)>> = fs::read_to_string("./day03/input/input.txt").unwrap()
        .lines().map(|w| w.split(',').map(|d| {
        let mut iter = d.chars();
        (iter.next().unwrap(), iter.collect::<String>().parse::<isize>().unwrap())
    }).collect()).collect();
    let mut grid: Grid<(u8, usize)> = HashMap::new();
    let mut closest: isize = isize::max_value();
    let mut min_steps: usize = usize::max_value();
    for (w, wire) in input.iter().enumerate() {
        let mut pos = (0, 0);
        let mut steps: usize = 0;
        for dir in wire {
            for _s in 0..dir.1 {
                steps += 1;
                match dir.0 {
                    'L' => pos.0 -= 1,
                    'U' => pos.1 -= 1,
                    'R' => pos.0 += 1,
                    'D' => pos.1 += 1,
                    _ => unreachable!()
                }
                let cur = grid.get(&pos);
                match cur {
                    Some(c) => {
                        if w as u8 != c.0 {
                            let dist = distance(&(0, 0), &pos);
                            if dist < closest {
                                closest = dist;
                            }
                            if steps + c.1 < min_steps {
                                min_steps = steps + c.1;
                            }
                        }
                    },
                    None => {
                        grid.insert(pos, (w as u8, steps));
                    }
                }
            }
        }
    }
    println!("Part 1: {}", closest);
    println!("Part 2: {}", min_steps);
}

fn distance(pos1: &Cell, pos2: &Cell) -> isize {
    return (pos1.0 - pos2.0).abs() + (pos1.1 + pos2.1).abs();
}

type Cell = (isize, isize);
type Grid<T> = HashMap<Cell, T>;
