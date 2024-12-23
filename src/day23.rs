use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::Result;
use graphrs::{Edge, Graph, GraphSpecs};
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
                // println!("found {:?}", trip_name);
                triples.insert(trip_name.join(","));
            }
        }
    }
    Ok(triples.len().to_string())
}

struct Chal2<'a> {
    adj: HashMap<&'a str, HashSet<&'a str>>,
}
impl<'a> Chal2<'a> {
    fn parse(lines: &'a Vec<String>) -> Result<Self> {
        let mut adj = HashMap::new();
        for line in lines.iter() {
            let (a, b) = line.split_once('-').unwrap();
            adj.entry(a).or_insert_with(|| HashSet::new()).insert(b);
            adj.entry(b).or_insert_with(|| HashSet::new()).insert(a);
        }
        Ok(Self { adj })
    }
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let c = Chal2::parse(&lines)?;
    let mut max_clique = "".to_owned();
    for (root, root_adj) in c.adj.iter() {
        // Clone a working-set of adjacent nodes
        let mut work_adj = root_adj.clone();
        // Go until all neighbours visited
        while work_adj.len() > 0 {
            // Clique has just the root initially
            let mut non_clique: HashSet<&str> = HashSet::new();
            let mut clique: BTreeSet<&str> = BTreeSet::new();
            clique.insert(root);

            // Visit each neighb. and see if it's in the clique. If it isn't,
            // still might be in another clique with this root.
            for node in work_adj.iter() {
                let node_adj = c.adj.get(node).unwrap();
                if clique.iter().all(|n| node_adj.contains(n)) {
                    clique.insert(node);
                } else {
                    non_clique.insert(node);
                }
            }

            // Name should already be sorted thanks to BTree
            let c_name = clique.into_iter().join(",");
            if c_name.len() > max_clique.len() {
                max_clique = c_name;
            }

            work_adj = non_clique;

            // TODO: there's probably some kind of optimization to remove found
            // cliques from future iterations, but it's fast enough for AoC
            // as-is.
        }
    }

    Ok(max_clique)
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use indoc::indoc;

    fn input() -> Vec<String> {
        let text = indoc! {"
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
        "};
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(input())?, "7");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(input())?, "co,de,ka,ta");
        Ok(())
    }
}
