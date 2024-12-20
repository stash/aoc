use core::f64;
use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use graphrs::{
    algorithms::shortest_path::{dijkstra, ShortestPathInfo},
    Edge, Graph, GraphSpecs,
};

use crate::common::{graphrs_anyhow, Dir, Pos};
type Point = Pos<usize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Debug, Clone)]
struct Map {
    bounds: Point,
    tiles: Vec<Vec<Tile>>,
    open: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Map {
    fn get(&self, p: Point) -> Tile {
        self.tiles[p.y][p.x]
    }

    fn in_main_bounds(&self, p: &Point) -> bool {
        p.x > 0 && p.x < (self.bounds.x - 1) && p.y > 0 && p.y < (self.bounds.y - 1)
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let mut start = Point::default();
    let mut end = Point::default();
    let mut tiles = vec![];
    let mut open = HashSet::new();
    for (y, line_str) in lines.into_iter().enumerate() {
        let mut row = vec![];
        for (x, c) in line_str.chars().enumerate() {
            let p = Point { x, y };
            row.push(match c {
                '#' => Tile::Wall,
                '.' => {
                    open.insert(p);
                    Tile::Empty
                }
                'S' => {
                    open.insert(p);
                    start = p;
                    Tile::Start
                }
                'E' => {
                    open.insert(p);
                    end = p;
                    Tile::End
                }
                _ => bail!("invalid tile {}", c),
            });
        }
        tiles.push(row)
    }
    let bounds = Point {
        x: tiles.first().unwrap().len(),
        y: tiles.len(),
    };
    Ok(Map {
        tiles,
        open,
        bounds,
        start,
        end,
    })
}

type G = Graph<Point, ()>;

fn compose_graph(m: &Map) -> Result<G> {
    let mut g: G = Graph::new(GraphSpecs {
        directed: false,
        edge_dedupe_strategy: graphrs::EdgeDedupeStrategy::KeepFirst,
        missing_node_strategy: graphrs::MissingNodeStrategy::Create,
        multi_edges: false,
        self_loops: false,
        self_loops_false_strategy: graphrs::SelfLoopsFalseStrategy::Error,
    });
    for u in m.open.iter() {
        for dir in enum_iterator::all::<Dir>() {
            let v = u.go(dir);
            match m.get(v) {
                Tile::Empty | Tile::End | Tile::Start => {
                    g.add_edge(Edge::with_weight(*u, v, 1.))
                        .map_err(graphrs_anyhow)?;
                }
                _ => {}
            }
        }
    }
    Ok(g)
}

fn calc_all_sps(m: &Map) -> Result<(HashMap<Point, HashMap<Point, ShortestPathInfo<Point>>>, f64)> {
    let g = compose_graph(m)?;
    let all_sps =
        dijkstra::all_pairs(&g, true, None, None, false, false).map_err(graphrs_anyhow)?;
    let main_time = {
        if let Some(from_start) = all_sps.get(&m.start) {
            if let Some(end) = from_start.get(&m.end) {
                end.distance
            } else {
                bail!("start not connected??")
            }
        } else {
            bail!("start not connected??")
        }
    };
    Ok((all_sps, main_time))
}

pub fn part1_inner(lines: Vec<String>, cutoff: f64) -> Result<String> {
    let mut total = 0;
    let m = parse(lines)?;
    let (all_sps, main_time) = calc_all_sps(&m)?;

    let mut visited: HashSet<Point> = HashSet::new();
    // visit all empty spaces and see if there's a shortcut
    for p0 in m.open.iter() {
        if !visited.insert(*p0) {
            continue;
        }
        for dir in enum_iterator::all::<Dir>() {
            let p1 = p0.go(dir);
            if !m.in_main_bounds(&p1) {
                continue;
            }
            let p2 = p1.go(dir);
            if visited.contains(&p1) {
                // U-shaped or headed "backwards"
                continue;
            }
            match (m.get(p1), m.get(p2)) {
                (Tile::Wall, Tile::Empty | Tile::End | Tile::Start) => {
                    // can knock out s1
                    let start_p0 = all_sps.get(&m.start).unwrap().get(&p0).unwrap().distance;
                    let p2_end = all_sps.get(&p2).unwrap().get(&m.end).unwrap().distance;
                    let time = start_p0 + 2. + p2_end;
                    if main_time - time >= cutoff {
                        // println!(
                        //     "wallhack: {}[{}]->{}->{}[{}] {}",
                        //     p0,
                        //     start_p0,
                        //     p1,
                        //     p2,
                        //     p2_end,
                        //     (main_time - time)
                        // );
                        total += 1;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(total.to_string())
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    part1_inner(lines, 100.)
}

const MAX_HACK: usize = 20;

pub fn part2_inner(lines: Vec<String>, cutoff: f64) -> Result<String> {
    let mut total = 0;
    let m = parse(lines)?;
    let (all_sps, main_time) = calc_all_sps(&m)?;
    let mut visited: HashSet<Point> = HashSet::new();
    // visit all empty spaces and see if there's a shortcut
    for p0 in m.open.iter() {
        if !visited.insert(*p0) {
            continue;
        }
        for p1 in m.open.iter() {
            let d = p0.manhattan(&p1);
            if d == 0 || MAX_HACK < d {
                continue;
            }
            match m.get(*p1) {
                Tile::Empty | Tile::End | Tile::Start => {
                    let start_p0 = all_sps.get(&m.start).unwrap().get(p0).unwrap().distance;
                    let p1_end = all_sps.get(p1).unwrap().get(&m.end).unwrap().distance;
                    let time = start_p0 + (d as f64) + p1_end;
                    if main_time - time >= cutoff {
                        // println!(
                        //     "cheat: {}[{}]->{}[{}] d{}, {}",
                        //     p0,
                        //     start_p0,
                        //     p1,
                        //     p1_end,
                        //     p0.manhattan(p1),
                        //     (main_time - time)
                        // );
                        total += 1;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    part2_inner(lines, 100.)
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
    fn test_part1_ezer() -> Result<()> {
        let lines = lines(indoc! {"
            #####
            #...#
            #.#.#
            #S#E#
            #####
        "});
        assert_eq!(part1_inner(lines, 1.)?, (2).to_string());
        Ok(())
    }

    #[test]
    fn test_part1_ez() -> Result<()> {
        let lines = lines(indoc! {"
            #########
            #.......#
            #.#.###.#
            #S#.#E..#
            #########
        "});
        assert_eq!(part1_inner(lines, 1.)?, (3).to_string());
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        "});
        assert_eq!(
            part1_inner(lines.clone(), 1.)?,
            (14 + 14 + 2 + 4 + 2 + 3 + 5).to_string()
        );
        assert_eq!(part1_inner(lines, 64.)?, "1");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let lines = lines(indoc! {"
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
        "});
        assert_eq!(
            part2_inner(lines.clone(), 50.)?,
            (32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3).to_string()
        );
        assert_eq!(part2_inner(lines.clone(), 72.)?, "29");
        assert_eq!(part2_inner(lines.clone(), 74.)?, "7");
        assert_eq!(part2_inner(lines.clone(), 76.)?, "3");
        Ok(())
    }
}
