use std::fmt::{Display, Write};

use anyhow::{anyhow, bail, Result};

use crate::common::{Dir, Pos};
type Point = Pos<isize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    Box,
    WideLeft,
    WideRight,
    Bot,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    bot: Point,
    bounds: Point,
    tiles: Vec<Vec<Tile>>,
    moves: Vec<Dir>,
}

impl Map {
    fn get(&self, p: Point) -> Tile {
        self.tiles[p.y as usize][p.x as usize]
    }

    fn set(&mut self, p: Point, tile: Tile) {
        self.tiles[p.y as usize][p.x as usize] = tile;
    }

    fn peek_move(&self, p: Point, dir: Dir) -> bool {
        let p_new = p.go(dir);
        let tile = self.get(p);
        match tile {
            Tile::Wall => false,
            Tile::Empty => true,
            Tile::Box => self.peek_move(p_new, dir),
            Tile::WideLeft => match dir {
                // both this and its dual need to be able to push
                // L/R is easy: just chain as usual
                Dir::Left | Dir::Right => self.peek_move(p_new, dir),
                // U/D need to branch the recursion
                _ => {
                    let dual = p.go(Dir::Right);
                    let dual_new = dual.go(dir);
                    self.peek_move(p_new, dir) && self.peek_move(dual_new, dir)
                }
            },
            Tile::WideRight => match dir {
                Dir::Left | Dir::Right => self.peek_move(p_new, dir),
                _ => {
                    let dual = p.go(Dir::Left);
                    let dual_new = dual.go(dir);
                    self.peek_move(p_new, dir) && self.peek_move(dual_new, dir)
                }
            },
            Tile::Bot => self.peek_move(p_new, dir),
        }
    }

