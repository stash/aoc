use crate::common::Point;
use anyhow::{anyhow, bail, Result};
use graphrs::{algorithms::components::connected_components, Graph, GraphSpecs, Node};

struct Map {
    plots: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let plots: Vec<Vec<char>> = lines.into_iter().map(|row| row.chars().collect()).collect();
    let width = plots.first().ok_or_else(|| anyhow!("empty map?"))?.len();
    let height = plots.len();
    Ok(Map {
        plots,
        width,
        height,
    })
}

fn graphrs_anyhow(err: graphrs::Error) -> anyhow::Error {
    anyhow!("graphrs: {}", err)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let map = parse(lines)?;

    let mut g: Graph<Point, ()> = Graph::new(GraphSpecs::undirected());
    for y in 0..map.height {
        for x in 0..map.width {
            let u = Point::new(x, y);
            g.add_node(Node::from_name(u));
        }
    }

    for y in 0..map.height {
        for x in 0..map.width {
            let u = Point::new(x, y);
            let u_plant = map.plots[u.y][u.x];
            if x > 0 {
                let v = Point::new(x - 1, y);
                let v_plant = map.plots[v.y][v.x];
                if u_plant == v_plant {
                    g.add_edge_tuple(u, v).map_err(graphrs_anyhow)?;
                }
            }
            if y > 0 {
                let v = Point::new(x, y - 1);
                let v_plant = map.plots[v.y][v.x];
                if u_plant == v_plant {
                    g.add_edge_tuple(u, v).map_err(graphrs_anyhow)?;
                }
            }
        }
    }

    let cc = connected_components(&g).map_err(graphrs_anyhow)?;
    let mut total = 0;
    // println!("components: {}", cc.len());
    for component in cc.iter() {
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
}
