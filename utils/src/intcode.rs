use std::collections::HashMap;

pub struct VM {
    i: usize,
    memory: Memory,
    rel_base: usize,
}

impl VM {
    pub fn new(input: &Memory) -> VM {
        VM { i: 0, memory: input.clone(), rel_base: 0 }
    }

    pub fn run(&mut self, in_values: Vec<isize>) -> ReturnState {
        let opcodes = get_opcodes();
        let mut in_iter = in_values.iter();
        loop {
            let op: &Opcode = opcodes.get(&(self.memory[&self.i] % 100)).unwrap();
            let params = self.fetch_params(&op);
            match (&op.apply)(&params, &mut self.memory) {
                OpResult::None => {}
                OpResult::Input => {
                    let next = in_iter.next();
                    match next {
                        None => return ReturnState::WaitingForInput,
                        Some(&i) => self.memory.insert(params[0] as usize, i)
                    };
                }
                OpResult::Output => {
                    self.i += op.param_count + 1;
                    return ReturnState::Output(params[0]);
                }
                OpResult::Write(v) => {
                    self.memory.insert(params[op.addr_param.unwrap()] as usize, v);
                }
                OpResult::Jump(d) => {
                    self.i = d as usize;
                    continue;
                }
                OpResult::SetRelBase(r) => {
                    self.rel_base = ((self.rel_base as isize) + r) as usize;
                }
                OpResult::Halt => return ReturnState::Halted
            }
            self.i += op.param_count + 1;
        }
    }

    fn fetch_params(&self, op: &Opcode) -> Vec<isize> {
        let mut params: Vec<isize> = Vec::with_capacity(op.param_count);
        let op_full = self.mem_uread(self.i);
        for i in 0..op.param_count {
            let p = self.mem_read(self.i + i + 1);
            let m_div = 10usize.pow((2 + i) as u32);
            let is_addr_param = op.addr_param.is_some() && i == op.addr_param.unwrap();
            let mode = if op_full >= m_div {
                op_full / m_div % 10
            } else if is_addr_param { 1 } else { 0 };
            params.push(match mode {
                0 => self.mem_read(p as usize),
                1 => p,
                2 => {
                    let addr = (self.rel_base as isize + p) as usize;
                    if is_addr_param { addr as isize } else { self.mem_read(addr) }
                }
                _ => unreachable!()
            });
        }
        params
    }

    fn mem_read(&self, addr: usize) -> isize {
        *self.memory.get(&addr).unwrap_or(&0)
    }

    fn mem_uread(&self, addr: usize) -> usize {
        self.mem_read(addr) as usize
    }
}

fn get_opcodes() -> HashMap<isize, Opcode> {
    hashmap! {
        1 => Opcode::new(&|v, _| OpResult::Write(v[0] + v[1]), 3, Some(2)),
        2 => Opcode::new(&|v, _| OpResult::Write(v[0] * v[1]), 3, Some(2)),
        3 => Opcode::new(&|_, _| OpResult::Input, 1, Some(0)),
        4 => Opcode::new(&|_, _| OpResult::Output, 1, None),
        5 => Opcode::new(&|v, _| if v[0] != 0 { OpResult::Jump(v[1]) } else { OpResult::None }, 2, None),
        6 => Opcode::new(&|v, _| if v[0] == 0 { OpResult::Jump(v[1]) } else { OpResult::None }, 2, None),
        7 => Opcode::new(&|v, _| OpResult::Write(if v[0] < v[1] { 1 } else { 0 }), 3, Some(2)),
        8 => Opcode::new(&|v, _| OpResult::Write(if v[0] == v[1] { 1 } else { 0 }), 3, Some(2)),
        9 => Opcode::new(&|v, _| OpResult::SetRelBase(v[0]), 1, None),
        99 => Opcode::new(&|_, _| OpResult::Halt, 0, None),
    }
}

type OpFn = dyn Fn(&Vec<isize>, &mut HashMap<usize, isize>) -> OpResult;
pub type Memory = HashMap<usize, isize>;

struct Opcode {
    apply: &'static OpFn,
    param_count: usize,
    addr_param: Option<usize>,
}

impl Opcode {
    fn new(apply: &'static OpFn, param_count: usize, addr_param: Option<usize>) -> Opcode {
        Opcode { apply, param_count, addr_param }
    }
}

#[derive(Debug)]
enum OpResult {
    None,
    Write(isize),
    Input,
    Output,
    Jump(isize),
    SetRelBase(isize),
    Halt,
}

pub enum ReturnState {
    WaitingForInput,
    Output(isize),
    Halted,
}
