use std::fmt::{Display, Formatter, Write};

use anyhow::{bail, Result};
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

type G = Graph<Pos<usize>, ()>;

fn add_edges_for(m: &Map, u: Point, g: &mut G, reciprocal: bool) -> Result<()> {
    Ok(for dir in enum_iterator::all::<Dir>() {
        if let Some(v) = m.go_bounded(&u, dir) {
            match m.get(v) {
                Tile::Empty => {
                    g.add_edge(Edge::with_weight(u, v, 1.))
                        .map_err(graphrs_anyhow)?;
                    if reciprocal {
                        g.add_edge(Edge::with_weight(v, u, 1.))
                            .map_err(graphrs_anyhow)?;
                    }
                }
                _ => {}
            }
        }
    })
}

fn compose_graph(m: &Map) -> Result<G> {
    let mut g: G = Graph::new(GraphSpecs {
        directed: true,
        edge_dedupe_strategy: graphrs::EdgeDedupeStrategy::KeepFirst,
        missing_node_strategy: graphrs::MissingNodeStrategy::Create,
        multi_edges: false,
        self_loops: false,
        self_loops_false_strategy: graphrs::SelfLoopsFalseStrategy::Error,
    });
    for u in m.bounds.clone().generator() {
        add_edges_for(m, u, &mut g, false)?;
    }
    Ok(g)
}

fn part1_inner(lines: Vec<String>, bounds: Point, n: usize) -> Result<usize> {
    let mut m = parse(lines, bounds)?;
    {
        let points = m.seq.clone().into_iter().take(n);
        for p in points {
            m.set(p, Tile::Wall);
        }
    }
    let start = Point::default();
    let end = bounds - Point::one();

    let g = compose_graph(&m)?;

    let sp = dijkstra::single_source(&g, true, start, Some(end), None, true, true)
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

fn part2_inner(lines: Vec<String>, bounds: Point) -> Result<Point> {
    let mut m = parse(lines, bounds)?;
    let start = Point::default();
    let end = bounds - Point::one();

    // I really wish graphrs had a "remove edge", but alas. Go in reverse!
    // Start with all points filled
    for p in m.seq.clone() {
        m.set(p, Tile::Wall);
    }

    // Connect initial empty set:
    let mut g = compose_graph(&m)?;
    // then, going in reverse, remove a wall
    for p in m.seq.clone().into_iter().rev() {
        m.set(p, Tile::Empty);
        add_edges_for(&m, p, &mut g, true)?;
        // does this open a path?
        let bfs = g.breadth_first_search(&start);
        if bfs.contains(&end) {
            // then it was this point that cut it off
            m.set(p, Tile::Mark);
            println!("{}", m);
            return Ok(p);
        }
    }
    bail!("no cut-offs?")
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let bounds = Point { x: 71, y: 71 };
    let cutoff = part2_inner(lines, bounds)?;
    Ok(format!("{},{}", cutoff.x, cutoff.y))
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

    #[test]
    fn test_part2() -> Result<()> {
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
        let steps = part2_inner(lines, Point { x: 7, y: 7 })?;
        assert_eq!(steps, Point { x: 6, y: 1 });
        Ok(())
    }
}