    fn do_move(&mut self, p: Point, dir: Dir) {
        let p_new = p.go(dir);
        let tile = self.get(p);
        match tile {
            Tile::Wall => {}
            Tile::Empty => {}
            Tile::Box => {
                self.do_move(p_new, dir);
                self.set(p_new, tile);
                self.set(p, Tile::Empty);
            }
            Tile::WideLeft => match dir {
                Dir::Left | Dir::Right => {
                    // L/R is again easy: just chain
                    self.do_move(p_new, dir);
                    self.set(p_new, tile);
                    self.set(p, Tile::Empty);
                }
                _ => {
                    // U/D: move forward each branch before the two halves
                    let dual = p.go(Dir::Right);
                    let dual_new = dual.go(dir);
                    self.do_move(p_new, dir);
                    self.do_move(dual_new, dir);
                    self.set(p_new, tile);
                    self.set(p, Tile::Empty);
                    self.set(dual_new, Tile::WideRight);
                    self.set(dual, Tile::Empty);
                }
            },
            Tile::WideRight => match dir {
                Dir::Left | Dir::Right => {
                    self.do_move(p_new, dir);
                    self.set(p_new, tile);
                    self.set(p, Tile::Empty);
                }
                _ => {
                    let dual = p.go(Dir::Left);
                    let dual_new = dual.go(dir);
                    self.do_move(p_new, dir);
                    self.do_move(dual_new, dir);
                    self.set(p_new, tile);
                    self.set(p, Tile::Empty);
                    self.set(dual_new, Tile::WideLeft);
                    self.set(dual, Tile::Empty);
                }
            },
            Tile::Bot => {
                self.do_move(p_new, dir);
                self.set(p_new, tile);
                self.set(p, Tile::Empty);
                self.bot = p_new;
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[\n")?;
        for row in self.tiles.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Bot => '@',
                    Tile::Box => 'O',
                    Tile::WideLeft => '[',
                    Tile::WideRight => ']',
                };
                f.write_char(c)?;
            }
            f.write_str("\n")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

fn parse(lines: Vec<String>, two: bool) -> Result<Map> {
    let mut iter = lines.into_iter().enumerate();
    let (mut y, mut line_str) = iter.next().ok_or_else(|| anyhow!("missing line"))?;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut moves = Vec::new();
    let mut bot = Point::default();
    loop {
        let mut row = Vec::new();
        for (x, c) in line_str.chars().enumerate() {
            match c {
                '#' => row.push(Tile::Wall),
                'O' => row.push(Tile::Box),
                '.' => row.push(Tile::Empty),
                '@' => {
                    row.push(Tile::Bot);
                    bot = (x, y).try_into()?;
                }
                _ => bail!("invalid tile {}", c),
            }
            if two {
                // embiggen:
                match c {
                    '#' => row.push(Tile::Wall),
                    'O' => {
                        _ = row.pop(); // was Box
                        row.push(Tile::WideLeft);
                        row.push(Tile::WideRight);
                    }
                    '.' => row.push(Tile::Empty),
                    '@' => {
                        row.push(Tile::Empty);
                        bot = (2 * x, y).try_into()?; // fix x coord
                    }
                    _ => bail!("invalid tile {}", c),
                }
            }
        }
        tiles.push(row);

        (y, line_str) = iter.next().ok_or_else(|| anyhow!("missing line"))?;
        if line_str == "" {
            break;
        }
    }

    while let Some((_, line_str)) = iter.next() {
        for c in line_str.chars() {
            match c {
                '<' => moves.push(Dir::Left),
                '>' => moves.push(Dir::Right),
                '^' => moves.push(Dir::Up),
                'v' => moves.push(Dir::Down),
                _ => bail!("invalid move {}", c),
            }
        }
    }

    let bounds = Point {
        x: tiles.first().unwrap().len() as isize,
        y: tiles.len() as isize,
    };
    Ok(Map {
        bot,
        bounds,
        tiles,
        moves,
    })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines, false)?;
    println!("bot: {:?}, bound: {:?}", map.bot, map.bounds);
    println!("map: {}", map);
    for dir in map.moves.clone() {
        let p = map.bot;
        if map.peek_move(p, dir) {
            map.do_move(p, dir);
        }
    }
    println!("after: {}", map);

    let mut total: usize = 0;
    for (y, row) in map.tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            if tile == Tile::Box {
                total += 100 * y + x;
            }
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines, true)?;
    println!("bot: {:?}, bound: {:?}", map.bot, map.bounds);
    println!("map: {}", map);
    for dir in map.moves.clone() {
        let p = map.bot;
        if map.peek_move(p, dir) {
            map.do_move(p, dir);
        }
        // println!("after {:?}: {}", dir, map);
    }
    println!("after: {}", map);
    let mut total: usize = 0;
    for (y, row) in map.tiles.into_iter().enumerate() {
        for (x, tile) in row.into_iter().enumerate() {
            if tile == Tile::WideLeft {
                total += 100 * y + x;
            }
        }
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let lines = lines(indoc! {"
            ########
            #..O.O.#
            ##@.O..#
            #...O..#
            #.#.O..#
            #...O..#
            #......#
            ########

            <^^>>>vv<v>>v<<
        "});
        assert_eq!(part1(lines)?, "2028");
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let lines = lines(indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "});
        assert_eq!(part1(lines)?, "10092");
        Ok(())
    }

    #[test]
    fn test_part2_a() -> Result<()> {
        let lines = lines(indoc! {"
            #######
            #...#.#
            #.....#
            #..OO@#
            #..O..#
            #.....#
            #######

            <vv<<^^<<^^
        "});
        assert_eq!(part2(lines)?, (105 + 207 + 306).to_string());
        Ok(())
    }

    #[test]
    fn test_part2_b() -> Result<()> {
        let lines = lines(indoc! {"
            ##########
            #..O..O.O#
            #......O.#
            #.OO..O.O#
            #..O@..O.#
            #O#..O...#
            #O..O..O.#
            #.OO.O.OO#
            #....O...#
            ##########

            <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
            vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
            ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
            <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
            ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
            ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
            >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
            <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
            ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
            v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "});
        assert_eq!(part2(lines)?, "9021");
        Ok(())
    }
}
