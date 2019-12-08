#[macro_use]
extern crate maplit;

use std::fs;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let input: Vec<isize> = fs::read_to_string("./day07/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).collect();
    println!("Part 1: {}", run_amp(&input, 0..=4, false));
    println!("Part 2: {}", run_amp(&input, 5..=9, true));
}

fn run_amp(tape: &Vec<isize>, phase_range: RangeInclusive<usize>, cycle: bool) -> isize {
    let mut max_out: isize = isize::min_value();
    for phases in permutations(phase_range, 5) {
        let mut states: Vec<(Vec<isize>, usize)> = vec![(tape.clone(), 0); 5];
        let mut last_out = 0;
        for (idx, &phase) in phases.iter().enumerate().cycle() {
            let state = states.get_mut(idx).unwrap();
            let mut in_values: Vec<isize> = Vec::new();
            if state.1 == 0 {
                in_values.push(phase as isize);
            }
            in_values.push(last_out);
            let result = run(&mut state.0, in_values, &mut state.1);
            if result.is_some() {
                last_out = result.unwrap();
                if !cycle && idx == 4 {
                    break;
                }
            } else if idx == 4 {
                break;
            }
        }
        if last_out > max_out {
            max_out = last_out;
        }
    }
    max_out
}

fn run(input: &mut Vec<isize>, in_values: Vec<isize>, i: &mut usize) -> Option<isize> {
    let opcodes = get_opcodes();
    let mut in_iter = in_values.iter();
    loop {
        let op: &Opcode = opcodes.get(&(input[*i] % 100)).unwrap();
        let param_modes = [input[*i] / 100 % 10, input[*i] / 1000 % 10, input[*i] / 10000 % 10];
        let params: Vec<isize> = input[*i + 1..=*i + op.param_count].iter().enumerate()
            .map(|(p_idx, p)| {
                let addr = op.addr_param_idx.is_some() && p_idx == op.addr_param_idx.unwrap();
                if addr || param_modes[p_idx] == 1 { *p } else { input[*p as usize] }
            }).collect();
        let r = (&op.apply)(&params, input);
        match r {
            OpResult::NONE => {}
            OpResult::INPUT => input[params[op.addr_param_idx.unwrap()] as usize] = *in_iter.next().unwrap(),
            OpResult::OUTPUT => {
                *i += op.param_count + 1;
                return Some(input[params[op.addr_param_idx.unwrap()] as usize])
            },
            OpResult::WRITE(v) => input[params[op.addr_param_idx.unwrap()] as usize] = v,
            OpResult::JUMP(d) => {
                *i = d as usize;
                continue;
            }
            OpResult::HALT => break
        }
        *i += op.param_count + 1;
    }
    None
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

pub fn permutations(range: RangeInclusive<usize>, size: usize) -> Permutations {
    Permutations { idxs: range.collect(), swaps: vec![0; size], i: 0 }
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