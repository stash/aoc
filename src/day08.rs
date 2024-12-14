use crate::common::Pos;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

struct Map {
    nodes: HashMap<char, Vec<Pos>>,
    bound: Pos,
}
impl Map {
    pub fn in_bounds(&self, p: &Pos) -> bool {
        p.x >= 0 && p.x < self.bound.x && p.y >= 0 && p.y < self.bound.y
    }
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let mut map = Map {
        nodes: HashMap::new(),
        bound: Pos {
            x: lines[0].len().try_into()?,
            y: lines.len().try_into()?,
        },
    };
    for (y, line) in lines.into_iter().enumerate() {
        for (x, f) in line.chars().enumerate() {
            if f == '.' {
                continue;
            }
            let p = Pos::new(x, y)?;
            map.nodes.entry(f).or_insert(Vec::new()).push(p);
        }
    }
    Ok(map)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    let map = parse(lines)?;
    for (_f, poses) in &map.nodes {
        for i in 0..poses.len() - 1 {
            // for some initial position
            let pi = poses[i];
            for j in i + 1..poses.len() {
                // calculate antinodes with positions after it in the list
                let pj = poses[j];
                let d = pi - pj;
                let a1 = pi + d;
                let a2 = pj - d;
                // println!(
                //     "f {}: pi {:?}, pj {:?}, d {:?}, a1 {:?}, a2 {:?}",
                //     _f, pi, pj, d, a1, a2
                // );
                if map.in_bounds(&a1) {
                    antinodes.insert(a1);
                }
                if map.in_bounds(&a2) {
                    antinodes.insert(a2);
                }
            }
        }
    }
    Ok(antinodes.into_iter().count().to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut antinodes: HashSet<Pos> = HashSet::new();
    let map = parse(lines)?;
    for (_f, poses) in &map.nodes {
        if poses.len() < 2 {
            // no resonance possible
            continue;
        }
        for i in 0..(poses.len() - 1) {
            // for some initial position
            let pi = poses[i];
            antinodes.insert(pi);
            for j in (i + 1)..poses.len() {
                // calculate antinodes with positions after it in the list
                let pj = poses[j];
                antinodes.insert(pj);

                let d = pi - pj;
                // resonate along the delta
                let mut a1 = pi + d;
                while map.in_bounds(&a1) {
                    antinodes.insert(a1);
                    // println!("f {}: {:?}", _f, a1);
                    a1 = a1 + d;
                }
                let mut a2 = pj - d;
                while map.in_bounds(&a2) {
                    antinodes.insert(a2);
                    // println!("f {}: {:?}", _f, a2);
                    a2 = a2 - d;
                }
            }
        }
    }
    Ok(antinodes.into_iter().count().to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;
    fn input() -> Vec<String> {
        indoc! {"
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
        "}
        .lines()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(input())?, "14");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(input())?, "34");
        Ok(())
    }
}
