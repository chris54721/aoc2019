use std::ops::{Add, AddAssign, Neg, Index, IndexMut};
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;
use std::slice::Iter;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Vec2d {
    pub x: isize,
    pub y: isize,
}

impl Vec2d {
    pub fn new(x: isize, y: isize) -> Vec2d {
        Vec2d { x, y }
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Vec2d {
    type Output = Vec2d;

    fn neg(self) -> Self::Output {
        Vec2d::new(-self.x, -self.y)
    }
}

impl Index<usize> for Vec2d {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => unreachable!()
        }
    }
}

impl IndexMut<usize> for Vec2d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable!()
        }
    }
}

impl IntoIterator for Vec2d {
    type Item = isize;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y].into_iter()
    }
}

impl FromStr for Vec2d {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=([-\d]+), y=([-\d]+)>").unwrap();
        }
        Regex::new("").unwrap().captures(s).unwrap();
        let cap = RE.captures(s).unwrap();
        Ok(Vec2d {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
        })
    }
}