use anyhow::Result;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Obst,
    Visited,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
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

#[derive(Clone)] // Allow reality-forking
struct Map {
    bound: Pos,
    dir: Dir,
    guard: Pos,
    grid: Vec<Vec<Tile>>,
}

impl Map {
    fn new(lines: Vec<String>) -> Self {
        let h = lines.len();
        let w = lines.first().expect("non-zero width").len();
        let mut guard = Pos { x: 0, y: 0 };
        let mut dir = Dir::Up;
        let mut grid: Vec<Vec<Tile>> = Vec::new();
        for y in 0..h {
            let line = &lines[y];
            let mut row: Vec<Tile> = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obst,
                    '^' => {
                        guard = Pos { x, y };
                        dir = Dir::Up;
                        Tile::Visited
                    }
                    // unsure if guard can be any of these initially, my challenge has it facing up:
                    '<' => {
                        guard = Pos { x, y };
                        dir = Dir::Left;
                        Tile::Visited
                    }
                    '>' => {
                        guard = Pos { x, y };
                        dir = Dir::Right;
                        Tile::Visited
                    }
                    'v' => {
                        guard = Pos { x, y };
                        dir = Dir::Down;
                        Tile::Visited
                    }
                    _ => {
                        panic!("illegal char")
                    }
                };
                row.push(tile);
            }
            grid.push(row);
        }
        Self {
            bound: Pos { x: w, y: h },
            dir,
            guard,
            grid,
        }
    }

    pub fn get_mut(&mut self, p: Pos) -> Option<&mut Tile> {
        if p.x > self.bound.x || p.y > self.bound.y {
            None
        } else {
            Some(&mut self.grid[p.y][p.x])
        }
    }

    pub fn next_move(&self) -> Option<Pos> {
        match self.dir {
            Dir::Up => {
                if self.guard.y == 0 {
                    return None;
                }
                Some(Pos {
                    x: self.guard.x,
                    y: self.guard.y - 1,
                })
            }
            Dir::Down => {
                if self.guard.y == self.bound.y - 1 {
                    return None;
                }
                Some(Pos {
                    x: self.guard.x,
                    y: self.guard.y + 1,
                })
            }
            Dir::Left => {
                if self.guard.x == 0 {
                    return None;
                }
                Some(Pos {
                    x: self.guard.x - 1,
                    y: self.guard.y,
                })
            }
            Dir::Right => {
                if self.guard.x == self.bound.x - 1 {
                    return None;
                }
                Some(Pos {
                    x: self.guard.x + 1,
                    y: self.guard.y,
                })
            }
        }
    }

    pub fn simulate(&mut self) -> bool {
        let next_pos = match self.next_move() {
            Some(p) => p,
            None => return false,
        };
        if let Some(tile) = self.get_mut(next_pos) {
            match *tile {
                Tile::Empty | Tile::Visited => {
                    *tile = Tile::Visited;
                    self.guard = next_pos;
                }
                Tile::Obst => {
                    self.dir = self.dir.cw();
                }
            }
            return true;
        } else {
            return false;
        }
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = Map::new(lines);
    while map.simulate() {
        print!(".")
    }
    let total: usize = map
        .grid
        .into_iter()
        .map(|row| {
            row.into_iter().fold(0, |acc, tile| {
                acc + match tile {
                    Tile::Visited => 1,
                    _ => 0,
                }
            })
        })
        .sum();
    Ok(total.to_string())
}

fn is_loopable(reality: &Map) -> bool {
    if let Some(next_pos) = reality.next_move() {
        let mut alt = reality.clone();
        if let Some(tile) = alt.get_mut(next_pos) {
            // warp in obstruction in front of guard iff Empty
            match *tile {
                Tile::Empty => {
                    *tile = Tile::Obst;
                }
                _ => return false,
            }
        } else {
            return false;
        };

        let mut been_there = HashSet::new();
        loop {
            if !been_there.insert((alt.guard, alt.dir)) {
                // guard returned to the same position, facing the same direction
                return true;
            }
            if !alt.simulate() {
                return false; // guard exited
            }
        }
    }
    false
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut reality = Map::new(lines);
    let mut total: usize = 0;
    loop {
        // Check if obstructing the guard at this point in time would result in a loop:
        if is_loopable(&reality) {
            print!("@");
            total += 1;
        } else {
            print!(".");
        }
        // Carry on with the illusion of a linear reality:
        if !reality.simulate() {
            break; // guard exited
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input: Vec<String> = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part1(input)?, "41");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input: Vec<String> = indoc! {"
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
        "}
        .lines()
        .map(|x| x.to_string())
        .collect();

        assert_eq!(part2(input)?, "6");
        Ok(())
    }
}
