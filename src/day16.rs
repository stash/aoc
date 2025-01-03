use std::collections::HashSet;

use anyhow::{bail, Result};
use graphrs::{algorithms::shortest_path::dijkstra, Graph, GraphSpecs};

use crate::common::{graphrs_anyhow, Dir, Pos};
type Point = Pos<usize>;

#[derive(Debug, Clone)]
struct Map {
    spaces: HashSet<Point>,
    start: Point,
    end: Point,
}

impl Map {
    fn has(&self, p: &Point) -> bool {
        self.spaces.contains(&p)
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let mut spaces = HashSet::new();
    let mut start = Point::default();
    let mut end = Point::default();
    for (y, line_str) in lines.into_iter().enumerate() {
        for (x, c) in line_str.chars().enumerate() {
            match c {
                '#' => {}
                '.' => {
                    spaces.insert(Point { x, y });
                }
                'S' => start = Point { x, y },
                'E' => end = Point { x, y },
                _ => bail!("invalid tile {}", c),
            }
        }
    }

    Ok(Map { spaces, start, end })
}

fn map_to_graph(map: &Map) -> Result<Graph<Point, ()>> {
    let mut q: Vec<(Point, Point, f64)> = Vec::new();
    //let e = Edge::with_weight(u, v, weight)
    let mut g = Graph::new(GraphSpecs::directed_create_missing());
    for space in map.spaces.iter() {
        let u = space.go(Dir::Up);
        let r = space.go(Dir::Right);
        let d = space.go(Dir::Down);
        let l = space.go(Dir::Left);
        let has_u = map.has(&u);
        let has_r = map.has(&r);
        let has_d = map.has(&d);
        let has_l = map.has(&l);
        // skips: forward, forward == 2
        if has_u && has_d {
            q.push((u, d, 2.));
            q.push((d, u, 2.));
        }
        if has_l && has_r {
            q.push((l, r, 2.));
            q.push((r, l, 2.));
        }
        // corners: forward, turn, forward == 92
        if has_u && has_r {
            q.push((u, r, 1002.));
            q.push((r, u, 1002.));
        }
        if has_r && has_d {
            q.push((d, r, 1002.));
            q.push((r, d, 1002.));
        }
        if has_l && has_d {
            q.push((d, l, 1002.));
            q.push((l, d, 1002.));
        }
        if has_l && has_u {
            q.push((u, l, 1002.));
            q.push((l, u, 1002.));
        }
    }

    {
        // From start assuming it's in bottom left hallway'd corner
        let s_u = map.start.go(Dir::Up);
        let s_r = map.start.go(Dir::Right);
        if map.has(&s_u) {
            q.push((map.start, s_u, 1001.)); // turn left, fwd
        }
        if map.has(&s_r) {
            q.push((map.start, s_r, 1.)); // fwd
        }
    }

    {
        // To end, assuming it's always in a hallway'd corner in top right
        // (examples and personal challenge seems that way)
        let e_d = map.end.go(Dir::Down);
        if map.has(&e_d) {
            q.push((e_d, map.end, 1.));
        }
        let e_l = map.end.go(Dir::Left);
        if map.has(&e_l) {
            q.push((e_l, map.end, 1.));
        }
    }

    g.add_edge_tuples_weighted(q).map_err(graphrs_anyhow)?;

    Ok(g)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let map = parse(lines)?;
    let g = map_to_graph(&map)?;
    let sp = dijkstra::single_source(&g, true, map.start, Some(map.end), None, true, true)
        .map_err(graphrs_anyhow)?;
    let mut total: usize = 0;
    if let Some(info) = sp.get(&map.end) {
        println!("from {} = {}", map.start, info.distance);
        println!("{:?}", info.paths.first());
        total = info.distance.round() as usize;
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let map = parse(lines)?;
    let g = map_to_graph(&map)?;
    let sp = dijkstra::single_source(&g, true, map.start, Some(map.end), None, false, true)
        .map_err(graphrs_anyhow)?;

    let mut travelled: HashSet<Point> = HashSet::new();
    if let Some(info) = sp.get(&map.end) {
        println!(
            "from {} = {}, {} paths",
            map.start,
            info.distance,
            info.paths.len()
        );
        for path in info.paths.iter() {
            let mut iter = path.iter();
            let mut prev = iter.next().unwrap();
            travelled.insert(*prev);
            while let Some(next) = iter.next() {
                travelled.insert(*next);
                let dx = prev.x.abs_diff(next.x);
                let dy = prev.y.abs_diff(next.y);
                if (dx == 0 && dy == 1) || (dx == 1 && dy == 0) {
                    // already done
                } else if dx == 2 {
                    // skip horiz
                    let missing_x = (prev.x + next.x) / 2;
                    travelled.insert(Point {
                        x: missing_x,
                        y: prev.y,
                    });
                } else if dy == 2 {
                    // skip vert
                    let missing_y = (prev.y + next.y) / 2;
                    travelled.insert(Point {
                        x: prev.x,
                        y: missing_y,
                    });
                } else if dx == 1 && dy == 1 {
                    // corner
                    let cand1 = Point {
                        x: next.x,
                        y: prev.y,
                    };
                    let cand2 = Point {
                        x: prev.x,
                        y: next.y,
                    };
                    if map.has(&cand1) {
                        travelled.insert(cand1);
                    } else {
                        travelled.insert(cand2);
                    }
                }
                prev = next;
            }
        }
    }
    let total: usize = travelled.len();
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
    fn test_part1_ez() -> Result<()> {
        let lines = lines(indoc! {"
            #######
            #...#E#
            #.#.#.#
            #S#...#
            #######
        "});
        assert_eq!(
            part1(lines)?,
            (1001 + 1002 + 1002 + 1002 + 1002 + 1).to_string()
        );
        Ok(())
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let lines = lines(indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "});
        assert_eq!(part1(lines)?, "7036");
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let lines = lines(indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        "});
        assert_eq!(part1(lines)?, "11048");
        Ok(())
    }

    #[test]
    fn test_part2_a() -> Result<()> {
        let lines = lines(indoc! {"
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
        "});
        assert_eq!(part2(lines)?, "45");
        Ok(())
    }

    #[test]
    fn test_part2_b() -> Result<()> {
        let lines = lines(indoc! {"
            #################
            #...#...#...#..E#
            #.#.#.#.#.#.#.#.#
            #.#.#.#...#...#.#
            #.#.#.#.###.#.#.#
            #...#.#.#.....#.#
            #.#.#.#.#.#####.#
            #.#...#.#.#.....#
            #.#.#####.#.###.#
            #.#.#.......#...#
            #.#.###.#####.###
            #.#.#...#.....#.#
            #.#.#.#####.###.#
            #.#.#.........#.#
            #.#.#.#########.#
            #S#.............#
            #################
        "});
        assert_eq!(part2(lines)?, "64");
        Ok(())
    }
}
