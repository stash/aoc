use std::collections::HashSet;

use anyhow::{anyhow, bail, Result};

use graphrs::{algorithms::components, Edge, Graph, GraphSpecs};
use itertools::Itertools;

use crate::common::*;

struct Chal<'a> {
    t_nodes: HashSet<&'a str>,
    g: Graph<&'a str, ()>,
}

impl<'a> Chal<'a> {
    fn parse(lines: &'a Vec<String>) -> Result<Self> {
        let mut g: Graph<&'a str, _> = Graph::new(GraphSpecs::undirected_create_missing());
        let mut t_nodes = HashSet::new();
        for line in lines {
            let (a, b) = line.split_once('-').unwrap();
            if a.starts_with('t') {
                t_nodes.insert(a);
            }
            if b.starts_with('t') {
                t_nodes.insert(b);
            }
            g.add_edge(Edge::new(a, b)).map_err(graphrs_anyhow)?;
        }

        Ok(Chal { t_nodes, g })
    }
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let c = Chal::parse(&lines)?;
    let mut triples: HashSet<String> = HashSet::new();
    for t_node in c.t_nodes {
        let conn = c.g.get_neighbor_nodes(t_node).map_err(graphrs_anyhow)?;
        for (u, v) in conn.iter().tuple_combinations() {
            if c.g.get_edge(u.name, v.name).is_ok() {
                let mut trip_name = vec![t_node, u.name, v.name];
                trip_name.sort();
                println!("found {:?}", trip_name);
                triples.insert(trip_name.join(","));
            }
        }
    }
    Ok(triples.len().to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("not done")
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

    fn to_lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        let lines = to_lines(indoc! {"
            kh-tc
            qp-kh
            de-cg
            ka-co
            yn-aq
            qp-ub
            cg-tb
            vc-aq
            tb-ka
            wh-tc
            yn-cg
            kh-ub
            ta-co
            de-co
            tc-td
            tb-wq
            wh-td
            ta-ka
            td-qp
            aq-cg
            wq-ub
            ub-vc
            de-ta
            wq-aq
            wq-vc
            wh-yn
            ka-de
            kh-ta
            co-tc
            wh-qp
            tb-vc
            td-yn
        "});
        assert_eq!(part1(lines)?, "7");
        Ok(())
    }
}
