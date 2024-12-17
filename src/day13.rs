use anyhow::{anyhow, Result};
use regex::Regex;

use crate::common::Pos;

#[derive(Debug)]
struct Challenge {
    a: Pos<isize>,
    b: Pos<isize>,
    prize: Pos<isize>,
}

fn re_parse_pos(haystack: &str, re: &Regex) -> Result<Pos<isize>> {
    let cap = re
        .captures(haystack)
        .ok_or_else(|| anyhow!("re fail '{}' '{}'", re, haystack))?;
    Ok(Pos {
        x: (&cap[1]).parse()?,
        y: (&cap[2]).parse()?,
    })
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

fn exact_solution(chal: &Challenge) -> Option<(isize, isize)> {
    // Button A & B are the basis vectors for a lattice. We want to know if the
    // prize is "in" the lattice. In other words: are there integer coefficients
    // ("button presses") of the also-integer basis vectors?

    // Represent basis vectors as matrix M:
    //  a b
    //  c d
    let m_a = chal.a.x;
    let m_b = chal.b.x;
    let m_c = chal.a.y;
    let m_d = chal.b.y;

    // Matrix inversion, but with integers: calculate (1/inv_det) * Adjoint
    // First: determinant
    let det = m_a * m_d - m_b * m_c;
    // Second: adjoint matrix
    let adj_a = m_d;
    let adj_b = -m_b;
    let adj_c = -m_c;
    let adj_d = m_a;

    // Matrix multiply by Adj, post-applying the determinant.
    // This is effectively multiplication by the inverse of M.
    // (These are the # of button presses)
    let a = (chal.prize.x * adj_a + chal.prize.y * adj_b) / det;
    let b = (chal.prize.x * adj_c + chal.prize.y * adj_d) / det;

    // Check it's an actual integer solution: scale the basis vectors and check if it
    // matches the prize coordinates.
    let prize = chal.a * a + chal.b * b;
    if prize == chal.prize {
        Some((a, b))
    } else {
        None
    }
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let offset = Pos {
        x: 10000000000000,
        y: 10000000000000,
    };
    let chals = parse(lines)?.into_iter().map(|c| Challenge {
        a: c.a,
        b: c.b,
        prize: c.prize + offset,
    });
    let mut solves = Vec::new();

    for c in chals {
        if let Some((a, b)) = exact_solution(&c) {
            // There's not actually multiple solutions with different costs, as
            // the story text suggests. Dirty, dirty lie!
            solves.push(3 * a + b);
        }
    }

    let total: isize = solves.into_iter().sum();
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

    #[test]
    fn test_part2_yes() -> Result<()> {
        let lines = lines(indoc! {"
            Button A: X+26, Y+66
            Button B: X+67, Y+21
            Prize: X=12748, Y=12176

            Button A: X+69, Y+23
            Button B: X+27, Y+71
            Prize: X=18641, Y=10279
        "});
        assert_eq!(part2(lines)?, "875318608908");
        Ok(())
    }

    #[test]
    fn test_part2_no() -> Result<()> {
        let lines = lines(indoc! {"
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400

            Button A: X+17, Y+86
            Button B: X+84, Y+37
            Prize: X=7870, Y=6450
        "});
        assert_eq!(part2(lines)?, "0");
        Ok(())
    }
}
