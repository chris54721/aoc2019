#[macro_use]
extern crate maplit;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use crate::vec2d::Vec2d;

pub mod vec2d;
pub mod vec3d;
pub mod intcode;

//#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
//pub struct Pos(pub isize, pub isize);

pub type Grid<T> = HashMap<Vec2d, T>;