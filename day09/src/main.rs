#[macro_use]
extern crate maplit;

use std::fs;
use std::collections::HashMap;

fn main() {
    let input: Memory = fs::read_to_string("./day09/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).enumerate().collect();
    println!("Part 1: {}", VM::new(&input).run(vec![1])[0]);
    println!("Part 2: {:?}", VM::new(&input).run(vec![2])[0]);
}

struct VM {
    i: usize,
    memory: Memory,
    rel_base: usize,
}

impl VM {
    fn new(input: &Memory) -> VM {
        VM { i: 0, memory: input.clone(), rel_base: 0 }
    }

    fn run(&mut self, in_values: Vec<isize>) -> Vec<isize> {
        let opcodes = get_opcodes();
        let mut in_iter = in_values.iter();
        let mut outputs = Vec::new();
        loop {
            let op: &Opcode = opcodes.get(&(self.memory[&self.i] % 100)).unwrap();
            let params = &self.fetch_params(&op);
            match (&op.apply)(&params, &mut self.memory) {
                OpResult::None => {}
                OpResult::Input => {
                    self.memory.insert(params[0] as usize, *in_iter.next().unwrap());
                }
                OpResult::Output => {
                    outputs.push(self.memory[&(params[0] as usize)]);
                }
                OpResult::Write(v) => {
                    self.memory.insert(params[op.addr_param_idx.unwrap()] as usize, v);
                }
                OpResult::Jump(d) => {
                    self.i = d as usize;
                    continue;
                }
                OpResult::SetRelBase(r) => {
                    self.rel_base = ((self.rel_base as isize) + r) as usize;
                }
                OpResult::Halt => break
            }
            self.i += op.param_count + 1;
        }
        outputs
    }

    fn fetch_params(&self, op: &Opcode) -> Vec<isize> {
        let mut params: Vec<isize> = Vec::with_capacity(op.param_count);
        let param_modes: Vec<_> = [100, 1000, 10000].iter()
            .map(|&p| self.memory[&self.i] / p % 10)
            .collect();
        let mem_fetch = |x: isize, a: bool| -> isize {
            if !a { *self.memory.get(&(x as usize)).unwrap_or_else(|| &0) } else { x }
        };
        for p_idx in 0..op.param_count {
            let p = self.memory[&(self.i + p_idx + 1)];
            let mode = param_modes[p_idx];
            let is_addr = op.addr_param_idx.is_some() && p_idx == op.addr_param_idx.unwrap();
            params.push(match mode {
                0 => mem_fetch(p, is_addr),
                1 => p,
                2 => mem_fetch(self.rel_base as isize + p, is_addr),
                _ => unreachable!()
            });
        }
        params
    }
}

fn get_opcodes() -> HashMap<isize, Opcode> {
    hashmap! {
        1 => Opcode::new(&|v, _| OpResult::Write(v[0] + v[1]), 3, Some(2)),
        2 => Opcode::new(&|v, _| OpResult::Write(v[0] * v[1]), 3, Some(2)),
        3 => Opcode::new(&|_, _| OpResult::Input, 1, Some(0)),
        4 => Opcode::new(&|_, _| OpResult::Output, 1, Some(0)),
        5 => Opcode::new(&|v, _| if v[0] != 0 { OpResult::Jump(v[1]) } else { OpResult::None }, 2, None),
        6 => Opcode::new(&|v, _| if v[0] == 0 { OpResult::Jump(v[1]) } else { OpResult::None }, 2, None),
        7 => Opcode::new(&|v, _| OpResult::Write(if v[0] < v[1] { 1 } else { 0 }), 3, Some(2)),
        8 => Opcode::new(&|v, _| OpResult::Write(if v[0] == v[1] { 1 } else { 0 }), 3, Some(2)),
        9 => Opcode::new(&|v, _| OpResult::SetRelBase(v[0]), 1, None),
        99 => Opcode::new(&|_, _| OpResult::Halt, 0, None),
    }
}

type OpFn = dyn Fn(&Vec<isize>, &mut HashMap<usize, isize>) -> OpResult;
type Memory = HashMap<usize, isize>;

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
    None,
    Write(isize),
    Input,
    Output,
    Jump(isize),
    SetRelBase(isize),
    Halt,
}
