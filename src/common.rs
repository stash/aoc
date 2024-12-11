use anyhow::Result;
use enum_iterator::Sequence;
use std::{fmt::Display, ops::{Add,Sub}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub fn new(x:usize, y:usize) -> Result<Self>{
        Ok(Self {
            x: x.try_into()?,
            y: y.try_into()?,
        })
    }

    pub fn go(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Self {x:self.x, y:self.y-1},
            Dir::Left => Self {x:self.x-1, y:self.y},
            Dir::Down => Self {x:self.x, y:self.y+1},
            Dir::Right => Self {x:self.x+1, y:self.y},
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum Dir {
    Up,
    Left,
    Down,
    Right,
}
impl Dir {
    pub fn cw(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}