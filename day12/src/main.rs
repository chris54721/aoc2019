extern crate num;

use std::fs;
use utils::vec3d::Vec3d;
use num::Integer;

fn main() {
    let mut moons: Vec<Moon> = fs::read_to_string("./day12/input/input.txt").unwrap()
        .lines().map(|l| Moon::new(l.parse().unwrap())).collect();
    let initial_states: Vec<AxisState> = [0, 1, 2].iter()
        .map(|a| axis_state(&moons, *a))
        .collect();
    let mut axis_repeat_steps: [usize; 3] = [0, 0, 0];
    let mut i: usize = 0;
    let mut energy_1k = 0;
    while axis_repeat_steps.iter().filter(|&&x| x == 0).count() > 0 {
        for a in 0..=2 {
            for m1 in 0..moons.len() - 1 {
                for m2 in m1 + 1..moons.len() {
                    let dg = (moons[m2].pos[a] - moons[m1].pos[a]).signum();
                    moons[m1].vel[a] += dg;
                    moons[m2].vel[a] -= dg;
                }
            }
            for m in moons.iter_mut() {
                m.pos[a] += m.vel[a];
            }
            if axis_state(&moons, a) == initial_states[a] {
                axis_repeat_steps[a] = i + 1;
            }
        }
        i += 1;
        if i == 1000 {
            energy_1k = moons.iter().map(|m| m.energy()).sum();
        }
    }

    println!("Part 1: {}", energy_1k);
    println!("Part 2: {}", axis_repeat_steps.iter().fold(1, |acc: usize, &x| acc.lcm(&x)));
}

fn axis_state(moons: &Vec<Moon>, axis: usize) -> AxisState {
    moons.iter().map(|m| vec![m.pos[axis], m.vel[axis]]).flatten().collect()
}

type AxisState = Vec<isize>;

struct Moon {
    pos: Vec3d,
    vel: Vec3d,
}

impl Moon {
    fn new(pos: Vec3d) -> Moon {
        Moon { pos, vel: Vec3d::new(0, 0, 0) }
    }

    fn pot(&self) -> isize {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kin(&self) -> isize {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }

    fn energy(&self) -> isize {
        self.pot() * self.kin()
    }
}

