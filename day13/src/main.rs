use std::fs;
use utils::intcode::{VM, ReturnState, Memory};
use utils::Grid;
use crate::Tile::{Empty, Wall, Block, Paddle, Ball};
use std::slice::Iter;
use crate::DrawState::{X, Y, TileId};
use std::collections::VecDeque;
use utils::vec2d::Vec2d;

fn main() {
    let mut input: Memory = fs::read_to_string("./day13/input/input.txt").unwrap()
        .split(',').map(|x| x.parse().unwrap()).enumerate().collect();
    let mut grid1: Grid<Tile> = Grid::new();
    run(&input, &mut grid1);
    println!("Part 1: {}", grid1.values().filter(|&t| *t == Tile::Block).count());
    input.insert(0, 2);
    println!("Part 2: {}", run(&input, &mut Grid::new()));
}

fn run(input: &Memory, grid: &mut Grid<Tile>) -> isize {
    let mut vm = VM::new(&input);
    let mut state = DrawState::iter().cycle();
    let mut draw_pos = Vec2d::new(0, 0);
    let mut score = 0;
    let mut next_input = VecDeque::new();
    loop {
        let mut in_values = vec![];
        if let Some(i) = next_input.pop_back() {
            in_values.push(i);
        }
        match vm.run(in_values) {
            ReturnState::WaitingForInput => {
                let paddle_pos = *find_tile(&grid, Tile::Paddle).unwrap().0;
                let ball_pos = *find_tile(&grid, Tile::Ball).unwrap().0;
                let x_diff = ball_pos.x - paddle_pos.x;
                for _ in 0..x_diff.abs() {
                    next_input.push_back(x_diff.signum());
                }
                if next_input.is_empty() {
                    next_input.push_back(0);
                }
            }
            ReturnState::Output(out) => {
                match state.next().unwrap() {
                    DrawState::X => draw_pos.x = out,
                    DrawState::Y => draw_pos.y = out,
                    DrawState::TileId => {
                        if draw_pos.x == -1 && draw_pos.y == 0 {
                            score = out;
                        } else {
                            grid.insert(draw_pos, Tile::from_id(out));
                        }
                    }
                }
            }
            ReturnState::Halted => break
        }
    }
    score
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn from_id(id: isize) -> Tile {
        match id {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Debug)]
enum DrawState {
    X,
    Y,
    TileId,
}

impl DrawState {
    fn iter() -> Iter<'static, DrawState> {
        static STATES: [DrawState; 3] = [X, Y, TileId];
        STATES.iter()
    }
}

fn draw_grid(grid: &Grid<Tile>) {
    for y in 0..=25 {
        for x in 0..=200 {
            if let Some(t) = grid.get(&Vec2d::new(x, y)) {
                print!("{}", match t {
                    Empty => " ",
                    Wall => "#",
                    Block => "X",
                    Ball => "O",
                    Paddle => "_"
                })
            }
        }
        println!()
    }
    ::std::thread::sleep(::std::time::Duration::from_millis(1000));
}

fn find_tile(grid: &Grid<Tile>, tile: Tile) -> Option<(&Vec2d, &Tile)> {
    grid.iter().find(|(_, t)| **t == tile)
}