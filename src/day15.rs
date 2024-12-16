use std::fmt::{Display, Write};

use anyhow::{anyhow, bail, Result};

use crate::common::{Dir, Pos};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    Box,
    Bot,
    Wall,
}

#[derive(Debug, Clone)]
struct Map {
    bot: Pos,
    bounds: Pos,
    tiles: Vec<Vec<Tile>>,
    moves: Vec<Dir>,
}

impl Map {
    fn try_move(&mut self, p: Pos, dir: Dir) -> Result<bool> {
        let new_p = p.go(dir);

        let tile = self.tiles[p.y as usize][p.x as usize];
        match tile {
            Tile::Wall => Ok(false),
            Tile::Empty => Ok(true),
            Tile::Box => {
                let move_ok = self.try_move(new_p, dir)?;
                if move_ok {
                    self.tiles[new_p.y as usize][new_p.x as usize] = tile;
                }
                Ok(move_ok)
            }
            Tile::Bot => {
                let move_ok = self.try_move(new_p, dir)?;
                if move_ok {
                    self.tiles[new_p.y as usize][new_p.x as usize] = tile;
                    self.bot = new_p;
                }
                Ok(move_ok)
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
                };
                f.write_char(c)?;
            }
            f.write_str("\n")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let mut iter = lines.into_iter().enumerate();
    let (mut y, mut line_str) = iter.next().ok_or_else(|| anyhow!("missing line"))?;
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut moves = Vec::new();
    let mut bot = Pos::new(0, 0)?;
    loop {
        let mut row = Vec::new();
        for (x, c) in line_str.chars().enumerate() {
            match c {
                '#' => row.push(Tile::Wall),
                'O' => row.push(Tile::Box),
                '.' => row.push(Tile::Empty),
                '@' => {
                    row.push(Tile::Bot);
                    bot = Pos::new(x, y)?
                }
                _ => bail!("invalid tile {}", c),
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

    let bounds = Pos::new(tiles.first().unwrap().len(), tiles.len())?;
    Ok(Map {
        bot,
        bounds,
        tiles,
        moves,
    })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;
    println!("bot: {:?}, bound: {:?}", map.bot, map.bounds);
    println!("map: {}", map);
    for dir in map.moves.clone() {
        let p = map.bot;
        if map.try_move(p, dir)? {
            map.tiles[p.y as usize][p.x as usize] = Tile::Empty;
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
    let mut map = parse(lines)?;
    bail!("not done")
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
}
