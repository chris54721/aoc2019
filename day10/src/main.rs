extern crate num;

use std::fs;
use std::collections::HashMap;
use num::Integer;
use std::f32::consts::{PI, FRAC_PI_2};

fn main() {
    let input = fs::read_to_string("./day10/input/input.txt").unwrap();
    let mut asteroids: Vec<Pos> = Vec::new();
    input.lines().enumerate().for_each(|(y, r)| {
        r.chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                asteroids.push(Pos(x as isize, y as isize));
            }
        });
    });
    let mut monitoring_pos: &Pos = &Pos(0, 0);
    let mut visible: Vec<&Pos> = vec![];
    for pos in asteroids.iter() {
        let mut v_curr = HashMap::new();
        for apos in asteroids.iter().filter(|&apos| apos != pos) {
            let d = &pos.direction_to(apos);
            if !v_curr.contains_key(d) || pos.distance(apos) < pos.distance(*v_curr.get(d).unwrap()) {
                v_curr.insert(pos.direction_to(apos), apos);
            }
        }
        if v_curr.len() > visible.len() {
            visible = v_curr.values().copied().collect();
            monitoring_pos = pos;
        }
    }
    visible.sort_by(|&p1, &p2| {
        monitoring_pos.angle_y_to(&p1).partial_cmp(&monitoring_pos.angle_y_to(&p2)).unwrap()
    });
    println!("Part 1: {}", visible.len());
    println!("Part 2: {}", visible[199].0 * 100 + visible[199].1);
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pos(isize, isize);

type Dir = (isize, isize);

impl Pos {
    fn distance(&self, pos2: &Pos) -> isize {
        return (pos2.0 - self.0).abs() + (pos2.1 - self.1).abs();
    }

    fn direction_to(&self, to: &Pos) -> Dir {
        let d = (to.0 - self.0, to.1 - self.1);
        let gcd = d.0.gcd(&d.1);
        (d.0 / gcd, d.1 / gcd)
    }

    fn angle_y_to(&self, to: &Pos) -> f32 {
        let dir = &self.direction_to(to);
        (dir.1 as f32 / dir.0 as f32).atan() + FRAC_PI_2 + (if dir.0 >= 0 { 0f32 } else { PI })
    }
}
