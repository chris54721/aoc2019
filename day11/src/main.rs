#[macro_use]
extern crate maplit;

use std::fs;
use std::collections::HashMap;

fn main() {
    let input: Memory = fs::read_to_string("./day11/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).enumerate().collect();
    println!("Part 1: {}", paint(&input, 0).len());
    println!("Part 2:\n");
    draw_grid(&paint(&input, 1), 50);
}

fn paint(input: &Memory, starting_tile: isize) -> Grid<isize> {
    let mut grid: Grid<isize> = Grid::new();
    let mut pos = Pos(0, 0);
    let mut facing = Facing::Up;
    let mut vm = VM::new(&input);
    let mut painted = false;
    grid.insert(pos, starting_tile);
    loop {
        let curr_tile = grid.get(&pos).unwrap_or(&0);
        match vm.run(vec![*curr_tile]) {
            ReturnState::WaitingForInput => continue,
            ReturnState::Output(out) => {
                if !painted {
                    grid.insert(pos, out);
                } else {
                    match out {
                        0 => facing = facing.left(),
                        1 => facing = facing.right(),
                        x => unreachable!(x)
                    };
                    let dir = facing.to_dir();
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                }
                painted = !painted;
            }
            ReturnState::Halted => break
        };
    }
    grid
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

    fn run(&mut self, in_values: Vec<isize>) -> ReturnState {
        let opcodes = get_opcodes();
        let mut in_iter = in_values.iter();
        loop {
            let op: &Opcode = opcodes.get(&(self.memory[&self.i] % 100)).unwrap();
            let params = self.fetch_params(&op);
//            println!("{:?}", (&op.apply)(&params, &mut self.memory));
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
type Memory = HashMap<usize, isize>;

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

enum ReturnState {
    WaitingForInput,
    Output(isize),
    Halted,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos(isize, isize);

type Dir = (isize, isize);

type Grid<T> = HashMap<Pos, T>;

#[derive(Debug)]
enum Facing {
    Left,
    Up,
    Right,
    Down,
}

impl Facing {
    fn to_dir(&self) -> Dir {
        match self {
            Facing::Left => (-1, 0),
            Facing::Up => (0, 1),
            Facing::Right => (1, 0),
            Facing::Down => (0, -1)
        }
    }

    fn left(&self) -> Facing {
        match self {
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right
        }
    }

    fn right(&self) -> Facing {
        match self {
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left
        }
    }
}

fn draw_grid(grid: &Grid<isize>, k: isize) {
    for x in -k..=k {
        for y in -k..=k {
            let curr_tile = grid.get(&Pos(x, y)).unwrap_or(&0);
            print!("{}", if *curr_tile == 1 { "#" } else { "." });
        }
        println!();
    }
}