use enum_iterator::Sequence;
use num::{Num, Zero};
use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos<T>
where
    T: Copy + Num + Display,
{
    pub x: T,
    pub y: T,
}

impl<T> Default for Pos<T>
where
    T: Copy + Num + Display + Zero,
{
    fn default() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

impl<T, U> TryFrom<(U, U)> for Pos<T>
where
    T: Copy + Num + Display + TryFrom<U>,
    U: TryFrom<T>,
{
    type Error = <T as TryFrom<U>>::Error;

    fn try_from(value: (U, U)) -> std::result::Result<Self, Self::Error> {
        let x = T::try_from(value.0)?;
        let y = T::try_from(value.1)?;
        Ok(Self { x, y })
    }
}

impl<T> Pos<T>
where
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Copy + Num + Display + PartialOrd + PartialEq,
{
    pub fn go(&self, dir: Dir) -> Self {
        let one = T::one();
        match dir {
            Dir::Up => Self {
                x: self.x,
                y: self.y - one,
            },
            Dir::Left => Self {
                x: self.x - one,
                y: self.y,
            },
            Dir::Down => Self {
                x: self.x,
                y: self.y + one,
            },
            Dir::Right => Self {
                x: self.x + one,
                y: self.y,
            },
        }
    }

    pub fn go_bounded(&self, dir: Dir, bounds: &Self) -> Option<Self> {
        let one = T::one();
        let zero = T::zero();
        match dir {
            Dir::Up => {
                if self.y > zero {
                    Some(Self {
                        x: self.x,
                        y: self.y - one,
                    })
                } else {
                    None
                }
            }
            Dir::Left => {
                if self.x > zero {
                    Some(Self {
                        x: self.x - one,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Dir::Down => {
                if self.y < (bounds.y - one) {
                    Some(Self {
                        x: self.x,
                        y: self.y + one,
                    })
                } else {
                    None
                }
            }
            Dir::Right => {
                if self.x < (bounds.x - one) {
                    Some(Self {
                        x: self.x + one,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn in_bounds(&self, bounds: &Self) -> bool {
        self.x >= T::zero() && self.x < bounds.x && self.y >= T::zero() && self.y < bounds.y
    }
}

impl<T> Sub for Pos<T>
where
    T: Sub<T, Output = T>,
    T: Copy + Num + Display,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Add for Pos<T>
where
    T: Add<T, Output = T>,
    T: Copy + Num,
    T: Copy + Num + Display,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T, V> Mul<V> for Pos<T>
where
    T: Mul<V, Output = T>,
    V: Copy + Num,
    T: Copy + Num + Display,
{
    type Output = Self;
    fn mul(self, rhs: V) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Display for Pos<T>
where
    T: Copy + Num + Display,
{
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
    // pub fn cw(&self) -> Self {
    //     match self {
    //         Self::Up => Self::Right,
    //         Self::Right => Self::Down,
    //         Self::Down => Self::Left,
    //         Self::Left => Self::Up,
    //     }
    // }

    /// Orthogonal directions, CCW then CW
    pub fn orthos(&self) -> (Self, Self) {
        match self {
            Dir::Up => (Dir::Left, Dir::Right),
            Dir::Left => (Dir::Down, Dir::Up),
            Dir::Down => (Dir::Right, Dir::Left),
            Dir::Right => (Dir::Up, Dir::Down),
        }
    }
}
