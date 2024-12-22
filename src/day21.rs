use anyhow::{bail, Result};
use itertools::Itertools;
use memoize::memoize;

use crate::common::Pos;
type Point = Pos<isize>;

static KP_A: Point = Point { x: 2, y: 3 };
static KP_0: Point = Point { x: 1, y: 3 };
static KP_1: Point = Point { x: 0, y: 2 };
static KP_2: Point = Point { x: 1, y: 2 };
static KP_3: Point = Point { x: 2, y: 2 };
static KP_4: Point = Point { x: 0, y: 1 };
static KP_5: Point = Point { x: 1, y: 1 };
static KP_6: Point = Point { x: 2, y: 1 };
static KP_7: Point = Point { x: 0, y: 0 };
static KP_8: Point = Point { x: 1, y: 0 };
static KP_9: Point = Point { x: 2, y: 0 };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum DP {
    A,
    U,
    L,
    D,
    R,
}

fn gen_raw_steps(from: Point, to: Point) -> Vec<DP> {
    let mut raw_steps = vec![];
    let d = to - from;
    for _ in 0..(d.y.abs() as usize) {
        raw_steps.push(if d.y < 0 { DP::U } else { DP::D });
    }
    for _ in 0..(d.x.abs() as usize) {
        raw_steps.push(if d.x < 0 { DP::L } else { DP::R });
    }
    raw_steps
}

fn shortest_encoding<F>(alts: Vec<Vec<DP>>, encode: F) -> usize
where
    F: Fn(Vec<DP>) -> usize,
{
    alts.into_iter()
        .map(encode)
        .min()
        .expect("at least one alt")
}

#[memoize]
fn encode_dp(dps: Vec<DP>, depth: usize) -> usize {
    let mut total = 0;
    let mut current = DP::A;
    for d in dps {
        let mut alts = match (current, d) {
            // MATCH-AGEDDON to avoid the corner gap by hand-coding permutations
            (DP::A, DP::A) => vec![vec![]],
            (DP::A, DP::D) => vec![vec![DP::D, DP::L], vec![DP::L, DP::D]],
            (DP::A, DP::L) => vec![vec![DP::D, DP::L, DP::L], vec![DP::L, DP::D, DP::L]], // gap-avoid
            (DP::A, DP::R) => vec![vec![DP::D]],
            (DP::A, DP::U) => vec![vec![DP::L]],

            (DP::D, DP::A) => vec![vec![DP::U, DP::R], vec![DP::R, DP::U]],
            (DP::D, DP::D) => vec![vec![]],
            (DP::D, DP::L) => vec![vec![DP::L]],
            (DP::D, DP::R) => vec![vec![DP::R]],
            (DP::D, DP::U) => vec![vec![DP::U]],

            (DP::L, DP::A) => vec![vec![DP::R, DP::U, DP::R], vec![DP::R, DP::R, DP::U]], // gap-avoid
            (DP::L, DP::D) => vec![vec![DP::R]],
            (DP::L, DP::L) => vec![vec![]],
            (DP::L, DP::R) => vec![vec![DP::R, DP::R]],
            (DP::L, DP::U) => vec![vec![DP::R, DP::U]], // gap-avoid

            (DP::R, DP::A) => vec![vec![DP::U]],
            (DP::R, DP::D) => vec![vec![DP::L]],
            (DP::R, DP::L) => vec![vec![DP::L, DP::L]],
            (DP::R, DP::R) => vec![vec![]],
            (DP::R, DP::U) => vec![vec![DP::L, DP::U], vec![DP::U, DP::L]],

            (DP::U, DP::A) => vec![vec![DP::R]],
            (DP::U, DP::D) => vec![vec![DP::D]],
            (DP::U, DP::L) => vec![vec![DP::D, DP::L]], // gap-avoid
            (DP::U, DP::R) => vec![vec![DP::D, DP::R], vec![DP::R, DP::D]],
            (DP::U, DP::U) => vec![vec![]],
        };
        alts.iter_mut().for_each(|alt| alt.push(DP::A));
        if depth == 1 {
            total += shortest_encoding(alts, |alt| alt.len());
        } else {
            total += shortest_encoding(alts, |dps| encode_dp(dps, depth - 1));
        }
        current = d;
    }

    total
}

fn route_keypad(kp_keys: &str, max_depth: usize) -> Result<usize> {
    let mut prev = KP_A;
    let mut total = 0;
    for key in kp_keys.chars() {
        let to = match key {
            '7' => KP_7,
            '8' => KP_8,
            '9' => KP_9,
            '4' => KP_4,
            '5' => KP_5,
            '6' => KP_6,
            '1' => KP_1,
            '2' => KP_2,
            '3' => KP_3,
            '0' => KP_0,
            'A' => KP_A,
            _ => bail!("invalid keypad key"),
        };

        let raw_steps = gen_raw_steps(prev, to);
        let raw_len = raw_steps.len();
        let alts: Vec<Vec<DP>> = raw_steps
            .into_iter()
            .permutations(raw_len)
            .unique()
            .filter(|alt| {
                let n = alt.len();
                match (prev.x, prev.y, to.x, to.y) {
                    (0, _, 1, 3) => alt[n - 1] != DP::R, // 1st col to 0: last isn't Right
                    (0, _, 2, 3) => !(alt[n - 1] == DP::R && alt[n - 2] == DP::R), // 1st col to A: can't end in R->R
                    (1 | 2, 3, 0, _) => alt[n - 1] != DP::U, // 0 or A to 1st col: last isn't Up
                    _ => true,
                }
            })
            .map(|mut alt| {
                alt.push(DP::A);
                alt
            })
            .collect();
        total += shortest_encoding(alts, |dps| encode_dp(dps, max_depth));
        prev = to;
    }

    Ok(total)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        let numeric = line.chars().take(3).join("").parse::<usize>()?;
        let presses = route_keypad(&line, 2)?;
        total += presses * numeric;
    }
    Ok(total.to_string())
}

pub fn part2(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        let numeric = line.chars().take(3).join("").parse::<usize>()?;
        let presses = route_keypad(&line, 25)?;
        total += presses * numeric;
    }
    Ok(total.to_string())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use indoc::indoc;

    use super::*;

    fn to_lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_part1_a() -> Result<()> {
        let lines = to_lines(indoc! {"
            179A
        "});
        assert_eq!(part1(lines)?, (68 * 179).to_string());
        Ok(())
    }

    #[test]
    fn test_part1_b() -> Result<()> {
        let lines = to_lines(indoc! {"
            379A
        "});
        assert_eq!(part1(lines)?, (64 * 379).to_string());
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let lines = to_lines(indoc! {"
            029A
            980A
            179A
            456A
            379A
        "});
        assert_eq!(part1(lines)?, "126384");
        Ok(())
    }
}
