use anyhow::{bail, Result};
use itertools::Itertools;

use crate::common::{Dir, Pos};
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

static DP_A: Point = Point { x: 2, y: 0 };
static DP_U: Point = Point { x: 1, y: 0 };
static DP_L: Point = Point { x: 0, y: 1 };
static DP_D: Point = Point { x: 1, y: 1 };
static DP_R: Point = Point { x: 2, y: 1 };

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DP {
    A,
    U,
    L,
    D,
    R,
}
impl DP {
    fn to_char(&self) -> char {
        match self {
            DP::A => 'A',
            DP::U => '^',
            DP::L => '<',
            DP::D => 'v',
            DP::R => '>',
        }
    }
    fn to_point(&self) -> Point {
        match self {
            DP::A => DP_A,
            DP::U => DP_U,
            DP::L => DP_L,
            DP::D => DP_D,
            DP::R => DP_R,
        }
    }
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

fn shortest_encoding<F>(alts: Vec<Vec<DP>>, encode: F) -> Result<Vec<DP>>
where
    F: Fn(Vec<DP>) -> Result<Vec<DP>>,
{
    let mut iter = alts.into_iter();
    let mut shortest: Vec<DP> = encode(iter.next().unwrap())?;
    for alt in iter {
        let encoding = encode(alt)?;
        if encoding.len() < shortest.len() {
            shortest = encoding;
        }
    }
    Ok(shortest)
}

fn encode_dp(dps: Vec<DP>, depth: usize) -> Result<Vec<DP>> {
    if depth == 0 {
        return Ok(dps);
    }
    println!(" encoding d{} {:?}", depth, dps);
    let mut next_dps = vec![];
    let mut current = DP::A;
    for d in dps {
        let mut alts = match (current, d) {
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
        let mut shortest = shortest_encoding(alts, |dps| encode_dp(dps, depth - 1))?;
        next_dps.append(&mut shortest);
        current = d;
    }

    Ok(next_dps)
}

fn route_keypad(kp_keys: &str) -> Result<String> {
    let mut prev = KP_A;
    let mut seq = String::new();
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
                match (prev.x, prev.y, to.x, to.y) {
                    (0, _, 1, 3) => alt[alt.len() - 1] != DP::R, // 1st col to 0: last isn't Right
                    (0, 0, 2, 3) => *alt != vec![DP::D, DP::D, DP::D, DP::R, DP::R], // 7 to A
                    (0, 1, 2, 3) => *alt != vec![DP::D, DP::D, DP::R, DP::R], // 4 to A
                    (0, 2, 2, 3) => *alt != vec![DP::D, DP::R, DP::R], // 1 to A
                    (1 | 2, 3, 0, _) => alt[alt.len() - 1] != DP::U, // 0 or A to 1st col: last isn't Up
                    _ => true,
                }
            })
            .map(|mut alt| {
                alt.push(DP::A);
                alt
            })
            .collect();
        println!("{} all: {:?}", key, alts);
        let shortest = shortest_encoding(alts, |dps| encode_dp(dps, 2))?;
        println!("{} shortest: {:?}", key, shortest);
        for d in shortest {
            seq.push(d.to_char());
        }
        prev = to;
    }

    Ok(seq)
}

pub fn part1(lines: Vec<String>) -> Result<String> {
    let mut total: usize = 0;
    for line in lines {
        let numeric = line.chars().take(3).join("").parse::<usize>()?;
        let presses = route_keypad(&line)?;
        println!("{}: {}", line, presses);
        println!("{}: {} x {}", line, presses.len(), numeric);
        total += presses.len() * numeric;
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

    fn to_lines(text: &str) -> Vec<String> {
        text.lines().map(|x| x.to_string()).collect()
    }

    // #[test]
    // fn test_part1_partial_a() -> Result<()> {
    //     let presses = route_keypad("029A")?;
    //     assert_eq!(presses_to_str(&presses), "<A^A^^>AvvvA");
    //     let presses = route_dpad(presses)?;
    //     assert_eq!(presses_to_str(&presses), "v<<A>>^A<A>A<AAv>A^Av<AAA>^A");
    //     let presses = route_dpad(presses)?;
    //     assert_eq!(
    //         presses_to_str(&presses),
    //         //  "v   <  < A    >  > ^   A  <    A    >  A  <    A    A v   >  A  ^  A  v   <  A    A A >  ^   A "
    //         //  "v<A <A A >>^A vA A ^<A >A v<<A >>^A vA ^A v<<A >>^A A v<A >A ^A <A >A v<A <A >>^A A A vA ^<A >A"
    //         "v<A<AA>>^AvAA^<A>Av<<A>>^AvA^Av<<A>>^AAv<A>A^A<A>Av<A<A>>^AAAvA^<A>A"
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn test_part1_partial_b() -> Result<()> {
    //     let presses = route_keypad("379A")?;
    //     assert_eq!(presses_to_str(&presses), "^A^^<<A>>AvvvA");
    //     let presses = route_dpad(presses)?;
    //     assert_eq!(presses_to_str(&presses), "<A>A<AAv<AA>>^AvAA^Av<AAA>^A");
    //     let presses = route_dpad(presses)?;
    //     assert_eq!(presses.len(), 64);
    //     assert_eq!(
    //         presses_to_str(&presses),
    //         "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
    //     );
    //     Ok(())
    // }

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
    //OK     ^         A     ^    ^ <   < A    >  > A  v   v v A
    //OK     <    A    >  A  <  A A v<A A >>^A vA A ^A v<A A A >^A
    //mine:  v<<A >>^A vA ^A v<<A * >>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
    //their: <v<A >>^A vA ^A <vA <A A>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

    //OK     <    A    >  A  <    A    A v   <  A    A >  > ^   A  v   A   A ^  A  v   <  A    A A > ^    A
    //manu:  v<<A >>^A vA ^A v<<A >>^A A v<A <A >>^A A vA A ^<A >A v<A >^A A <A >A v<A <A >>^A A A vA ^<A >A

    //manu:   v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
    //theirs: <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A
    // TODO: decode "theirs" above and see if it outputs 379A. If it does, there's a bug in my algorithm
}
