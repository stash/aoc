use std::fmt::{Display, Formatter, Write};

use anyhow::{anyhow, bail, Result};
use itertools::*;
use regex::Regex;

struct Chal {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn parse(lines: Vec<String>) -> Result<Chal> {
    let mut iter = lines.into_iter();
    let patterns = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_owned())
        .collect();
    iter.next();
    let designs = iter.collect();
    Ok(Chal { patterns, designs })
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let c = parse(lines)?;
    let re = {
        let all_pats = c.patterns.join("|");
        let giga_re_str = format!("^({})+$", all_pats);
        Regex::new(&giga_re_str)?
    };
    let mut total = 0;
    for d in c.designs {
        if re.is_match(&d) {
            total += 1;
        }
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
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
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            r, wr, b, g, bwu, rb, gb, br

            brwrr
            bggr
            gbbr
            rrbgbr
            ubwu
            bwurrg
            brgr
            bbrgwb
        "});
        assert_eq!(part1(lines)?, "6");
        Ok(())
    }
}
