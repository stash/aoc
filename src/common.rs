use anyhow::anyhow;
use enum_iterator::Sequence;
use num::{Num, Zero};
use std::{
    fmt::Display,
    iter,
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
    pub fn one() -> Self {
        Self {
            x: T::one(),
            y: T::one(),
        }
    }

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

    pub fn generator(self) -> iter::FromFn<impl FnMut() -> Option<Self>> {
        let mut p = if self.x <= T::zero() || self.y <= T::zero() {
            self
        } else {
            Self::default()
        };
        std::iter::from_fn(move || {
            if p == self {
                return None;
            }

            let out = p.clone();

            if p.x < self.x {
                p.x = p.x + T::one();
            }
            // fallthru from increment above:
            if p.x == self.x {
                p.y = p.y + T::one();
                if p.y < self.y {
                    p.x = T::zero();
                } else {
                    // terminal condition p.x == self.x && p.y == self.y
                }
            }
            Some(out)
        })
    }
}

impl<T> Pos<T>
where
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Copy + Num + Display + PartialOrd,
{
    pub fn manhattan(&self, other: &Self) -> T {
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        return x + y;
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

pub fn graphrs_anyhow(err: graphrs::Error) -> anyhow::Error {
    anyhow!("graphrs: {}", err)
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use super::*;

    #[test]
    fn test_pos_generator_unsigned() -> Result<()> {
        let p: Pos<usize> = Pos { x: 2, y: 3 };
        let expect: Vec<Pos<usize>> = (0..3_usize)
            .map(|y| (0..2_usize).map(move |x| Pos { x, y }))
            .flatten()
            .collect();
        let generated: Vec<Pos<usize>> = p.generator().collect();
        assert_eq!(generated, expect);
        Ok(())
    }

    #[test]
    fn test_pos_generator_usize_zeroes() -> Result<()> {
        let x0: Pos<usize> = Pos { x: 0, y: 3 };
        let y0: Pos<usize> = Pos { x: 2, y: 0 };
        let b0: Pos<usize> = Pos { x: 0, y: 0 };
        let expect = vec![];
        for p in [x0, y0, b0] {
            let generated: Vec<Pos<usize>> = p.generator().collect();
            assert_eq!(generated, expect);
        }
        Ok(())
    }

    #[test]
    fn test_pos_generator_isize_under() -> Result<()> {
        let p: Pos<isize> = Pos { x: 2, y: -3 };
        let expect = vec![];
        let generated: Vec<Pos<isize>> = p.generator().collect();
        assert_eq!(generated, expect);
        Ok(())
    }

    #[test]
    fn test_pos_generator_isize_okay() -> Result<()> {
        let p: Pos<isize> = Pos { x: 2, y: 3 };
        let expect: Vec<Pos<isize>> = (0..3_isize)
            .map(|y| (0..2_isize).map(move |x| Pos { x, y }))
            .flatten()
            .collect();
        let generated: Vec<Pos<isize>> = p.generator().collect();
        assert_eq!(generated, expect);
        Ok(())
    }

    #[test]
    fn test_pos_generator_u8_maxrow() -> Result<()> {
        let p: Pos<u8> = Pos { x: 255, y: 1 };
        let expect: Vec<Pos<u8>> = (0u8..=254u8).map(|x| Pos { x, y: 0 }).collect();
        let generated: Vec<Pos<u8>> = p.generator().collect();
        assert_eq!(generated, expect);
        Ok(())
    }

    #[test]
    fn test_manhattan() -> Result<()> {
        let p1: Pos<usize> = Pos { x: 1, y: 3 };
        let p2: Pos<usize> = Pos { x: 3, y: 7 };
        let expect = 6;
        assert_eq!(p1.manhattan(&p2), expect);
        assert_eq!(p2.manhattan(&p1), expect);
        Ok(())
    }
}
