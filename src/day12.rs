use std::collections::HashSet;

use crate::common::{graphrs_anyhow, Dir, Pos};
use anyhow::{anyhow, Result};
use graphrs::{algorithms::components::connected_components, Graph, GraphSpecs, Node};

type Point = Pos<usize>;

struct Map {
    plots: Vec<Vec<char>>,
    bounds: Point,
}
impl Map {
    pub fn go(&self, p: &Point, dir: Dir) -> Option<Point> {
        p.go_bounded(dir, &self.bounds)
    }
}

fn map_to_graph(map: &Map) -> Result<Graph<Point, ()>> {
    let mut g: Graph<Point, ()> = Graph::new(GraphSpecs::undirected());
    for y in 0..map.bounds.y {
        for x in 0..map.bounds.x {
            let u = Point { x, y };
            g.add_node(Node::from_name(u));
        }
    }
    for y in 0..map.bounds.y {
        for x in 0..map.bounds.x {
            let u = Point { x, y };
            let u_plant = map.plots[u.y][u.x];
            if x > 0 {
                let v = Point { x: x - 1, y };
                let v_plant = map.plots[v.y][v.x];
                if u_plant == v_plant {
                    g.add_edge_tuple(u, v).map_err(graphrs_anyhow)?;
                }
            }
            if y > 0 {
                let v = Point { x, y: y - 1 };
                let v_plant = map.plots[v.y][v.x];
                if u_plant == v_plant {
                    g.add_edge_tuple(u, v).map_err(graphrs_anyhow)?;
                }
            }
        }
    }
    Ok(g)
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let plots: Vec<Vec<char>> = lines.into_iter().map(|row| row.chars().collect()).collect();
    let bounds = Point {
        x: plots.first().ok_or_else(|| anyhow!("empty map?"))?.len(),
        y: plots.len(),
    };
    Ok(Map { plots, bounds })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let map = parse(lines)?;
    let g = map_to_graph(&map)?;

    let cc = connected_components(&g).map_err(graphrs_anyhow)?;
    let mut total = 0;
    for component in cc {
        // println!("component: {:?}", component);
        let area = component.len();
        let mut fences = component.len() * 4;
        for p in component.iter() {
            let edges = g.get_edges_for_node(*p).map_err(graphrs_anyhow)?;
            fences -= edges.len();
        }
        // println!(" fences: {}", fences);
        let cost = area * fences;
        total += cost;
    }

    Ok(total.to_string())
}

fn linear_flood(
    map: &Map,
    p: &Point,
    dir: Dir,
    fenced: &HashSet<Point>,
    seen: &mut HashSet<Point>,
) {
    let mut cursor = p.clone();
    while let Some(p2) = map.go(&cursor, dir) {
        // println!("  try: {:?} {:?}", dir, p2);
        if fenced.contains(&p2) {
            // println!("  extension: {:?} {:?}", dir, p2);
            seen.insert(p2);
            cursor = p2;
        } else {
            break;
        }
    }
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let map = parse(lines)?;
    let g = map_to_graph(&map)?;

    let cc = connected_components(&g).map_err(graphrs_anyhow)?;
    let mut total = 0;
    for component in cc {
        // println!("component: {:?}", component);
        let area = component.len();
        if area <= 2 {
            // single or double always a rectangle:
            total += area * 4;
            continue;
        }

        let mut sides = 0;

        for dir in enum_iterator::all::<Dir>() {
            // println!(" dir {:?}", dir);
            let (ccw, cw) = dir.orthos();
            let mut seen = HashSet::new(); // "seen for this fence direction"

            let fenced: HashSet<Pos<usize>> =
                HashSet::from_iter(component.iter().filter_map(|x| {
                    let beside = map.go(x, dir);
                    if beside.is_none() || !component.contains(&beside.unwrap()) {
                        Some(x.clone())
                    } else {
                        None
                    }
                }));

            for p in fenced.iter() {
                if !seen.insert(p.clone()) {
                    continue; // already excluded
                }

                // Has directional fence, so will form part of that side.
                sides += 1;

                // Flood fill in orthogonal directions to include them in the
                // side, i.e., exclude them from being detected as forming a new
                // side.
                // println!("  orthos: {:?} {:?}", ccw, cw);
                linear_flood(&map, p, ccw, &fenced, &mut seen);
                linear_flood(&map, p, cw, &fenced, &mut seen);
            }
        }

        total += sides * area;
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
    fn test_part1_ez() -> Result<()> {
        let lines = lines(indoc! {"
            RR
            RA
        "});
        assert_eq!(part1(lines)?, (3 * 8 + 1 * 4).to_string());
        Ok(())
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let lines = lines(indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "});
        assert_eq!(part1(lines)?, "140");
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let lines = lines(indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "});
        assert_eq!(part1(lines)?, "1930");
        Ok(())
    }

    #[test]
    fn test_part2_ez() -> Result<()> {
        let lines = lines(indoc! {"
            RR
            RA
        "});
        assert_eq!(part2(lines)?, (3 * 6 + 1 * 4).to_string());
        Ok(())
    }

    #[test]
    fn test_part2_a() -> Result<()> {
        let lines = lines(indoc! {"
            AAAA
            BBCD
            BBCC
            EEEC
        "});
        assert_eq!(part2(lines)?, "80");
        Ok(())
    }

    #[test]
    fn test_part2_b() -> Result<()> {
        let lines = lines(indoc! {"
            EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE
        "});
        assert_eq!(part2(lines)?, "236");
        Ok(())
    }

    #[test]
    fn test_part2_c() -> Result<()> {
        let lines = lines(indoc! {"
            AAAAAA
            AAABBA
            AAABBA
            ABBAAA
            ABBAAA
            AAAAAA
        "});
        assert_eq!(part2(lines)?, "368");
        Ok(())
    }

    #[test]
    fn test_part2_d() -> Result<()> {
        let lines = lines(indoc! {"
            RRRRIICCFF
            RRRRIICCCF
            VVRRRCCFFF
            VVRCCCJFFF
            VVVVCJJCFE
            VVIVCCJJEE
            VVIIICJJEE
            MIIIIIJJEE
            MIIISIJEEE
            MMMISSJEEE
        "});
        assert_eq!(part2(lines)?, "1206");
        Ok(())
    }
}
