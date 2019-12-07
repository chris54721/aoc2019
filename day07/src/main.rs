#[macro_use]
extern crate maplit;

use std::{fs, thread};
use std::collections::HashMap;

fn main() {
    let input: Vec<isize> = fs::read_to_string("./day07/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).collect();
    let mut max_out: isize = isize::min_value();
    for phases in permutations(5) {
        let mut last_output = 0;
        for phase in phases {
            last_output = run(&input, vec![phase as isize, last_output]);
        }
        if last_output > max_out {
            max_out = last_output;
        }
    }
    println!("Part 1: {}", max_out);
}

fn run(tape: &Vec<isize>, in_values: Vec<isize>) -> isize {
    let mut input = tape.clone();
    let mut output: isize = 0;
    let opcodes = get_opcodes();
    let mut in_iter = in_values.iter();
    let mut i: usize = 0;
    loop {
        let op: &Opcode = opcodes.get(&(input[i] % 100)).unwrap();
        let param_modes = [input[i] / 100 % 10, input[i] / 1000 % 10, input[i] / 10000 % 10];
        let params: Vec<isize> = input[i + 1..=i + op.param_count].iter().enumerate()
            .map(|(p_idx, p)| {
                let addr = op.addr_param_idx.is_some() && p_idx == op.addr_param_idx.unwrap();
                if addr || param_modes[p_idx] == 1 { *p } else { input[*p as usize] }
            }).collect();
        let r = (&op.apply)(&params, &mut input);
        match r {
            OpResult::NONE => {}
            OpResult::INPUT => input[params[op.addr_param_idx.unwrap()] as usize] = *in_iter.next().unwrap(),
            OpResult::OUTPUT => output = input[params[op.addr_param_idx.unwrap()] as usize],
            OpResult::WRITE(v) => input[params[op.addr_param_idx.unwrap()] as usize] = v,
            OpResult::JUMP(d) => {
                i = d as usize;
                continue;
            }
            OpResult::HALT => break
        }
        i += op.param_count + 1;
    }
    output
}

fn get_opcodes() -> HashMap<isize, Opcode> {
    hashmap! {
        1 => Opcode::new(&|v, _| OpResult::WRITE(v[0] + v[1]), 3, Some(2)),
        2 => Opcode::new(&|v, _| OpResult::WRITE(v[0] * v[1]), 3, Some(2)),
        3 => Opcode::new(&|_, _| OpResult::INPUT, 1, Some(0)),
        4 => Opcode::new(&|_, _| OpResult::OUTPUT, 1, Some(0)),
        5 => Opcode::new(&|v, _| if v[0] != 0 { OpResult::JUMP(v[1]) } else { OpResult::NONE }, 2, None),
        6 => Opcode::new(&|v, _| if v[0] == 0 { OpResult::JUMP(v[1]) } else { OpResult::NONE }, 2, None),
        7 => Opcode::new(&|v, _|  OpResult::WRITE(if v[0] < v[1] { 1 } else { 0 }), 3, Some(2)),
        8 => Opcode::new(&|v, _|  OpResult::WRITE(if v[0] == v[1] { 1 } else { 0 }), 3, Some(2)),
        99 => Opcode::new(&|_, _| OpResult::HALT, 0, None),
    }
}

type OpFn = dyn Fn(&Vec<isize>, &mut Vec<isize>) -> OpResult;

struct Opcode {
    apply: &'static OpFn,
    param_count: usize,
    addr_param_idx: Option<usize>, // param which should be always treated as an address
}

impl Opcode {
    fn new(apply: &'static OpFn, param_count: usize, addr_param_idx: Option<usize>) -> Opcode {
        Opcode { apply, param_count, addr_param_idx }
    }
}

enum OpResult {
    NONE,
    WRITE(isize),
    INPUT,
    OUTPUT,
    JUMP(isize),
    HALT,
}

pub fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size).collect(), swaps: vec![0; size], i: 0 }
}

pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}

impl Iterator for Permutations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}