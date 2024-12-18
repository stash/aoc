use std::fmt::{Display, Formatter, Write};

use anyhow::{anyhow, bail, Result};
use graphrs::{algorithms::shortest_path::dijkstra, Edge, Graph, GraphSpecs};
use itertools::Itertools;

use crate::common::{graphrs_anyhow, Dir, Pos};
type Point = Pos<usize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    Wall,
    Mark,
}

#[derive(Debug, Clone)]
struct Map {
    bounds: Point,
    tiles: Vec<Vec<Tile>>,
    seq: Vec<Point>,
}

impl Map {
    fn get(&self, p: Point) -> Tile {
        self.tiles[p.y][p.x]
    }

    fn set(&mut self, p: Point, tile: Tile) {
        self.tiles[p.y][p.x] = tile;
    }

    fn go_bounded(&self, p: &Point, dir: Dir) -> Option<Point> {
        p.go_bounded(dir, &self.bounds)
    }

    fn simulate_n(&mut self, n: usize) -> Result<()> {
        let points = self.seq.clone().into_iter().take(n);
        for p in points {
            self.set(p, Tile::Wall);
        }
        Ok(())
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("[\n")?;
        for row in self.tiles.iter() {
            for tile in row.iter() {
                let c = match tile {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Mark => 'O',
                };
                f.write_char(c)?;
            }
            f.write_str("\n")?;
        }
        f.write_str("]")?;
        Ok(())
    }
}

fn parse(lines: Vec<String>, bounds: Point) -> Result<Map> {
    let seq = lines
        .into_iter()
        .map(|line| {
            if let Some((x, y)) = line.split_once(',') {
                Ok(Point {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            } else {
                bail!("can't split");
            }
        })
        .try_collect()?;
    let tiles = std::iter::repeat_with(|| vec![Tile::Empty; bounds.x])
        .take(bounds.y)
        .collect();
    Ok(Map { bounds, tiles, seq })
}

fn part1_inner(lines: Vec<String>, bounds: Point, n: usize) -> Result<usize> {
    let mut m = parse(lines, bounds)?;
    m.simulate_n(n)?;

    let start = Point::default();
    let end = bounds - Point::one();

    let mut g: Graph<Point, ()> = Graph::new(GraphSpecs::directed_create_missing());
    for u in m.bounds.clone().generator() {
        for dir in enum_iterator::all::<Dir>() {
            if u == end {
                continue;
            }
            if let Some(v) = m.go_bounded(&u, dir) {
                if v == start {
                    continue;
                }
                match m.get(v) {
                    Tile::Empty => {
                        g.add_edge(Edge::with_weight(u, v, 1.))
                            .map_err(graphrs_anyhow)?;
                    }
                    _ => {}
                }
            }
        }
    }

    let sp = dijkstra::single_source(&g, true, Point { x: 0, y: 0 }, Some(end), None, true, true)
        .map_err(graphrs_anyhow)?;
    if let Some(info) = sp.get(&end) {
        println!("from {} = {}", start, info.distance);
        let first = info.paths.first().unwrap();
        for f in first {
            m.set(*f, Tile::Mark);
        }
        // println!("{:?}", first);
        println!("{}", m);
        Ok(first.len() - 1) // -1 for steps, not nodes
    } else {
        bail!("no path!")
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let bounds = Point { x: 71, y: 71 };
    let total = part1_inner(lines, bounds, 1024)?;
    Ok(total.to_string())
}

pub fn part2(_lines: Vec<String>) -> Result<String> {
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
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0
        "});
        let steps = part1_inner(lines, Point { x: 7, y: 7 }, 12)?;
        assert_eq!(steps, 22);
        Ok(())
    }
}
