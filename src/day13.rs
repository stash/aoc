use anyhow::{anyhow, Result};
use regex::Regex;
use std::{collections::HashSet, usize};

use crate::common::Pos;

struct Challenge {
    a: Pos,
    b: Pos,
    prize: Pos,
}

fn re_parse_pos(haystack: &str, re: &Regex) -> Result<Pos> {
    let cap = re
        .captures(haystack)
        .ok_or_else(|| anyhow!("re fail '{}' '{}'", re, haystack))?;
    let p = Pos::new((&cap[1]).parse::<usize>()?, (&cap[2]).parse::<usize>()?)?;
    Ok(p)
}

fn parse(lines: Vec<String>) -> Result<Vec<Challenge>> {
    let mut iter = lines.into_iter();
    let re_button = Regex::new(r"^Button .: X\+([0-9]+), Y\+([0-9]+)").unwrap();
    let re_prize = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)").unwrap();
    let mut challenges = Vec::new();
    loop {
        let a = re_parse_pos(&iter.next().unwrap(), &re_button)?;
        let b = re_parse_pos(&iter.next().unwrap(), &re_button)?;
        let prize = re_parse_pos(&iter.next().unwrap(), &re_prize)?;
        challenges.push(Challenge { a, b, prize });

        if iter.next().is_none() {
            break;
        }
    }

    Ok(challenges)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let chals = parse(lines)?;
    let mut solves: Vec<isize> = vec![isize::MAX; chals.len()];
    for (i, c) in chals.iter().enumerate() {
        for a_press in 0..=100 {
            for b_press in 0..=100 {
                let a = c.a * a_press;
                let b = c.b * b_press;
                let p = a + b;
                if p == c.prize {
                    let cost = 3 * a_press + b_press;
                    if cost < solves[i] {
                        solves[i] = cost;
                    }
                }
            }
        }
    }

    let total: isize = solves.into_iter().filter(|x| *x != isize::MAX).sum();
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let chals = parse(lines)?;

    let mut total = 0;

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
    fn test_part1() -> Result<()> {
        let lines = lines(indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "});
        assert_eq!(part1(lines)?, "480");
        Ok(())
    }
}
