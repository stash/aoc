use anyhow::{anyhow, Result};
use regex::Regex;

use crate::common::Pos;

struct Bot {
    p: Pos,
    v: Pos,
}
impl Bot {
    fn simulate(&mut self, bounds: Pos) {
        let mut p = self.p + self.v;
        if p.x < 0 {
            p.x += bounds.x;
        } else if p.x >= bounds.x {
            p.x -= bounds.x;
        }
        if p.y < 0 {
            p.y += bounds.y;
        } else if p.y >= bounds.y {
            p.y -= bounds.y;
        }
        self.p = p;
    }
}

fn parse(lines: Vec<String>) -> Result<Vec<Bot>> {
    let re = Regex::new(r"^p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    let mut bots = Vec::new();
    for line in lines {
        let cap = re
            .captures(&line)
            .ok_or_else(|| anyhow!("failed to parse line: {}", &line))?;
        let p = Pos {
            x: cap[1].parse()?,
            y: cap[2].parse()?,
        };
        let v = Pos {
            x: cap[3].parse()?,
            y: cap[4].parse()?,
        };
        bots.push(Bot { p, v });
    }
    Ok(bots)
}

pub fn part1(lines: Vec<String>, bounds: Pos) -> Result<String> {
    let mut bots = parse(lines)?;
    let cycles = 100;
    for _ in 0..cycles {
        for b in bots.iter_mut() {
            b.simulate(bounds);
        }
    }
    let x_part = bounds.x / 2;
    let y_part = bounds.y / 2;
    let mut quads: [isize; 4] = [0, 0, 0, 0];
    for b in bots {
        println!("{:?}", b.p);
        if b.p.x < x_part {
            if b.p.y < y_part {
                quads[0] += 1;
            } else if b.p.y > y_part {
                quads[1] += 1;
            }
        } else if b.p.x > x_part {
            if b.p.y < y_part {
                quads[2] += 1;
            } else if b.p.y > y_part {
                quads[3] += 1;
            }
        }
    }
    println!("{:?}", quads);
    let total: isize = quads.into_iter().fold(1, |acc, x| acc * x);
    Ok(total.to_string())
}

pub fn part2(_lines: Vec<String>) -> Result<String> {
    todo!()
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
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
        "});
        assert_eq!(part1(lines, Pos::new(11, 7)?)?, "12");
        Ok(())
    }
}
