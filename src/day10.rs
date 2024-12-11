use std::{collections::{HashMap, HashSet}, ops::Range};
use anyhow::{anyhow, bail, Result};
use crate::common::{Dir,Pos};
use enum_iterator;



#[derive(Debug)]
struct Map {
    layers: Vec<Vec<Pos>>,
}

fn parse(lines: Vec<String>) -> Result<Map> {
    let mut map = Map {
        layers: Vec::new()
    };
    for _ in 0..=10 {
        map.layers.push(Vec::new());
    }

    for (y,line) in lines.into_iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            let p = Pos::new(x,y)?;
            let z = match c {
                '.' => 10, // "unreachable"
                '0'..='9' => c.to_digit(10).expect("decimal digit"),
                c => bail!("invalid map character {}", c)
            } as usize;
            map.layers[z].push(p);
        }
    }
    Ok(map)
}

fn peak_closure(map: &mut Map) -> usize {
    let mut above: HashMap<Pos, HashSet<Pos>> = HashMap::new();
    for p in &map.layers[9] {
        let mut reach = HashSet::new();
        reach.insert(*p);
        above.insert(*p, reach);
    }

    for z in (0..=8).rev() {
        // println!(" above {}: {:?}", z, above);
        let mut current: HashMap<Pos, HashSet<Pos>> = HashMap::new();
        for p1 in &map.layers[z] {
            // println!("  consider {:?}", p1);
            let mut reach = HashSet::new();
            for dir in enum_iterator::all::<Dir>() {
                let p2 = p1.go(dir);
                // println!("   then {:?}", p2);
                if let Some(reach2) = above.get(&p2) {
                    // println!("    ok {:?} has {:?}", p2, reach2);
                    for r in reach2 {
                        reach.insert(*r);
                    }
                }
            }
            current.insert(*p1, reach);
        }
        // println!(" at {}: {:?}", z, current);
        above = current;
    }

    // println!("at end {:?}", above);
    let mut total = 0;
    for p in &map.layers[0] {
        if let Some(reach) = above.get(p) {
            let score = reach.iter().count();
            total += score;
        }
    }

    total
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut map = parse(lines)?;
    let total = peak_closure(&mut map);
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    bail!("not implemented")
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let text = indoc! {"
            0123
            1234
            8765
            9876
        "};
        assert_eq!(part1(lines(text))?, "1");
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let text = indoc! {"
            ...0...
            ...1...
            ...2...
            6543456
            7.....7
            8.....8
            9.....9
        "};
        assert_eq!(part1(lines(text))?, "2");
        Ok(())
    }

    #[test]
    fn test_part1_c() -> Result<()> {
        let text = indoc! {"
            ..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....
        "};
        assert_eq!(part1(lines(text))?, "4");
        Ok(())
    }


    #[test]
    fn test_part1_d() -> Result<()> {
        let text = indoc! {"
            10..9..
            2...8..
            3...7..
            4567654
            ...8..3
            ...9..2
            .....01
        "};
        assert_eq!(part1(lines(text))?, "3");
        Ok(())
    }

    #[test]
    fn test_part1_f() -> Result<()> {
        let text = indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        assert_eq!(part1(lines(text))?, "36");
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let text = indoc! {"
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "};
        assert_eq!(part2(lines(text))?, "");
        Ok(())
    }
}
