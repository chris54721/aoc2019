use std::ops::{Add, AddAssign, Neg, Index, IndexMut};
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Vec3d {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vec3d {
    pub fn new(x: isize, y: isize, z: isize) -> Vec3d {
        Vec3d { x, y, z }
    }
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Vec3d {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!()
        }
    }
}

impl IndexMut<usize> for Vec3d {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable!()
        }
    }
}

impl IntoIterator for Vec3d {
    type Item = isize;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}

impl FromStr for Vec3d {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=([-\d]+), y=([-\d]+), z=([-\d]+)>").unwrap();
        }
        Regex::new("").unwrap().captures(s).unwrap();
        let cap = RE.captures(s).unwrap();
        Ok(Vec3d {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
            z: cap[3].parse()?,
        })
    }
}